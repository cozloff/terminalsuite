use crate::app::ports::dev_environment::{DevEnvironmentError, DevEnvironmentProvisioner};
use crate::domain::dev_environment::DevEnvironmentSummary;

pub struct StartDevEnvironment<'a, TProvisioner: ?Sized> {
    provisioner: &'a TProvisioner,
}

impl<'a, TProvisioner> StartDevEnvironment<'a, TProvisioner>
where
    TProvisioner: DevEnvironmentProvisioner + ?Sized,
{
    pub fn new(provisioner: &'a TProvisioner) -> Self {
        Self { provisioner }
    }

    pub async fn execute(&self) -> Result<DevEnvironmentSummary, DevEnvironmentError> {
        self.provisioner.start().await
    }
}
