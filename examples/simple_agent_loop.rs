use ai_agent::{llm::compiete::chat_complete, tools::build_toolbox};
use chrono::Local;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let toolbox = build_toolbox().await?;
    let now = Local::now();

    let current_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let system_prompt = format!(
        r#"你是一位专业、可靠、乐于帮助用户的 AI 助手。

当前本地时间：{}

请始终将"今天"、"昨天"、"明天"、"本周"、"本月"、"上个月"等相对时间，
解释为相对于上面的当前时间。

你可以使用多个工具来帮助完成任务。

工具使用原则：

1. 如果问题可以直接回答，则直接回答，不要调用工具。

2. 如果用户的问题需要最新的信息，例如：
   - 新闻
   - 天气
   - 汇率
   - 股票
   - 网络搜索
   等，请使用 Web Search 工具。

3. 如果需要进行数学计算、金额计算、百分比计算、
   或者任何要求结果精确的计算，请使用 Calculator 工具。

4. 当用户需要查询、统计、新增、修改、删除费用记录时，
   请使用 Expense MCP 提供的工具，例如：
   - create_expense
   - list_expenses
   - get_summary
   等。

5. 不要猜测工具可以提供的数据。

6. 如果工具能够得到答案，就应该调用工具，
   不要回答"我不知道"。

7. 工具返回结果以后，请直接根据工具结果生成自然、简洁、准确的回答，
   不要把工具调用过程告诉用户。

请始终优先完成用户的任务，而不是刻意调用工具。"#,
        current_time
    );

    // 测试 1：需要实时信息（触发 web_search）
    println!("\n=== 测试 1：实时信息查询 ===");
    let result = chat_complete(
        "Kimi-K3",
        Some(&system_prompt),
        "今天股票为什么下跌",
        &toolbox,
    )
    .await?;
    println!("回答: {result}");

    // 测试 2：需要计算（触发 calculator）
    println!("\n=== 测试 2：计算任务 ===");
    let result = chat_complete(
        "Kimi-K3",
        Some(&system_prompt),
        "123456789 乘以 987654321 等于多少？",
        &toolbox,
    )
    .await?;
    println!("回答: {result}");

    Ok(())
}
