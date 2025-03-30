use crate::error::{OrchestratorError, OrchestratorResult};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Env {
    // AWS Configuration
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub aws_region: String,
    pub aws_endpoint_url: Option<String>,
    pub aws_default_region: Option<String>,

    // Resource Prefixes and Names
    pub prefix: String,
    pub s3_bucket_name: String,
    pub sqs_base_url: String,
    pub sqs_suffix: String,

    // EventBridge Configuration
    pub event_bridge_type: String,
    pub event_bridge_trigger_rule_name: String,
    pub event_bridge_trigger_role_name: String,
    pub event_bridge_trigger_policy_name: String,
    pub event_bridge_target_queue_name: String,

    // Alerts Configuration
    pub aws_sns_arn: String,

    // Data Availability Configuration
    pub ethereum_da_rpc_url: String,

    // Database Configuration
    pub mongodb_connection_url: String,
    pub database_name: String,

    // Prover Configuration
    // SHARP
    pub sharp_customer_id: String,
    pub sharp_url: String,
    pub sharp_user_crt: String,
    pub sharp_user_key: String,
    pub sharp_server_crt: String,
    pub sharp_rpc_node_url: String,
    pub sharp_proof_layout: String,
    pub gps_verifier_contract_address: String,

    // Atlantic
    pub atlantic_api_key: String,
    pub atlantic_service_url: String,
    pub atlantic_mock_fact_hash: String,
    pub atlantic_prover_type: String,
    pub atlantic_settlement_layer: String,
    pub atlantic_verifier_contract_address: String,
    pub atlantic_rpc_node_url: String,

    // Settlement Configuration
    // Ethereum
    pub ethereum_settlement_rpc_url: String,
    pub ethereum_private_key: String,
    pub l1_core_contract_address: String,
    pub starknet_operator_address: String,

    // Starknet
    pub starknet_settlement_rpc_url: String,
    pub starknet_private_key: String,
    pub starknet_account_address: String,
    pub starknet_cairo_core_contract_address: Option<String>,
    pub starknet_finality_retry_wait_in_secs: u64,

    // Server Configuration
    pub host: String,
    pub port: u16,

    // Service Configuration
    pub max_block_no_to_process: Option<u64>,
    pub min_block_no_to_process: Option<u64>,
    pub madara_rpc_url: String,

    // SNOS Configuration
    pub rpc_for_snos: String,

    // Instrumentation
    pub otel_service_name: String,
}

