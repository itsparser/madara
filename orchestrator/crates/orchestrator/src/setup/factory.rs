use crate::core::client::cron::event_bridge::EventBridgeClient;
use crate::core::client::SNS;
use crate::core::traits::resource::Resource;
use crate::setup::creator::{
    EventBridgeResourceCreator, ResourceCreator, ResourceType, S3ResourceCreator, SNSResourceCreator,
    SQSResourceCreator,
};
use crate::setup::wrapper::ResourceWrapper;
use crate::types::params::MiscellaneousArgs;
use crate::{
    core::client::storage::sss::AWSS3,
    core::client::SQS,
    core::cloud::CloudProvider,
    types::params::{AlertArgs, CronArgs, QueueArgs, StorageArgs},
    OrchestratorError, OrchestratorResult,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tracing::{error, info};

/// ResourceFactory is responsible for creating resources based on their type
pub struct ResourceFactory {
    creators: HashMap<ResourceType, Box<dyn ResourceCreator>>,
    resource_status: Mutex<HashMap<ResourceType, bool>>,
    cloud_provider: Arc<CloudProvider>,
    queue_params: QueueArgs,
    cron_params: CronArgs,
    storage_params: StorageArgs,
    alert_params: AlertArgs,
    miscellaneous_params: MiscellaneousArgs,
}

impl ResourceFactory {
    /// new_with_gcs - Create a new ResourceFactory with default resource creators for Orchestrator
    /// with GCS Cloud Provider
    pub fn new_with_gcs(
        cloud_provider: Arc<CloudProvider>,
        queue_params: QueueArgs,
        cron_params: CronArgs,
        storage_params: StorageArgs,
        alert_params: AlertArgs,
        miscellaneous_params: MiscellaneousArgs,
    ) -> Self {
        let creators = HashMap::new();
        let resource_status = Mutex::new(HashMap::new());
        ResourceFactory {
            creators,
            resource_status,
            cloud_provider,
            queue_params,
            cron_params,
            storage_params,
            alert_params,
            miscellaneous_params,
        }
    }

    /// new_with_aws - Create a new ResourceFactory with default resource creators for Orchestrator
    /// with AWS Cloud Provider
    pub fn new_with_aws(
        cloud_provider: Arc<CloudProvider>,
        queue_params: QueueArgs,
        cron_params: CronArgs,
        storage_params: StorageArgs,
        alert_params: AlertArgs,
        miscellaneous_params: MiscellaneousArgs,
    ) -> Self {
        let mut creators = HashMap::new();
        let resource_status = Mutex::new(HashMap::new());
        creators.insert(ResourceType::Storage, Box::new(S3ResourceCreator) as Box<dyn ResourceCreator>);
        creators.insert(ResourceType::Queue, Box::new(SQSResourceCreator) as Box<dyn ResourceCreator>);
        creators.insert(ResourceType::Cron, Box::new(EventBridgeResourceCreator) as Box<dyn ResourceCreator>);
        creators.insert(ResourceType::Notification, Box::new(SNSResourceCreator) as Box<dyn ResourceCreator>);

        ResourceFactory {
            creators,
            resource_status,
            cloud_provider,
            queue_params,
            cron_params,
            storage_params,
            alert_params,
            miscellaneous_params,
        }
    }

    pub async fn setup_resource(&mut self) -> OrchestratorResult<()> {
        let mut is_queue_ready: bool = false;
        for (resource_type, creator) in self.creators.iter() {
            info!(" ⏳ Setting up resource: {:?}", resource_type);
            let mut resource = creator.create_resource(self.cloud_provider.clone()).await?;
            match resource_type {
                ResourceType::Storage => {
                    let rs = resource.downcast_mut::<AWSS3>().unwrap();
                    rs.setup(self.storage_params.clone()).await?;
                    let is_bucket_ready = rs
                        .poll(
                            self.storage_params.bucket_name.clone(),
                            self.miscellaneous_params.poll_interval,
                            self.miscellaneous_params.timeout,
                        )
                        .await;
                    self.update_resource_status(ResourceType::Storage, is_bucket_ready)?;
                }
                ResourceType::Queue => {
                    let rs = resource.downcast_mut::<SQS>().unwrap();
                    rs.setup(self.queue_params.clone()).await?;
                    is_queue_ready = rs
                        .poll(
                            (self.queue_params.queue_base_url.clone(), self.queue_params.clone()),
                            self.miscellaneous_params.poll_interval,
                            self.miscellaneous_params.timeout,
                        )
                        .await;
                    self.update_resource_status(ResourceType::Queue, is_queue_ready)?;
                }
                ResourceType::Notification => {
                    let rs = resource.downcast_mut::<SNS>().unwrap();
                    rs.setup(self.alert_params.clone()).await?;
                    let is_sns_ready = rs
                        .poll(
                            self.alert_params.endpoint.clone(),
                            self.miscellaneous_params.poll_interval,
                            self.miscellaneous_params.timeout,
                        )
                        .await;
                    self.update_resource_status(ResourceType::Notification, is_sns_ready)?;
                }
                ResourceType::Cron => {
                    let start_time = std::time::Instant::now();
                    let timeout_duration = Duration::from_secs(self.miscellaneous_params.timeout);
                    let poll_duration = Duration::from_secs(self.miscellaneous_params.poll_interval);
                    let mut can_setup_cron = false;
                    while start_time.elapsed() < timeout_duration {
                        if is_queue_ready {
                            can_setup_cron = true;
                            break;
                        } else {
                            tokio::time::sleep(poll_duration).await;
                        }
                    }
                    if can_setup_cron {
                        let rs = resource.downcast_mut::<EventBridgeClient>().unwrap();
                        rs.setup(self.cron_params.clone()).await?;
                    } else {
                        error!(" ❌ Failed to setup cron because queues are not ready yet");
                    }
                }
                _ => {}
            }
            info!(" ✅ Resource setup completed: {:?}", resource_type);
        }
        Ok(())
    }

    /// Register a new resource creator
    pub fn register(&mut self, resource_type: ResourceType, creator: Box<dyn ResourceCreator>) {
        self.creators.insert(resource_type, creator);
    }

    /// Create a resource of the specified type
    pub async fn create_resource(
        &self,
        resource_type: ResourceType,
        cloud_provider: Arc<CloudProvider>,
    ) -> OrchestratorResult<ResourceWrapper> {
        match self.creators.get(&resource_type) {
            Some(creator) => creator.create_resource(cloud_provider).await,
            None => Err(OrchestratorError::ResourceError(format!(
                "No creator registered for resource type {:?}",
                resource_type
            ))),
        }
    }

    /// Create a resource from a string type
    pub async fn create_resource_from_str(
        &self,
        resource_type: &str,
        cloud_provider: Arc<CloudProvider>,
    ) -> OrchestratorResult<ResourceWrapper> {
        match ResourceType::from_str(resource_type) {
            Some(rt) => self.create_resource(rt, cloud_provider).await,
            None => Err(OrchestratorError::ResourceError(format!("Unknown resource type: {}", resource_type))),
        }
    }

    pub fn update_resource_status(&self, resource_type: ResourceType, is_ready: bool) -> OrchestratorResult<()> {
        match self.resource_status.lock() {
            Ok(mut status) => {
                status.insert(resource_type, is_ready);
                Ok(())
            }
            Err(_) => Err(OrchestratorError::ResourceError("Failed to acquire lock on resource_status".to_string())),
        }
    }

    pub fn get_resource_status(&self, resource_type: ResourceType) -> OrchestratorResult<bool> {
        match self.resource_status.lock() {
            Ok(status) => Ok(status.get(&resource_type).cloned().unwrap_or(false)),
            Err(_) => Err(OrchestratorError::ResourceError("Failed to acquire lock on resource_status".to_string())),
        }
    }
}
