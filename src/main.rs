pub mod infra;
pub mod app;
pub mod domain;

use tokio::net::TcpListener;
use infra::adapters::input::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind on 8081 locally by default.
    let bind_address: String = std::env::var(
        "API_BIND_ADDRESS")  
            .unwrap_or_else(|_| "127.0.0.1:8081".to_string());

    // Create the router and serve the application
    let app: axum::Router = routes::create_router();
    let listener: TcpListener = TcpListener::bind(
        &bind_address).await.map_err(|err| {
            format!("failed to bind to {bind_address}: {err}")
        })?;

    println!("TerminalSuite API listening on http://{bind_address}");
    println!("Scalar API docs available at http://{bind_address}/api");

    axum::serve(listener, app).await?;
    Ok(())
}
