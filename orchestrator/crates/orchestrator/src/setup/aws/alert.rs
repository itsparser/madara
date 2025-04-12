use crate::core::client::SNS;
use crate::core::cloud::CloudProvider;
use crate::core::traits::resource::Resource;
use crate::types::params::AlertArgs;
use crate::{OrchestratorError, OrchestratorResult};
use anyhow::Context;
use async_trait::async_trait;
use aws_sdk_sns::Client as SNSClient;
use std::sync::Arc;

#[async_trait]
impl Resource for SNS {
    type SetupResult = ();
    type CheckResult = bool;
    type TeardownResult = ();
    type Error = ();
    type SetupArgs = AlertArgs;
    type CheckArgs = String;

    async fn new(provider: Arc<CloudProvider>) -> OrchestratorResult<Self> {
        match provider.as_ref() {
            CloudProvider::AWS(aws_config) => {
                let client = SNSClient::new(&aws_config);
                Ok(Self::constructor(Arc::new(client)))
            }
            _ => Err(OrchestratorError::InvalidCloudProviderError(
                "Mismatch Cloud Provider for S3Bucket resource".to_string(),
            ))?,
        }
    }

    async fn setup(&self, args: Self::SetupArgs) -> OrchestratorResult<Self::SetupResult> {
        tracing::info!("Setting up SNS topic");
        tracing::info!("Topic ARN: {}", args.endpoint);
        let topic_name = args.endpoint.clone();
        if self.check(&topic_name).await? {
            tracing::info!("SNS topic already exists, skipping");
            return Ok(());
        }
        let response = self
            .client
            .create_topic()
            .name(topic_name)
            .send()
            .await
            .context("Failed to create topic")
            .expect("Failed to create topic");
        let topic_arn = response.topic_arn().context("Failed to create topic").expect("Topic Not found");
        tracing::info!("SNS topic created. Topic ARN: {}", topic_arn);
        Ok(())
    }

    async fn check(&self, topic_name: &Self::CheckArgs) -> OrchestratorResult<Self::CheckResult> {
        Ok(self.client.get_topic_attributes().topic_arn(topic_name).send().await.is_ok())
    }

    async fn teardown(&self) -> OrchestratorResult<()> {
        Ok(())
    }
}
