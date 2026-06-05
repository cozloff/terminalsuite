#[derive(Debug, Clone)]
pub struct DevEnvironmentSummary {
    pub network: String,
    pub postgres: DevContainerSummary,
    pub api: DevContainerSummary,
    pub database_url: String,
}

#[derive(Debug, Clone)]
pub struct DevContainerSummary {
    pub name: String,
    pub image: String,
    pub status: String,
    pub host_address: String,
    pub network_address: String,
}
