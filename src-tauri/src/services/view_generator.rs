use chrono::NaiveDate;
use serde::Serialize;
use std::collections::HashMap;

use crate::error::{Result, TimeFlowError};
use crate::models::Task;
use crate::services::storage::StorageService;
use crate::services::time_merge::{MergedEntry, TimeMergeService};

#[derive(Debug, Clone, Serialize)]
pub struct FullView {
    pub date: NaiveDate,
    pub tasks: Vec<Task>,
    pub total_minutes: u32,
    pub direct_count: usize,
    pub mergeable_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActiTimeEntry {
    pub category_path: String,
    pub duration_minutes: u32,
    pub task_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActiTimeView {
    pub date: NaiveDate,
    pub entries: Vec<ActiTimeEntry>,
    pub total_minutes: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DaySummary {
    pub date: NaiveDate,
    pub total_minutes: u32,
    pub task_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeekView {
    pub week_start: NaiveDate,
    pub days: Vec<DaySummary>,
    pub total_minutes: u32,
}

pub struct ViewGenerator {
    storage: StorageService,
}

impl ViewGenerator {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    /// Generate full view showing all tasks
    pub fn get_full_view(&self, date: NaiveDate) -> Result<FullView> {
        let tasks = self.storage.load_tasks(date)?;
        let total_minutes: u32 = tasks.iter().map(|t| t.duration_minutes).sum();
        let direct_count = tasks.iter().filter(|t| t.is_direct()).count();
        let mergeable_count = tasks.iter().filter(|t| t.is_mergeable()).count();

        Ok(FullView {
            date,
            tasks,
            total_minutes,
            direct_count,
            mergeable_count,
        })
    }

    /// Generate ActiTime-tuned view with merged times
    pub fn get_actitime_view(&self, date: NaiveDate) -> Result<ActiTimeView> {
        let tasks = self.storage.load_tasks(date)?;
        
        let direct_tasks: Vec<_> = tasks.iter().filter(|t| t.is_direct()).cloned().collect();
        let mergeable_tasks: Vec<_> = tasks.iter().filter(|t| t.is_mergeable()).cloned().collect();

        // Check for no direct tasks with mergeable tasks
        if direct_tasks.is_empty() && !mergeable_tasks.is_empty() {
            return Err(TimeFlowError::NoDirectTasks);
        }

        // Calculate merged entries
        let merged_entries = TimeMergeService::calculate_distribution(&mergeable_tasks, &direct_tasks)?;

        // Aggregate by category
        let entries = Self::aggregate_by_category(merged_entries);
        let total_minutes: u32 = entries.iter().map(|e| e.duration_minutes).sum();

        Ok(ActiTimeView {
            date,
            entries,
            total_minutes,
        })
    }

    /// Aggregate merged entries by category path
    fn aggregate_by_category(entries: Vec<MergedEntry>) -> Vec<ActiTimeEntry> {
        let mut by_category: HashMap<String, ActiTimeEntry> = HashMap::new();

        for entry in entries {
            by_category
                .entry(entry.category_path.clone())
                .and_modify(|e| {
                    e.duration_minutes += entry.merged_duration;
                    if !e.task_names.contains(&entry.task_name) {
                        e.task_names.push(entry.task_name.clone());
                    }
                })
                .or_insert(ActiTimeEntry {
                    category_path: entry.category_path,
                    duration_minutes: entry.merged_duration,
                    task_names: vec![entry.task_name],
                });
        }

        let mut result: Vec<_> = by_category.into_values().collect();
        result.sort_by(|a, b| a.category_path.cmp(&b.category_path));
        result
    }

    /// Generate week view with daily summaries
    pub fn get_week_view(&self, week_start: NaiveDate) -> Result<WeekView> {
        let mut days = Vec::new();
        let mut total_minutes = 0u32;
        let mut current = week_start;

        for _ in 0..7 {
            let tasks = self.storage.load_tasks(current)?;
            let day_total: u32 = tasks.iter().map(|t| t.duration_minutes).sum();
            
            days.push(DaySummary {
                date: current,
                total_minutes: day_total,
                task_count: tasks.len(),
            });

            total_minutes += day_total;
            current = current.succ_opt().unwrap_or(current);
        }

        Ok(WeekView {
            week_start,
            days,
            total_minutes,
        })
    }
}
