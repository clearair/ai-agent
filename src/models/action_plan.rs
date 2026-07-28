use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActionPlan {
    pub goal: String,
    pub steps: Vec<ActionStep>,
    pub difficulty: Difficulty,
    pub estimated_minutes: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ActionStep {
    pub index: u8,
    pub description: String,
    pub tool_hint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
