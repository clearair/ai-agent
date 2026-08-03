use anyhow::Result;
use rmcp::model::{CallToolRequestParams, Tool};
use rmcp::service::{RoleClient, RunningService};
use rmcp::transport::StreamableHttpClientTransport;

use rmcp::{ServiceExt, transport::TokioChildProcess};
use tokio::process::Command;

use crate::tools::mcp::McpServerConfig;

pub struct McpClient {
    service: RunningService<RoleClient, ()>,
}

impl McpClient {
    pub async fn connect(msc: McpServerConfig) -> Result<Self> {
        match msc.transport {
            super::McpTransport::Stdio {
                command,
                args,
                envs,
            } => {
                let mut command = Command::new(command);

                command.args(args).envs(envs);
                let service = ().serve(TokioChildProcess::new(command)?).await?;
                Ok(Self { service })
            }
            super::McpTransport::Http { url, .. } | super::McpTransport::Sse { url, .. } => {
                let transport = StreamableHttpClientTransport::from_uri(url);
                let service = ().serve(transport).await?;

                Ok(Self { service })
            }
        }
        // car
    }

    /// 拿到 server 暴露的所有工具
    pub async fn list_tools(&self) -> Result<Vec<Tool>> {
        let result = self.service.list_tools(Default::default()).await?;
        Ok(result.tools)
    }

    /// 按名字调用某个工具，arguments 是一个 JSON 对象
    pub async fn call_tool(&self, name: &str, arguments: serde_json::Value) -> Result<String> {
        let params = CallToolRequestParams::new(name.to_string())
            .with_arguments(arguments.as_object().cloned().unwrap_or_default());

        let result = self.service.call_tool(params).await?;

        let text = result
            .content
            .iter()
            .filter_map(|block| block.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(text)
    }
}
