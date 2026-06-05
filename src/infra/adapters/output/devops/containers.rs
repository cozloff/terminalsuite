use std::collections::HashMap;
use std::env;
use std::fmt;
use std::path::PathBuf;
use bollard::{
    Docker,
    errors::Error as DockerError,
    models::{
        ContainerCreateBody, HostConfig, NetworkCreateRequest, PortBinding, PortMap, RestartPolicy,
        RestartPolicyNameEnum,
    }, 
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, RemoveContainerOptionsBuilder,
        StartContainerOptions,
    }
}; 
use futures_util::{FutureExt, TryStreamExt};

use crate::app::ports::dev_environment::{
    DevEnvironmentError as AppDevEnvironmentError, DevEnvironmentProvisioner,
};
use crate::domain::dev_environment::{DevContainerSummary, DevEnvironmentSummary};

const NETWORK_NAME: &str = "terminalsuite-dev";
const POSTGRES_CONTAINER_NAME: &str = "terminalsuite-postgres";
const API_CONTAINER_NAME: &str = "terminalsuite-api";
const POSTGRES_IMAGE: &str = "postgres:16";
const API_IMAGE: &str = "rust:1.95-bookworm";
const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "appdb";
const POSTGRES_PORT: &str = "5433";
const API_CONTAINER_PORT: &str = "8082";
const API_HOST_PORT: &str = "8083";

#[derive(Debug)]
pub enum DevEnvironmentError {
    CurrentDirectory(std::io::Error),
    Docker(DockerError),
}

impl fmt::Display for DevEnvironmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentDirectory(error) => {
                write!(
                    formatter,
                    "could not read the current project directory: {error}"
                )
            }
            Self::Docker(error) => write!(formatter, "docker operation failed: {error}"),
        }
    }
}

impl std::error::Error for DevEnvironmentError {}

impl From<DockerError> for DevEnvironmentError {
    fn from(error: DockerError) -> Self {
        Self::Docker(error)
    }
}

pub struct DockerDevEnvironment {}

impl DockerDevEnvironment {
    pub fn new() -> Self {
        Self {}
    }
}

impl DevEnvironmentProvisioner for DockerDevEnvironment {
    fn start(
        &self,
    ) -> futures_util::future::BoxFuture<'_, Result<DevEnvironmentSummary, AppDevEnvironmentError>>
    {
        async move {
            DockerDevEnvironmentRuntime::from_local_docker()
                .map_err(|error| AppDevEnvironmentError::new(error.to_string()))?
                .provision()
                .await
                .map_err(|error| AppDevEnvironmentError::new(error.to_string()))
        }
        .boxed()
    }
}

struct DockerDevEnvironmentRuntime {
    docker: Docker,
    project_root: PathBuf,
}

impl DockerDevEnvironmentRuntime {
    fn from_local_docker() -> Result<Self, DevEnvironmentError> {
        let docker = Docker::connect_with_local_defaults()?;
        let project_root = env::current_dir().map_err(DevEnvironmentError::CurrentDirectory)?;

        Ok(Self {
            docker,
            project_root,
        })
    }

    async fn provision(&self) -> Result<DevEnvironmentSummary, DevEnvironmentError> {
        self.ensure_network().await?;
        self.pull_image(POSTGRES_IMAGE).await?;
        self.pull_image(API_IMAGE).await?;
        self.recreate_container(POSTGRES_CONTAINER_NAME).await?;
        self.recreate_container(API_CONTAINER_NAME).await?;
        self.create_postgres_container().await?;
        self.create_api_container().await?;
        self.start_container(POSTGRES_CONTAINER_NAME).await?;
        self.start_container(API_CONTAINER_NAME).await?;

        Ok(DevEnvironmentSummary {
            network: NETWORK_NAME.to_string(),
            postgres: DevContainerSummary {
                name: POSTGRES_CONTAINER_NAME.to_string(),
                image: POSTGRES_IMAGE.to_string(),
                status: "running".to_string(),
                host_address: format!("127.0.0.1:{POSTGRES_PORT}"),
                network_address: format!("{POSTGRES_CONTAINER_NAME}:{POSTGRES_PORT}"),
            },
            api: DevContainerSummary {
                name: API_CONTAINER_NAME.to_string(),
                image: API_IMAGE.to_string(),
                status: "running".to_string(),
                host_address: format!("127.0.0.1:{API_HOST_PORT}"),
                network_address: format!("{API_CONTAINER_NAME}:{API_CONTAINER_PORT}"),
            },
            database_url: database_url(),
        })
    }

