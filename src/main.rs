pub mod infra;
pub mod app;
pub mod domain;

use tokio::net::TcpListener;
use infra::adapters::input::routes;

#[tokio::main]
async fn main() {
    // Bind on 8080 locally
    let bind_address: String = std::env::var(
        "API_BIND_ADDRESS")  
            .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Create the router and serve the application
    let app: axum::Router = routes::create_router();
    let listener: TcpListener = TcpListener::bind(
        &bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}