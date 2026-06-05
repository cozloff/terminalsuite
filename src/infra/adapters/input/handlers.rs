use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

use crate::app::dev_environment::StartDevEnvironment;
use crate::app::ports::dev_environment::DevEnvironmentProvisioner;
use crate::domain::dev_environment::{DevContainerSummary, DevEnvironmentSummary};

#[derive(Clone)]
pub struct AppState {
    dev_environment_provisioner: Arc<dyn DevEnvironmentProvisioner>,
}

impl AppState {
    pub fn new(dev_environment_provisioner: Arc<dyn DevEnvironmentProvisioner>) -> Self {
        Self {
            dev_environment_provisioner,
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct TestResponse {
    #[schema(example = "Hello, World!")]
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct StartPostgresResponse {
    #[schema(example = "Postgres Container Started!")]
    pub message: String,
    pub dev_environment: DevEnvironmentSummaryResponse,
}

#[derive(Serialize, ToSchema)]
pub struct DevEnvironmentSummaryResponse {
    #[schema(example = "terminalsuite-dev")]
    pub network: String,
    pub postgres: DevContainerSummaryResponse,
    pub api: DevContainerSummaryResponse,
    #[schema(example = "postgres://postgres:postgres@terminalsuite-postgres:5433/appdb")]
    pub database_url: String,
}

#[derive(Serialize, ToSchema)]
pub struct DevContainerSummaryResponse {
    #[schema(example = "terminalsuite-postgres")]
    pub name: String,
    #[schema(example = "postgres:16")]
    pub image: String,
    #[schema(example = "running")]
    pub status: String,
    #[schema(example = "127.0.0.1:5433")]
    pub host_address: String,
    #[schema(example = "terminalsuite-postgres:5433")]
    pub network_address: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "docker operation failed: Docker response server error")]
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
    description = "Starts the local development Postgres and API containers with Docker.",
    responses(
        (status = 200, description = "Successful response.", body = StartPostgresResponse),
        (status = 500, description = "Docker could not start the development environment.", body = ErrorResponse)
    )
)]
pub async fn start_postgres_container(
    State(state): State<AppState>,
) -> Result<Json<StartPostgresResponse>, (StatusCode, Json<ErrorResponse>)> {
    let use_case = StartDevEnvironment::new(state.dev_environment_provisioner.as_ref());
    let dev_environment = use_case.execute().await.map_err(internal_server_error)?;

    Ok(Json(StartPostgresResponse {
        message: "Development containers started.".to_string(),
        dev_environment: dev_environment.into(),
    }))
}

fn internal_server_error(error: impl std::fmt::Display) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            message: error.to_string(),
        }),
    )
}

impl From<DevEnvironmentSummary> for DevEnvironmentSummaryResponse {
    fn from(summary: DevEnvironmentSummary) -> Self {
        Self {
            network: summary.network,
            postgres: summary.postgres.into(),
            api: summary.api.into(),
            database_url: summary.database_url,
        }
    }
}

impl From<DevContainerSummary> for DevContainerSummaryResponse {
    fn from(summary: DevContainerSummary) -> Self {
        Self {
            name: summary.name,
            image: summary.image,
            status: summary.status,
            host_address: summary.host_address,
            network_address: summary.network_address,
        }
    }
}