    async fn ensure_network(&self) -> Result<(), DevEnvironmentError> {
        match self.docker.inspect_network(NETWORK_NAME, None).await {
            Ok(_) => Ok(()),
            Err(DockerError::DockerResponseServerError {
                status_code: 404, ..
            }) => {
                self.docker
                    .create_network(NetworkCreateRequest {
                        name: NETWORK_NAME.to_string(),
                        driver: Some("bridge".to_string()),
                        attachable: Some(true),
                        ..Default::default()
                    })
                    .await?;
                Ok(())
            }
            Err(error) => Err(error.into()),
        }
    }

    async fn pull_image(&self, image: &str) -> Result<(), DevEnvironmentError> {
        self.docker
            .create_image(
                Some(
                    CreateImageOptionsBuilder::default()
                        .from_image(image)
                        .build(),
                ),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;

        Ok(())
    }

    async fn recreate_container(&self, name: &str) -> Result<(), DevEnvironmentError> {
        match self
            .docker
            .remove_container(
                name,
                Some(
                    RemoveContainerOptionsBuilder::default()
                        .force(true)
                        .v(false)
                        .build(),
                ),
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(DockerError::DockerResponseServerError {
                status_code: 404, ..
            }) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    async fn create_postgres_container(&self) -> Result<(), DevEnvironmentError> {
        self.docker
            .create_container(
                Some(
                    CreateContainerOptionsBuilder::default()
                        .name(POSTGRES_CONTAINER_NAME)
                        .build(),
                ),
                ContainerCreateBody {
                    image: Some(POSTGRES_IMAGE.to_string()),
                    env: Some(vec![
                        format!("POSTGRES_USER={POSTGRES_USER}"),
                        format!("POSTGRES_PASSWORD={POSTGRES_PASSWORD}"),
                        format!("POSTGRES_DB={POSTGRES_DB}"),
                    ]),
                    exposed_ports: Some(vec![format!("{POSTGRES_PORT}/tcp")]),
                    host_config: Some(HostConfig {
                        network_mode: Some(NETWORK_NAME.to_string()),
                        port_bindings: Some(port_bindings(POSTGRES_PORT, POSTGRES_PORT)),
                        restart_policy: Some(restart_unless_stopped()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    async fn create_api_container(&self) -> Result<(), DevEnvironmentError> {
        let project_mount = format!("{}:/workspace", self.project_root.display());

        self.docker
            .create_container(
                Some(
                    CreateContainerOptionsBuilder::default()
                        .name(API_CONTAINER_NAME)
                        .build(),
                ),
                ContainerCreateBody {
                    image: Some(API_IMAGE.to_string()),
                    working_dir: Some("/workspace".to_string()),
                    cmd: Some(vec![
                        "sh".to_string(),
                        "-lc".to_string(),
                        "/usr/local/cargo/bin/cargo run".to_string(),
                    ]),
                    env: Some(vec![
                        format!("API_BIND_ADDRESS=0.0.0.0:{API_CONTAINER_PORT}"),
                        format!("DATABASE_URL={}", database_url()),
                        "PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
                            .to_string(),
                        "RUST_LOG=info".to_string(),
                    ]),
                    exposed_ports: Some(vec![format!("{API_CONTAINER_PORT}/tcp")]),
                    host_config: Some(HostConfig {
                        binds: Some(vec![project_mount]),
                        network_mode: Some(NETWORK_NAME.to_string()),
                        port_bindings: Some(port_bindings(API_CONTAINER_PORT, API_HOST_PORT)),
                        restart_policy: Some(restart_unless_stopped()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    async fn start_container(&self, name: &str) -> Result<(), DevEnvironmentError> {
        self.docker
            .start_container(name, None::<StartContainerOptions>)
            .await?;

        Ok(())
    }
}

fn database_url() -> String {
    format!(
        "postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{POSTGRES_CONTAINER_NAME}:{POSTGRES_PORT}/{POSTGRES_DB}"
    )
}

fn port_bindings(container_port: &str, host_port: &str) -> PortMap {
    let mut bindings = HashMap::new();
    bindings.insert(
        format!("{container_port}/tcp"),
        Some(vec![PortBinding {
            host_ip: Some("127.0.0.1".to_string()),
            host_port: Some(host_port.to_string()),
        }]),
    );
    bindings
}

fn restart_unless_stopped() -> RestartPolicy {
    RestartPolicy {
        name: Some(RestartPolicyNameEnum::UNLESS_STOPPED),
        maximum_retry_count: Some(0),
    }
}
