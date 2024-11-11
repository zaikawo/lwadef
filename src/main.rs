use serde_json::json;

enum VariableScope {
    Line,
    Local,
    Global,
    Save,
}

type DFNumber = f64;

enum Primitive {
    StringValue(String),
    NumberValue(DFNumber),
    ComponentValue(String),
    LocationValue(DFNumber, DFNumber, DFNumber, DFNumber, DFNumber),
    VectorValue(DFNumber, DFNumber, DFNumber),
    Variable(String, VariableScope),
}

impl Primitive {
    fn id(&self) -> &'static str {
        match self {
            Self::StringValue(..) => "str",
            Self::NumberValue(..) => "num",
            Self::ComponentValue(..) => "comp",
            Self::LocationValue(..) => "loc",
            Self::VectorValue(..) => "vec",
            Self::Variable(..) => "var",
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::StringValue(s) => {
                json!({
                    "id": "txt",
                    "data": {
                        "name": s
                    }
                })
            }
            Self::NumberValue(n) => {
                json!({
                    "id": "num",
                    "data": {
                        "name": n.to_string()
                    }
                })
            }
            _ => {
                json!({
                    "im too": "lazy"
                })
            }
        }
    }
}

struct Chest {
    contents: Vec<Primitive>,
}

impl Chest {
    fn to_json(&self) -> serde_json::Value {
        let mut args: Vec<serde_json::Value> = vec![];

        for i in &self.contents {
            args.push(json!({
                "item": i.to_json(),
                "slot": args.len()
            }));
        }

        json!({
            "items": args
        })
    }
}

enum BlockType {
    PlayerAction,
    SetVar,
    CallFunction,
}

impl BlockType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::PlayerAction => "player_action",
            Self::SetVar => "set_var",
            Self::CallFunction => "call_func",
        }
    }
}

struct Block {
    block: BlockType,
    data: String,
    args: Chest,
}

impl Block {
    pub fn to_json(&self) -> serde_json::Value {
        match self.block {
            BlockType::CallFunction => {
                json!({
                    "id": "block",
                    "block": self.block.name(),
                    "args": self.args.to_json(),
                    "data": self.data
                })
            }
            _ => {
                json!({
                    "id": "block",
                    "block": self.block.name(),
                    "args": self.args.to_json(),
                    "action": self.data
                })
            }
        }
    }
}

enum Line {
    Event {
        name: String,
        line: Vec<Block>,
    },
    Function {
        name: String,
        args: Chest,
        line: Vec<Block>,
    },
}

impl Line {
    fn line(&self) -> &Vec<Block> {
        match self {
            Self::Event { name, line } => line,
            Self::Function { name, args, line } => line,
        }
    }

    fn contents_to_json(&self) -> Vec<serde_json::Value> {
        let mut ln: Vec<serde_json::Value> = vec![];

        for i in self.line() {
            ln.push(i.to_json());
        }

        ln
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut ln: Vec<serde_json::Value> = vec![];

        ln.push(match self {
            Self::Event { name, line } => {
                json!({
                    "id": "block",
                    "block": "event",
                    "args": {
                        "items": []
                    },
                    "action": name
                })
            }
            Self::Function { name, args, line } => {
                json!({
                    "id": "block",
                    "block": "func",
                    "args": args.to_json(),
                    "data": name
                })
            }
        });

        let mut g = self.contents_to_json();

        ln.append(&mut g);

        json!(ln)
    }
}

struct Program {
    lines: Vec<Line>,
}

impl Program {
    pub fn compile(&self) -> serde_json::Value {
        let mut ret: Vec<serde_json::Value> = vec![];

        for i in &self.lines {
            ret.push(i.to_json())
        }

        json!({
            "blocks": ret
        })
    }
}

fn main() {
    println!("this compiled");

    let m = Line::Event {
        name: "Join".to_string(),
        line: vec![Block {
            block: BlockType::PlayerAction,
            data: "SendMessage".to_string(),
            args: Chest {
                contents: vec![Primitive::NumberValue(64.3)],
            },
        }],
    };

    let p = Program { lines: vec![m] };

    println!("{}", p.compile().to_string())
}
