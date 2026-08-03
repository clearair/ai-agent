use std::{collections::HashMap, sync::Arc};

use crate::tools::{
    calculator::r#impl::CalculatorTool,
    mcp::{client::McpClient, load_config, tool::McpTool},
    tool::Tool,
    web_search::r#impl::WebSearchTool,
};

pub mod calculator;
pub mod mcp;
pub mod tool;
pub mod web_search;

pub type ToolBox = HashMap<String, Box<dyn Tool>>;

// pub fn tools() -> Vec<ChatCompletionTools> {

// }

pub async fn build_toolbox() -> anyhow::Result<ToolBox> {
    let mut tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    let config = load_config("mcp_servers.json")?;
    for server_config in config.servers {
        let mcp_client = Arc::new(McpClient::connect(server_config).await?);
        for tool in mcp_client.list_tools().await? {
            tools.push(Box::new(McpTool::new(mcp_client.clone(), tool)));
        }
    }

    Ok(tools
        .into_iter()
        .map(|t| (t.name().to_string(), t))
        .collect())
}
