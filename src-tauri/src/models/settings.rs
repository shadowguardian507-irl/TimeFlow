use serde::{Deserialize, Serialize};

use super::task::DistributionStrategy;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub first_run_complete: bool,
    pub minimum_task_duration: u32,
    pub work_day_hours: f32,
    pub default_distribution_strategy: DistributionStrategy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            first_run_complete: false,
            minimum_task_duration: 1,
            work_day_hours: 7.5,
            default_distribution_strategy: DistributionStrategy::Proportional,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SettingsUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_task_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_day_hours: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_distribution_strategy: Option<DistributionStrategy>,
}
