use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Task {
    pub task_id: String,

    pub name: String,

    pub description: String,

    pub condition: serde_json::Value,

    pub function: FunctionType,

    pub input: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FunctionType {
    Validate,
    Enrich,
    Publish,
}
