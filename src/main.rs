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
    args: Vec<Primitive>,
}

impl Block {
    pub fn to_json(&self) -> serde_json::Value {
        match self.block {
            BlockType::CallFunction => {
                json!({
                    "id": "block",
                    "block": self.block.name(),
                    "args": "hi",
                    "data": self.data
                })
            }
            _ => {
                json!({
                    "id": "block",
                    "block": self.block.name(),
                    "args": "hi",
                    "action": self.data
                })
            }
        }
    }
}

fn main() {
    println!("this compiled")
}
