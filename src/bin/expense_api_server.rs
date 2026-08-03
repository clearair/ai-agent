use ai_agent::expense_api::{AppState, app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("EXPENSE_API_KEY").unwrap_or_else(|_| "dev-secret-key".to_string());
    let address =
        std::env::var("EXPENSE_API_BIND").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&address).await?;

    tracing::info!(%address, "expense API server listening");
    axum::serve(listener, app(AppState::new(), api_key)).await?;
    Ok(())
}
