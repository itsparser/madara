use super::super::cloud::CloudProvider;
use crate::OrchestratorResult;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

/// Resource trait
///
/// The Resource trait is used to define the interface for resources that can be used by the Orchestrator.
/// It provides a common interface for interacting with resources, such as databases, queues, and storage.
/// The trait is used to abstract away the specific implementation of the resource, allowing the Orchestrator
/// to work with a variety of resources without needing to know the specific details of each implementation.
/// The trait is also used to define the interface for mocking the resource, allowing the Orchestrator to
/// be tested in isolation without needing to set up a real resource.
// #[automock] // TODO: uncomment this when automock is fixed
#[async_trait]
pub trait Resource: Send + Sync {
    type SetupResult: Send + Sync;
    type CheckResult: Send + Sync;
    type TeardownResult: Send + Sync;
    type Error: Send + Sync;
    type SetupArgs: Send + Sync;
    type CheckArgs: Send + Sync;

    async fn new(provider: Arc<CloudProvider>) -> OrchestratorResult<Self>
    where
        Self: Sized;

    async fn setup(&self, args: Self::SetupArgs) -> OrchestratorResult<Self::SetupResult>;
    async fn check(&self, args: &Self::CheckArgs) -> OrchestratorResult<bool>;
    async fn teardown(&self) -> OrchestratorResult<()>;
    async fn poll(&self, args: Self::CheckArgs, poll_interval: u64, timeout: u64) -> bool {
        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(timeout);
        let poll_duration = Duration::from_secs(poll_interval);

        while start_time.elapsed() < timeout_duration {
            match self.check(&args).await {
                Ok(result) => {
                    if result {
                        return true;
                    } else {
                        tokio::time::sleep(poll_duration).await;
                    }
                }
                Err(_) => return false,
            }
        }
        false
    }
}
