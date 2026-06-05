pub mod openapi;

use crate::infra::adapters::input::handlers;
use crate::infra::adapters::output::devops::containers::DockerDevEnvironment;
use axum::response::Redirect;
use axum::{Router, routing::get};
use serde_json::json;
use std::sync::Arc;

pub fn create_router() -> Router {
    let scalar_configuration = json!({
        "url": "/api/openapi.json",
        "agent": { "disabled": true },
    });
    let app_state = handlers::AppState::new(Arc::new(DockerDevEnvironment::new()));

    let api_routes: Router =
        Router::new()
            .route(
                "/docs",
                get(move || {
                    let scalar_configuration = scalar_configuration.clone();
                    async move {
                        scalar_api_reference::axum::scalar_response(&scalar_configuration, None)
                    }
                }),
            )
            .route("/openapi.json", get(openapi::api_openapi))
            .route("/test", get(handlers::test_handler))
            .route("/start_postgres", get(handlers::start_postgres_container))
            .with_state(app_state);

    return Router::new()
        .route("/api", get(|| async { Redirect::to("/api/docs") }))
        .nest("/api", api_routes);
}
