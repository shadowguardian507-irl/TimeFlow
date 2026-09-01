use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::task::{DistributionStrategy, TaskType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub task_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_path: Option<String>,
    pub task_type: TaskType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_strategy: Option<DistributionStrategy>,
    pub start_time: DateTime<Utc>,
    pub is_running: bool,
}

impl TimerState {
    pub fn new(
        task_name: String,
        category_path: Option<String>,
        task_type: TaskType,
        distribution_strategy: Option<DistributionStrategy>,
    ) -> Self {
        Self {
            task_name,
            category_path,
            task_type,
            distribution_strategy,
            start_time: Utc::now(),
            is_running: true,
        }
    }

    /// Calculate elapsed time in minutes
    pub fn elapsed_minutes(&self) -> u32 {
        let elapsed = Utc::now() - self.start_time;
        elapsed.num_minutes().max(0) as u32
    }

    /// Calculate elapsed time in seconds (for display)
    pub fn elapsed_seconds(&self) -> u64 {
        let elapsed = Utc::now() - self.start_time;
        elapsed.num_seconds().max(0) as u64
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerInfo {
    pub task_name: String,
    pub category_path: Option<String>,
    pub task_type: TaskType,
    pub distribution_strategy: Option<DistributionStrategy>,
    pub elapsed_seconds: u64,
    pub is_running: bool,
}

impl From<&TimerState> for TimerInfo {
    fn from(state: &TimerState) -> Self {
        Self {
            task_name: state.task_name.clone(),
            category_path: state.category_path.clone(),
            task_type: state.task_type.clone(),
            distribution_strategy: state.distribution_strategy.clone(),
            elapsed_seconds: state.elapsed_seconds(),
            is_running: state.is_running,
        }
    }
}
