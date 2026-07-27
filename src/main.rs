use anyhow::Ok;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::llm::compiete::chat_complete;

mod llm;


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let content = chat_complete("grok-3", Some("你是一个全能的助手, 简单回答问题"), "爱尔兰的首都是哪里").await?;

    println!("Response {}", content);
    Ok(())
}