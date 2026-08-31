use uuid::Uuid;

use crate::error::{Result, TimeFlowError};
use crate::models::{DistributionStrategy, ManualAllocation, Task, WeightedAllocation};

#[derive(Debug, Clone)]
pub struct TimeAllocation {
    pub task_id: Uuid,
    pub task_name: String,
    pub category_path: String,
    pub original_duration: u32,
    pub allocated_duration: u32,
}

#[derive(Debug, Clone)]
pub struct MergeSource {
    pub source_task_id: Uuid,
    pub source_task_name: String,
    pub contributed_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct MergedEntry {
    pub task_id: Uuid,
    pub task_name: String,
    pub category_path: String,
    pub original_duration: u32,
    pub merged_duration: u32,
    pub merged_from: Vec<MergeSource>,
}

pub struct TimeMergeService;

impl TimeMergeService {
    /// Calculate time distribution for all mergeable tasks
    pub fn calculate_distribution(
        mergeable_tasks: &[Task],
        direct_tasks: &[Task],
    ) -> Result<Vec<MergedEntry>> {
        if direct_tasks.is_empty() && !mergeable_tasks.is_empty() {
            return Err(TimeFlowError::NoDirectTasks);
        }

        // Initialize merged entries from direct tasks
        let mut merged_entries: Vec<MergedEntry> = direct_tasks
            .iter()
            .map(|t| MergedEntry {
                task_id: t.id,
                task_name: t.name.clone(),
                category_path: t.category_path.clone(),
                original_duration: t.duration_minutes,
                merged_duration: t.duration_minutes,
                merged_from: Vec::new(),
            })
            .collect();

        // Process each mergeable task
        for mergeable in mergeable_tasks {
            let strategy = mergeable
                .distribution_strategy
                .as_ref()
                .ok_or(TimeFlowError::DistributionStrategyRequired)?;

            let allocations = match strategy {
                DistributionStrategy::Proportional => {
                    Self::proportional_distribute(mergeable.duration_minutes, direct_tasks)
                }
                DistributionStrategy::Even => {
                    Self::even_distribute(mergeable.duration_minutes, direct_tasks)
                }
                DistributionStrategy::Manual => {
                    Self::manual_distribute(
                        mergeable.duration_minutes,
                        &mergeable.manual_allocations,
                    )?
                }
                DistributionStrategy::Weighted => {
                    Self::weighted_distribute(mergeable.duration_minutes, &mergeable.weights)?
                }
            };

            // Apply allocations to merged entries
            for (task_id, minutes) in allocations {
                if let Some(entry) = merged_entries.iter_mut().find(|e| e.task_id == task_id) {
                    entry.merged_duration += minutes;
                    entry.merged_from.push(MergeSource {
                        source_task_id: mergeable.id,
                        source_task_name: mergeable.name.clone(),
                        contributed_minutes: minutes,
                    });
                }
            }
        }

        Ok(merged_entries)
    }

    /// Proportional distribution based on direct task durations
    fn proportional_distribute(mergeable_time: u32, direct_tasks: &[Task]) -> Vec<(Uuid, u32)> {
        let total_direct_time: u32 = direct_tasks.iter().map(|t| t.duration_minutes).sum();

        if total_direct_time == 0 {
            // Fall back to even distribution
            return Self::even_distribute(mergeable_time, direct_tasks);
        }

        let mut allocations = Vec::new();
        let mut allocated: u32 = 0;

        // Sort by duration descending (longest first gets remainder)
        let mut sorted_tasks: Vec<_> = direct_tasks.iter().collect();
        sorted_tasks.sort_by(|a, b| b.duration_minutes.cmp(&a.duration_minutes));

        for (i, task) in sorted_tasks.iter().enumerate() {
            let allocation = if i == sorted_tasks.len() - 1 {
                // Last task gets remainder
                mergeable_time.saturating_sub(allocated)
            } else {
                let ratio = task.duration_minutes as f64 / total_direct_time as f64;
                (mergeable_time as f64 * ratio).floor() as u32
            };

            allocations.push((task.id, allocation));
            allocated += allocation;
        }

        allocations
    }

    /// Even distribution across all direct tasks
    fn even_distribute(mergeable_time: u32, direct_tasks: &[Task]) -> Vec<(Uuid, u32)> {
        if direct_tasks.is_empty() {
            return Vec::new();
        }

        let count = direct_tasks.len() as u32;
        let base_allocation = mergeable_time / count;
        let remainder = mergeable_time % count;

        direct_tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let allocation = base_allocation + if (i as u32) < remainder { 1 } else { 0 };
                (task.id, allocation)
            })
            .collect()
    }

    /// Manual distribution based on user-specified percentages
    fn manual_distribute(
        mergeable_time: u32,
        allocations: &[ManualAllocation],
    ) -> Result<Vec<(Uuid, u32)>> {
        // Validate percentages sum to 100
        let total_percentage: f32 = allocations.iter().map(|a| a.percentage).sum();
        if (total_percentage - 100.0).abs() > 0.01 {
            return Err(TimeFlowError::InvalidAllocation(format!(
                "Manual allocations must sum to 100%, got {}%",
                total_percentage
            )));
        }

        let mut result = Vec::new();
        let mut allocated: u32 = 0;

        for (i, alloc) in allocations.iter().enumerate() {
            let allocation = if i == allocations.len() - 1 {
                mergeable_time.saturating_sub(allocated)
            } else {
                (mergeable_time as f32 * alloc.percentage / 100.0).floor() as u32
            };

            result.push((alloc.target_task_id, allocation));
            allocated += allocation;
        }

        Ok(result)
    }

    /// Weighted distribution based on user-assigned weights
    fn weighted_distribute(
        mergeable_time: u32,
        weights: &[WeightedAllocation],
    ) -> Result<Vec<(Uuid, u32)>> {
        let total_weight: f32 = weights.iter().map(|w| w.weight).sum();

        if total_weight <= 0.0 {
            return Err(TimeFlowError::InvalidAllocation(
                "At least one task must have a positive weight".into(),
            ));
        }

        let mut result = Vec::new();
        let mut allocated: u32 = 0;

        for (i, weight_alloc) in weights.iter().enumerate() {
            let allocation = if i == weights.len() - 1 {
                mergeable_time.saturating_sub(allocated)
            } else {
                let ratio = weight_alloc.weight / total_weight;
                (mergeable_time as f32 * ratio).floor() as u32
            };

            result.push((weight_alloc.target_task_id, allocation));
            allocated += allocation;
        }

        Ok(result)
    }
}
