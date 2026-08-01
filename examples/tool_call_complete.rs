use ai_agent::{llm::compiete::chat_complete, tools::tools};
use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    let tools = tools();

    tracing::subscriber::set_global_default(subscriber)?;

//     let content = chat_complete(
//         "grok-3",
//         Some("你是一个全能的助手, 简单回答问题"),
//         "爱尔兰的首都是哪里, 我准备过去旅游 给我一些攻略",
//         tools.clone(),
//     )
//     .await?;

//     println!("Response {content:#?}");
    let system_prompt = r#"你是一个全能的助手。今天的日期是2026年7月31日。
你可以使用工具来搜索最新信息。
重要：当工具返回搜索结果时，你必须直接使用这些结果来回答，不要说"信息尚未公布"或"我不知道"。
你的训练数据有截止日期，可能已经过时，请始终优先信任工具返回的内容。"#;

//     let plan = chat_complete(
//         "Kimi-K3",
//         Some(system_prompt),
//         "14*6等于多少",
//         tools.clone(),
//     )
//     .await?;

    let plan = chat_complete(
        "Kimi-K3",
        Some(system_prompt,),
        "今天股票为什么下跌",
        tools.clone(),
    )
    .await?;

    tracing::info!("Response: {plan:#?}");
    Ok(())
}
