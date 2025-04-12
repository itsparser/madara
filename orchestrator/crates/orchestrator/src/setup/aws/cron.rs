use crate::core::client::cron::event_bridge::EventBridgeClient;
use crate::core::client::cron::CronClient;
use crate::core::cloud::CloudProvider;
use crate::core::traits::resource::Resource;
use crate::types::jobs::WorkerTriggerType;
use crate::types::params::CronArgs;
use crate::{OrchestratorError, OrchestratorResult};
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

lazy_static! {
    pub static ref WORKER_TRIGGERS: Vec<WorkerTriggerType> = vec![
        WorkerTriggerType::Snos,
        WorkerTriggerType::Proving,
        WorkerTriggerType::DataSubmission,
        WorkerTriggerType::UpdateState
    ];
}

#[async_trait]
impl Resource for EventBridgeClient {
    type SetupResult = ();
    type CheckResult = bool;
    type TeardownResult = ();
    type Error = ();
    type SetupArgs = CronArgs;
    type CheckArgs = ();

    async fn new(provider: Arc<CloudProvider>) -> OrchestratorResult<Self> {
        match provider.as_ref() {
            CloudProvider::AWS(aws_config) => {
                let eb_client = aws_sdk_eventbridge::Client::new(&aws_config);
                let scheduler_client = aws_sdk_scheduler::Client::new(&aws_config);
                let queue_client = aws_sdk_sqs::Client::new(&aws_config);
                let iam_client = aws_sdk_iam::Client::new(&aws_config);
                Ok(Self::constructor(
                    Arc::new(eb_client),
                    Arc::new(scheduler_client),
                    Arc::new(queue_client),
                    Arc::new(iam_client),
                ))
            }
            _ => Err(OrchestratorError::InvalidCloudProviderError(
                "Mismatch Cloud Provider for S3Bucket resource".to_string(),
            ))?,
        }
    }

    async fn setup(&self, args: Self::SetupArgs) -> OrchestratorResult<Self::SetupResult> {
        let trigger_arns = self
            .create_cron(
                args.target_queue_name.clone(),
                args.trigger_role_name.clone(),
                args.trigger_policy_name.clone(),
            )
            .await
            .map_err(|e| OrchestratorError::SetupCommandError(format!("Failed to create cron: {:?}", e)))?;
        sleep(Duration::from_secs(15)).await;

        for trigger in WORKER_TRIGGERS.iter() {
            self.add_cron_target_queue(
                trigger,
                &trigger_arns,
                args.trigger_rule_name.clone(),
                args.event_bridge_type.clone(),
                Duration::from_secs(args.cron_time.clone().parse::<u64>().map_err(|e| {
                    OrchestratorError::SetupCommandError(format!("Failed to parse the cron time: {:?}", e))
                })?),
            )
            .await
            .expect("Failed to add cron target queue");
        }
        Ok(())
    }

    async fn check(&self, args: &Self::CheckArgs) -> OrchestratorResult<Self::CheckResult> {
        todo!()
    }

    async fn teardown(&self) -> OrchestratorResult<()> {
        todo!()
    }
}
