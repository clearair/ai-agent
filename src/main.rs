use ai_agent::llm::{compiete::chat_complete, structured::chat_complete_structured, structured_ds::chat_complete_structured_ds};
use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let content = chat_complete_structured_ds(
        "DeepSeek-V4-Flash",
        // Some("你是一个全能的助手, 简单回答问题"),
        "爱尔兰的首都是哪里, 我准备过去旅游 给我一些攻略",
    )
    .await?;

    println!("Response {content:#?}");
    Ok(())
}