impl Env {
    pub fn load() -> OrchestratorResult<Self> {
        // Load .env file if it exists
        dotenv().ok();

        Ok(Self {
            // AWS Configuration
            aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").ok(),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").ok(),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-west-1".to_string()),
            aws_endpoint_url: env::var("AWS_ENDPOINT_URL").ok(),
            aws_default_region: env::var("AWS_DEFAULT_REGION").ok(),

            // Resource Prefixes and Names
            prefix: env::var("MADARA_ORCHESTRATOR_PREFIX").unwrap_or_else(|_| "orchestrator".to_string()),
            s3_bucket_name: env::var("MADARA_ORCHESTRATOR_AWS_S3_BUCKET_NAME").unwrap_or_else(|_| "bucket".to_string()),
            sqs_base_url: env::var("MADARA_ORCHESTRATOR_SQS_BASE_QUEUE_URL")
                .unwrap_or_else(|_| "https://sqs.us-west-1.amazonaws.com/".to_string()),
            sqs_suffix: env::var("MADARA_ORCHESTRATOR_SQS_SUFFIX").unwrap_or_else(|_| "queue".to_string()),

            // EventBridge Configuration
            event_bridge_type: env::var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TYPE").unwrap_or_else(|_| "rule".to_string()),
            event_bridge_trigger_rule_name: env::var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TRIGGER_RULE_NAME")
                .map_err(|_| OrchestratorError::ConfigError("EVENT_BRIDGE_TRIGGER_RULE_NAME not set".to_string()))?,
            event_bridge_trigger_role_name: env::var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TRIGGER_ROLE_NAME")
                .map_err(|_| OrchestratorError::ConfigError("EVENT_BRIDGE_TRIGGER_ROLE_NAME not set".to_string()))?,
            event_bridge_trigger_policy_name: env::var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TRIGGER_POLICY_NAME")
                .map_err(|_| OrchestratorError::ConfigError("EVENT_BRIDGE_TRIGGER_POLICY_NAME not set".to_string()))?,
            event_bridge_target_queue_name: env::var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TARGET_QUEUE_NAME")
                .map_err(|_| OrchestratorError::ConfigError("EVENT_BRIDGE_TARGET_QUEUE_NAME not set".to_string()))?,

            // Alerts Configuration
            aws_sns_arn: env::var("MADARA_ORCHESTRATOR_AWS_SNS_ARN")
                .map_err(|_| OrchestratorError::ConfigError("AWS_SNS_ARN not set".to_string()))?,

            // Data Availability Configuration
            ethereum_da_rpc_url: env::var("MADARA_ORCHESTRATOR_ETHEREUM_DA_RPC_URL")
                .map_err(|_| OrchestratorError::ConfigError("ETHEREUM_DA_RPC_URL not set".to_string()))?,

            // Database Configuration
            mongodb_connection_url: env::var("MADARA_ORCHESTRATOR_MONGODB_CONNECTION_URL")
                .map_err(|_| OrchestratorError::ConfigError("MONGODB_CONNECTION_URL not set".to_string()))?,
            database_name: env::var("MADARA_ORCHESTRATOR_DATABASE_NAME")
                .map_err(|_| OrchestratorError::ConfigError("DATABASE_NAME not set".to_string()))?,

            // Prover Configuration
            sharp_customer_id: env::var("MADARA_ORCHESTRATOR_SHARP_CUSTOMER_ID")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_CUSTOMER_ID not set".to_string()))?,
            sharp_url: env::var("MADARA_ORCHESTRATOR_SHARP_URL")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_URL not set".to_string()))?,
            sharp_user_crt: env::var("MADARA_ORCHESTRATOR_SHARP_USER_CRT")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_USER_CRT not set".to_string()))?,
            sharp_user_key: env::var("MADARA_ORCHESTRATOR_SHARP_USER_KEY")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_USER_KEY not set".to_string()))?,
            sharp_server_crt: env::var("MADARA_ORCHESTRATOR_SHARP_SERVER_CRT")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_SERVER_CRT not set".to_string()))?,
            sharp_rpc_node_url: env::var("MADARA_ORCHESTRATOR_SHARP_RPC_NODE_URL")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_RPC_NODE_URL not set".to_string()))?,
            sharp_proof_layout: env::var("MADARA_ORCHESTRATOR_SHARP_PROOF_LAYOUT")
                .map_err(|_| OrchestratorError::ConfigError("SHARP_PROOF_LAYOUT not set".to_string()))?,
            gps_verifier_contract_address: env::var("MADARA_ORCHESTRATOR_GPS_VERIFIER_CONTRACT_ADDRESS")
                .map_err(|_| OrchestratorError::ConfigError("GPS_VERIFIER_CONTRACT_ADDRESS not set".to_string()))?,

            // Atlantic Configuration
            atlantic_api_key: env::var("MADARA_ORCHESTRATOR_ATLANTIC_API_KEY")
                .map_err(|_| OrchestratorError::ConfigError("ATLANTIC_API_KEY not set".to_string()))?,
            atlantic_service_url: env::var("MADARA_ORCHESTRATOR_ATLANTIC_SERVICE_URL")
                .map_err(|_| OrchestratorError::ConfigError("ATLANTIC_SERVICE_URL not set".to_string()))?,
            atlantic_mock_fact_hash: env::var("MADARA_ORCHESTRATOR_ATLANTIC_MOCK_FACT_HASH")
                .map_err(|_| OrchestratorError::ConfigError("ATLANTIC_MOCK_FACT_HASH not set".to_string()))?,
            atlantic_prover_type: env::var("MADARA_ORCHESTRATOR_ATLANTIC_PROVER_TYPE")
                .map_err(|_| OrchestratorError::ConfigError("ATLANTIC_PROVER_TYPE not set".to_string()))?,
            atlantic_settlement_layer: env::var("MADARA_ORCHESTRATOR_ATLANTIC_SETTLEMENT_LAYER")
                .map_err(|_| OrchestratorError::ConfigError("ATLANTIC_SETTLEMENT_LAYER not set".to_string()))?,
            atlantic_verifier_contract_address: env::var("MADARA_ORCHESTRATOR_ATLANTIC_VERIFIER_CONTRACT_ADDRESS")
                .map_err(|_| {
                    OrchestratorError::ConfigError("ATLANTIC_VERIFIER_CONTRACT_ADDRESS not set".to_string())
                })?,
            atlantic_rpc_node_url: env::var("MADARA_ORCHESTRATOR_ATLANTIC_RPC_NODE_URL")
                .map_err(|_| OrchestratorError::ConfigError("ATLANTIC_RPC_NODE_URL not set".to_string()))?,

            // Settlement Configuration
            ethereum_settlement_rpc_url: env::var("MADARA_ORCHESTRATOR_ETHEREUM_SETTLEMENT_RPC_URL")
                .map_err(|_| OrchestratorError::ConfigError("ETHEREUM_SETTLEMENT_RPC_URL not set".to_string()))?,
            ethereum_private_key: env::var("MADARA_ORCHESTRATOR_ETHEREUM_PRIVATE_KEY")
                .map_err(|_| OrchestratorError::ConfigError("ETHEREUM_PRIVATE_KEY not set".to_string()))?,
            l1_core_contract_address: env::var("MADARA_ORCHESTRATOR_L1_CORE_CONTRACT_ADDRESS")
                .map_err(|_| OrchestratorError::ConfigError("L1_CORE_CONTRACT_ADDRESS not set".to_string()))?,
            starknet_operator_address: env::var("MADARA_ORCHESTRATOR_STARKNET_OPERATOR_ADDRESS")
                .map_err(|_| OrchestratorError::ConfigError("STARKNET_OPERATOR_ADDRESS not set".to_string()))?,

            // Starknet Configuration
            starknet_settlement_rpc_url: env::var("MADARA_ORCHESTRATOR_STARKNET_SETTLEMENT_RPC_URL")
                .map_err(|_| OrchestratorError::ConfigError("STARKNET_SETTLEMENT_RPC_URL not set".to_string()))?,
            starknet_private_key: env::var("MADARA_ORCHESTRATOR_STARKNET_PRIVATE_KEY")
                .map_err(|_| OrchestratorError::ConfigError("STARKNET_PRIVATE_KEY not set".to_string()))?,
            starknet_account_address: env::var("MADARA_ORCHESTRATOR_STARKNET_ACCOUNT_ADDRESS")
                .map_err(|_| OrchestratorError::ConfigError("STARKNET_ACCOUNT_ADDRESS not set".to_string()))?,
            starknet_cairo_core_contract_address: env::var("MADARA_ORCHESTRATOR_STARKNET_CAIRO_CORE_CONTRACT_ADDRESS")
                .ok(),
            starknet_finality_retry_wait_in_secs: env::var("MADARA_ORCHESTRATOR_STARKNET_FINALITY_RETRY_WAIT_IN_SECS")
                .map(|v| v.parse::<u64>().unwrap_or(10))
                .unwrap_or(10),

            // Server Configuration
            host: env::var("MADARA_ORCHESTRATOR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("MADARA_ORCHESTRATOR_PORT").map(|v| v.parse::<u16>().unwrap_or(3000)).unwrap_or(3000),

            // Service Configuration
            max_block_no_to_process: env::var("MADARA_ORCHESTRATOR_MAX_BLOCK_NO_TO_PROCESS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok()),
            min_block_no_to_process: env::var("MADARA_ORCHESTRATOR_MIN_BLOCK_NO_TO_PROCESS")
                .ok()
                .and_then(|v| v.parse::<u64>().ok()),
            madara_rpc_url: env::var("MADARA_ORCHESTRATOR_MADARA_RPC_URL")
                .map_err(|_| OrchestratorError::ConfigError("MADARA_RPC_URL not set".to_string()))?,

            // SNOS Configuration
            rpc_for_snos: env::var("MADARA_ORCHESTRATOR_RPC_FOR_SNOS")
                .map_err(|_| OrchestratorError::ConfigError("RPC_FOR_SNOS not set".to_string()))?,

            // Instrumentation
            otel_service_name: env::var("MADARA_ORCHESTRATOR_OTEL_SERVICE_NAME")
                .map_err(|_| OrchestratorError::ConfigError("OTEL_SERVICE_NAME not set".to_string()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_values() {
        // Clear all relevant environment variables
        env::remove_var("AWS_REGION");
        env::remove_var("MADARA_ORCHESTRATOR_PREFIX");
        env::remove_var("MADARA_ORCHESTRATOR_AWS_S3_BUCKET_NAME");
        env::remove_var("MADARA_ORCHESTRATOR_SQS_BASE_QUEUE_URL");
        env::remove_var("MADARA_ORCHESTRATOR_SQS_SUFFIX");
        env::remove_var("MADARA_ORCHESTRATOR_HOST");
        env::remove_var("MADARA_ORCHESTRATOR_PORT");

        // Set required environment variables for the test
        env::set_var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TRIGGER_RULE_NAME", "test-rule");
        env::set_var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TRIGGER_ROLE_NAME", "test-role");
        env::set_var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TRIGGER_POLICY_NAME", "test-policy");
        env::set_var("MADARA_ORCHESTRATOR_EVENT_BRIDGE_TARGET_QUEUE_NAME", "test-queue");
        env::set_var("MADARA_ORCHESTRATOR_AWS_SNS_ARN", "test-arn");
        // ... set other required variables ...

        let env = Env::load().unwrap();

        assert_eq!(env.aws_region, "us-west-1");
        assert_eq!(env.prefix, "orchestrator");
        assert_eq!(env.s3_bucket_name, "bucket");
        assert_eq!(env.sqs_base_url, "https://sqs.us-west-1.amazonaws.com/");
        assert_eq!(env.sqs_suffix, "queue");
        assert_eq!(env.host, "127.0.0.1");
        assert_eq!(env.port, 3000);
        assert_eq!(env.starknet_finality_retry_wait_in_secs, 10);
    }
}
