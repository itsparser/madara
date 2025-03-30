use crate::args::provider::{AWSConfigValidatedArgs, ProviderValidatedArgs};
use crate::args::queue::QueueValidatedArgs;
use crate::args::SetupCmd;
use crate::core::cloud::CloudProvider;
use crate::resource::aws::s3::SSS;
use crate::resource::aws::sqs::SQS;
use crate::resource::Resource;
use crate::{OrchestratorError, OrchestratorResult};
use aws_config::meta::region::RegionProviderChain;
use aws_config::{Region, SdkConfig};
use aws_credential_types::Credentials;
use std::sync::Arc;
use tracing::{debug, info};

pub fn setup(setup_cmd: &SetupCmd) -> OrchestratorResult<()> {
    // Initialize the cloud provider
    info!(" 🌎 Initializing Cloud Provider");
    setup_cmd.validate_provider_params();

    // let cloud_provider = match setup_cmd.aws_config_args {
    //     "aws" => CloudProvider::AWS(Box::new(aws_config::load_from_env().await?)),
    //     _ => {
    //         return Err(OrchestratorError::InvalidCloudProviderError(format!(
    //             "Invalid cloud provider: {}",
    //             setup_cmd.cloud_provider
    //         )))
    //     }
    // };
    Ok(())
}

/// To build a `SdkConfig` for AWS provider.
pub async fn get_aws_config(aws_config: &AWSConfigValidatedArgs) -> SdkConfig {
    let region = aws_config.aws_region.clone();
    let region_provider = RegionProviderChain::first_try(Region::new(region)).or_default_provider();
    let credentials =
        Credentials::from_keys(aws_config.aws_access_key_id.clone(), aws_config.aws_secret_access_key.clone(), None);
    aws_config::from_env().credentials_provider(credentials).region(region_provider).load().await
}

/// Builds the provider config
pub async fn build_provider_config(provider_params: &ProviderValidatedArgs) -> Arc<CloudProvider> {
    match provider_params {
        ProviderValidatedArgs::AWS(aws_params) => {
            Arc::new(CloudProvider::AWS(Box::new(get_aws_config(aws_params).await)))
        }
    }
}
async fn setup_cloud(setup_cmd: &SetupCmd) -> OrchestratorResult<()> {
    info!(" 🌎 Initializing Setup Cloud");
    let provider = setup_cmd.validate_provider_params().map_err(|e| OrchestratorError::SetupCommandError(e))?;
    let cloud_provider = build_provider_config(&provider).await;
    let aws_config = cloud_provider.get_aws_client_or_panic();
    debug!("Cloud provider setup completed, Provider: {}", cloud_provider);

    info!("Starting setup of the Resource");
    let queue_params = setup_cmd.validate_queue_params().map_err(|e| OrchestratorError::SetupCommandError(e))?;
    match queue_params {
        QueueValidatedArgs::AWSSQS(aws_sqs_params) => {}
    }

    let resource: Vec<Box<dyn Resource>> =
        vec![Box::new(SQS::new(cloud_provider.clone()).await?), Box::new(SSS::new(cloud_provider.clone()).await?)];
    for resource in resource {
        resource
            .setup(setup_cmd.validate_provider_params().map_err(|e| OrchestratorError::SetupCommandError(e))?)
            .await?;
    }

    Ok(())
}

fn setup_db() -> OrchestratorResult<()> {
    Ok(())
}
