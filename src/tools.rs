use std::collections::HashMap;

use async_openai::types::chat::ChatCompletionTools;

use crate::tools::{
    calculator::{definition::calculator_tool_definition, r#impl::CalculatorTool},
    tool::Tool,
    web_search::{definition::web_search_tool_definition, r#impl::WebSearchTool},
};

pub mod calculator;
pub mod tool;
pub mod web_search;

pub type ToolBox = HashMap<String, Box<dyn Tool>>;

// pub fn tools() -> Vec<ChatCompletionTools> {

// }

pub fn build_toolbox() -> ToolBox {
    let tools: Vec<Box<dyn Tool>> = vec![Box::new(CalculatorTool), Box::new(WebSearchTool)];

    tools
        .into_iter()
        .map(|t| (t.name().to_string(), t))
        .collect()
}
