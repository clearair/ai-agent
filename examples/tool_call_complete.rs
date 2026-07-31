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

    let content = chat_complete(
        "grok-3",
        Some("你是一个全能的助手, 简单回答问题"),
        "爱尔兰的首都是哪里, 我准备过去旅游 给我一些攻略",
        tools.clone(),
    )
    .await?;

    println!("Response {content:#?}");

    let plan = chat_complete(
        "Kimi-K3",
        Some("你是一个全能的助手, 简单回答问题"),
        "14*6等于多少",
        tools.clone(),
    )
    .await?;

    tracing::info!("Response: {plan:#?}");
    Ok(())
}
