use axum::Json;
use utoipa::OpenApi;

use crate::infra::adapters::input::handlers::{
    self, DevContainerSummaryResponse, DevEnvironmentSummaryResponse, ErrorResponse,
    StartPostgresResponse, TestResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(handlers::test_handler, handlers::start_postgres_container),
    components(schemas(
        TestResponse,
        StartPostgresResponse,
        ErrorResponse,
        DevEnvironmentSummaryResponse,
        DevContainerSummaryResponse
    )),
    info(
        title = "TerminalSuite API",
        version = "0.1.0",
        description = "TerminalSuite API documentation."
    )
)]
struct ApiDoc;

pub async fn api_openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
