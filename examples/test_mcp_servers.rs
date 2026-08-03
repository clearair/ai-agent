use ai_agent::{llm::compiete::chat_complete, tools::build_toolbox};
use chrono::Local;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // build_toolbox 会读取 mcp_servers.json，连接其中的每个 MCP Server，
    // 调用 list_tools，并把远程工具包装成 Agent 可以使用的 Tool。
    let toolbox = build_toolbox().await?;
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");

    let system_prompt = format!(
        r#"你是一个可以使用 MCP 工具的 AI Agent。

当前时间：{current_time}

工具使用规则：
1. 当任务明确要求使用某个工具时，必须调用工具，不要只给出猜测。
2. 工具返回结果后，根据结果给出简洁的最终回答。
3. 不要向用户暴露内部工具调用过程。"#
    );

    println!("\n=== Test Expense MCP ===");
    let expense_result = chat_complete(
        "Kimi-K3",
        Some(&system_prompt),
        "请使用 Expense MCP 的 list_expenses 工具查询所有费用记录，然后告诉我查询结果。",
        &toolbox,
    )
    .await?;
    println!("Agent: {expense_result}");

    println!("\n=== Test Streamable HTTP MCP ===");
    let http_result = chat_complete(
        "Kimi-K3",
        Some(&system_prompt),
        "请使用 HTTP Demo MCP 提供的 lorem 工具生成 8 个英文单词，然后把结果返回给我。",
        &toolbox,
    )
    .await?;
    println!("Agent: {http_result}");

    Ok(())
}
