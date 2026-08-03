use std::sync::Arc;

use serde_json::Value;

use crate::tools::{mcp::client::McpClient, tool::Tool};

pub struct McpTool {
    client: Arc<McpClient>,
    name: String,
    description: String,
    parameters: Value,
}

impl McpTool {
    pub fn new(client: Arc<McpClient>, tool: rmcp::model::Tool) -> Self {
        let parameters = Value::Object((*tool.input_schema).clone());

        Self {
            client,
            name: tool.name.to_string(),
            description: tool.description.map(|d| d.to_string()).unwrap_or_default(),
            parameters,
        }
    }
}

#[async_trait::async_trait]
impl Tool for McpTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn parameters(&self) -> Value {
        self.parameters.clone()
    }

    async fn execute(&self, args_json: &str) -> anyhow::Result<String> {
        let args: Value = serde_json::from_str(args_json)?;
        self.client.call_tool(&self.name, args).await
    }
}
