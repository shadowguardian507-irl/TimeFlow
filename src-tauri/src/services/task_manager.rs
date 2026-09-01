use chrono::{Local, NaiveDate, Utc};
use uuid::Uuid;

use crate::error::{Result, TimeFlowError};
use crate::models::{DistributionStrategy, Settings, Task, TaskType, TaskUpdate};
use crate::services::storage::StorageService;

pub struct TaskManager {
    storage: StorageService,
}

impl TaskManager {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    /// Create a new task
    pub fn create_task(
        &self,
        name: String,
        duration_minutes: u32,
        category_path: String,
        task_type: TaskType,
        distribution_strategy: Option<DistributionStrategy>,
        date: Option<NaiveDate>,
        settings: &Settings,
    ) -> Result<Task> {
        // Validate duration
        if duration_minutes < settings.minimum_task_duration {
            return Err(TimeFlowError::InvalidDuration(format!(
                "Duration must be at least {} minute(s)",
                settings.minimum_task_duration
            )));
        }

        // Validate mergeable tasks have strategy
        if task_type == TaskType::Mergeable && distribution_strategy.is_none() {
            return Err(TimeFlowError::DistributionStrategyRequired);
        }

        let task_date = date.unwrap_or_else(|| Local::now().date_naive());
        
        let task = Task::new(
            name,
            task_date,
            duration_minutes,
            category_path,
            task_type,
            distribution_strategy,
        );

        // Load existing tasks and add new one
        let mut tasks = self.storage.load_tasks(task_date)?;
        tasks.push(task.clone());
        self.storage.save_tasks(task_date, tasks)?;

        Ok(task)
    }

    /// Update an existing task
    pub fn update_task(
        &self,
        task_id: Uuid,
        date: NaiveDate,
        updates: TaskUpdate,
        settings: &Settings,
    ) -> Result<Task> {
        let mut tasks = self.storage.load_tasks(date)?;
        
        let task = tasks
            .iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| TimeFlowError::TaskNotFound(task_id.to_string()))?;

        // Apply updates
        if let Some(name) = updates.name {
            task.name = name;
        }
        if let Some(duration) = updates.duration_minutes {
            if duration < settings.minimum_task_duration {
                return Err(TimeFlowError::InvalidDuration(format!(
                    "Duration must be at least {} minute(s)",
                    settings.minimum_task_duration
                )));
            }
            task.duration_minutes = duration;
        }
        if let Some(category_path) = updates.category_path {
            task.category_path = category_path;
        }
        if let Some(task_type) = updates.task_type {
            task.task_type = task_type;
        }
        if let Some(strategy) = updates.distribution_strategy {
            task.distribution_strategy = Some(strategy);
        }
        if let Some(allocations) = updates.manual_allocations {
            task.manual_allocations = allocations;
        }
        if let Some(weights) = updates.weights {
            task.weights = weights;
        }

        // Validate mergeable tasks have strategy
        if task.task_type == TaskType::Mergeable && task.distribution_strategy.is_none() {
            return Err(TimeFlowError::DistributionStrategyRequired);
        }

        task.updated_at = Utc::now();

        let updated_task = task.clone();
        self.storage.save_tasks(date, tasks)?;

        Ok(updated_task)
    }

    /// Delete a task
    pub fn delete_task(&self, task_id: Uuid, date: NaiveDate) -> Result<()> {
        let mut tasks = self.storage.load_tasks(date)?;
        
        let initial_len = tasks.len();
        tasks.retain(|t| t.id != task_id);

        if tasks.len() == initial_len {
            return Err(TimeFlowError::TaskNotFound(task_id.to_string()));
        }

        self.storage.save_tasks(date, tasks)?;
        Ok(())
    }

    /// Get tasks for a specific date
    pub fn get_tasks_for_date(&self, date: NaiveDate) -> Result<Vec<Task>> {
        self.storage.load_tasks(date)
    }

    /// Get tasks for a date range
    pub fn get_tasks_for_range(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Task>> {
        self.storage.load_tasks_range(start, end)
    }

    /// Validate daily time totals
    pub fn validate_daily_time(&self, date: NaiveDate, settings: &Settings) -> Result<DailyValidation> {
        let tasks = self.storage.load_tasks(date)?;
        let total_minutes: u32 = tasks.iter().map(|t| t.duration_minutes).sum();
        let total_hours = total_minutes as f32 / 60.0;

        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        if total_hours > 24.0 {
            errors.push("Total time exceeds 24 hours - please check entries".to_string());
        } else if total_hours > settings.work_day_hours {
            warnings.push(format!(
                "Total time ({:.1}h) exceeds expected work day ({:.1}h)",
                total_hours, settings.work_day_hours
            ));
        }

        Ok(DailyValidation {
            total_minutes,
            total_hours,
            warnings,
            errors,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DailyValidation {
    pub total_minutes: u32,
    pub total_hours: f32,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
