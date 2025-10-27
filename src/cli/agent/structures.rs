use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDocumentation {
    pub schema_version: String,
    pub tool: ToolInfo,
    pub capabilities: Capabilities,
    pub constraints: Constraints,
    pub best_practices: Vec<String>,
    pub common_errors: Vec<CommonError>,
    pub examples: HashMap<String, Example>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub repository: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub commands: Vec<Command>,
    pub modules: Vec<Module>,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub api: HashMap<String, FunctionDoc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<HashMap<String, String>>,
    pub usage_example: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionDoc {
    pub signature: String,
    pub description: String,
    pub example: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, String>>,
    pub returns: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraints {
    pub security: SecurityConstraints,
    pub limitations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityConstraints {
    pub sandboxing: bool,
    pub restricted_operations: Vec<String>,
    pub memory_limits: String,
    pub instruction_limits: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonError {
    pub pattern: String,
    pub cause: String,
    pub solution: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    pub description: String,
    pub code: String,
    pub command: String,
}
