use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct TestResponse {
    #[schema(example = "Hello, World!")]
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct StartPostgresResponse {
    #[schema(example = "Postgres Container Started!")]
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/api/test",
    summary = "Test Endpoint",
    description = "Just a test endpoint.",
    responses(
        (status = 200, description = "Successful response.", body = TestResponse)
    )
)]
pub async fn test_handler() -> Json<TestResponse> {
    Json(TestResponse {
        message: "Hello, World!".to_string(),
    })
}

#[utoipa::path(
    get,
    path = "/api/start_postgres",
    summary = "Start Postgres Container",
    description = "Starts a Postgres container.",
    responses(
        (status = 200, description = "Successful response.", body = StartPostgresResponse)
    )
)]
pub async fn start_postgres_container() -> Json<StartPostgresResponse> {
    Json(StartPostgresResponse {
        message: "Postgres Container Started!".to_string(),
    })
}
