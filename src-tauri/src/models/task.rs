use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Direct,
    Mergeable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DistributionStrategy {
    Proportional,
    Even,
    Manual,
    Weighted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualAllocation {
    pub target_task_id: Uuid,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedAllocation {
    pub target_task_id: Uuid,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub duration_minutes: u32,
    pub category_path: String,
    pub task_type: TaskType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_strategy: Option<DistributionStrategy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manual_allocations: Vec<ManualAllocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weights: Vec<WeightedAllocation>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        name: String,
        date: NaiveDate,
        duration_minutes: u32,
        category_path: String,
        task_type: TaskType,
        distribution_strategy: Option<DistributionStrategy>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            date,
            duration_minutes,
            category_path,
            task_type,
            distribution_strategy,
            manual_allocations: Vec::new(),
            weights: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_direct(&self) -> bool {
        self.task_type == TaskType::Direct
    }

    pub fn is_mergeable(&self) -> bool {
        self.task_type == TaskType::Mergeable
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<TaskType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_strategy: Option<DistributionStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_allocations: Option<Vec<ManualAllocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<WeightedAllocation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayTasks {
    pub date: NaiveDate,
    pub tasks: Vec<Task>,
}
