use std::sync::Arc;

use async_trait::async_trait;
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use chrono::{SubsecRound, Utc};
use color_eyre::eyre::{eyre, WrapErr};
use orchestrator_prover_client_interface::{Task, TaskStatus};
use thiserror::Error;
use uuid::Uuid;

use crate::core::config::Config;
use crate::error::job::proving::ProvingError;
use crate::error::job::JobError;
use crate::error::other::OtherError;
use crate::types::jobs::job_item::JobItem;
use crate::types::jobs::metadata::{JobMetadata, ProvingInputType, ProvingMetadata};
use crate::types::jobs::status::JobVerificationStatus;
use crate::types::jobs::types::{JobStatus, JobType};
use crate::utils::helpers::JobProcessingState;
use crate::worker::event_handler::jobs::JobHandlerTrait;

pub struct ProvingJobHandler;

#[async_trait]
impl ProvingJobTrait for ProvingJobHandler {
}
