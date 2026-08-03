use std::{collections::HashMap, fs};

use serde::Deserialize;

pub mod client;
pub mod tool;

#[derive(Debug, Deserialize)]
pub struct McpConfig {
    pub servers: Vec<McpServerConfig>,
}

#[derive(Debug, Deserialize)]

pub struct McpServerConfig {
    pub title: String,

    #[serde(flatten)]
    pub transport: McpTransport,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpTransport {
    Stdio {
        command: String,

        #[serde(default)]
        args: Vec<String>,

        #[serde(default)]
        envs: HashMap<String, String>,
    },

    Http {
        url: String,

        #[serde(default)]
        headers: HashMap<String, String>,
    },

    Sse {
        url: String,

        #[serde(default)]
        headers: HashMap<String, String>,
    },
}

pub fn load_config(path: &str) -> anyhow::Result<McpConfig> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}
