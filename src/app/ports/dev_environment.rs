use std::fmt;

use futures_util::future::BoxFuture;

use crate::domain::dev_environment::DevEnvironmentSummary;

#[derive(Debug, Clone)]
pub struct DevEnvironmentError {
    message: String,
}

impl DevEnvironmentError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for DevEnvironmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for DevEnvironmentError {}

pub trait DevEnvironmentProvisioner: Send + Sync {
    fn start(&self) -> BoxFuture<'_, Result<DevEnvironmentSummary, DevEnvironmentError>>;
}
