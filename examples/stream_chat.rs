use ai_agent::llm::{semaphore::get_semaphore, stream::chat_stream_with_retry};
use tokio::task::JoinSet;
use tracing::{Instrument, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let prompts = vec![
        "Rust 声明周期是什么",
        "TCP 三次握手时什么",
        "料理实验室这个书如何",
        "东野圭吾的一生写了多少本书",
        "嫌疑人X的献身简介",
    ];

    let mut set = JoinSet::new();

    for prompt in prompts {
        let span = tracing::info_span!("Chat", prompt = prompt);

        set.spawn(
            async move {
                tracing::info!("\n\n{prompt}");
                let permit = get_semaphore().acquire().await?;
                let output = chat_stream_with_retry(
                    "grok-3",
                    Some("你是一个全能的助手, 简单回答问题"),
                    prompt,
                )
                .await?;
                drop(permit);
                Ok::<_, anyhow::Error>((prompt, output))
            }
            .instrument(span),
        );
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok((prompt, result))) => tracing::info!("\n{prompt}\n{result}"),
            Ok(Err(err)) => tracing::error!("Task panicked: {err}"),
            Err(err) => tracing::error!("Task panicked: {err}"),
        }
    }

    Ok(())
}
