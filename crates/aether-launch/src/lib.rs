use std::path::PathBuf;

use aether_core::InstanceId;
use aether_runtime::JavaRuntime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchRequest {
    pub instance_id: InstanceId,
    pub memory_mib: u32,
}

impl LaunchRequest {
    pub fn validate(&self) -> Result<(), LaunchError> {
        if !(512..=131_072).contains(&self.memory_mib) {
            return Err(LaunchError::InvalidMemory(self.memory_mib));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchPlan {
    pub runtime: JavaRuntime,
    pub working_directory: PathBuf,
    pub arguments: Vec<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LaunchError {
    #[error("requested memory {0} MiB is outside the supported range")]
    InvalidMemory(u32),
}
