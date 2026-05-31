pub mod openapi;

use axum::{Router, routing::get};
use serde_json::json;
use crate::infra::adapters::input::handlers;

pub fn create_router() -> Router {
    let scalar_configuration = json!({
        "url": "/api/openapi.json",
        "agent": { "disabled": true },
    });

    let api_routes: Router = Router::new().route(
        "/test", 
        get(handlers::test_handler));

    return Router::new()
        .route(
            "/api",
            get(move || {
                let scalar_configuration = scalar_configuration.clone();
                async move {
                    scalar_api_reference::axum::scalar_response(
                        &scalar_configuration, None)
                }
            }),
        )
        .route(
            "/api/openapi.json", 
            get(
                openapi::api_openapi))
        .nest("/api", api_routes);
}
