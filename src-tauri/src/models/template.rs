use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::task::DistributionStrategy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub default_duration: u32,
    pub category_path: String,
    pub is_mergeable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_strategy: Option<DistributionStrategy>,
    pub created_at: DateTime<Utc>,
}

impl Template {
    pub fn new(
        name: String,
        default_duration: u32,
        category_path: String,
        is_mergeable: bool,
        distribution_strategy: Option<DistributionStrategy>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            default_duration,
            category_path,
            is_mergeable,
            distribution_strategy,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_strategy: Option<DistributionStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mergeable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_strategy: Option<DistributionStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateList {
    pub templates: Vec<Template>,
}

impl Default for TemplateList {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
        }
    }
}
