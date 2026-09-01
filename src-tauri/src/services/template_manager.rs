use chrono::Local;
use uuid::Uuid;

use crate::error::{Result, TimeFlowError};
use crate::models::{
    DistributionStrategy, Settings, Task, TaskOverrides, TaskType, Template, TemplateUpdate,
};
use crate::services::storage::StorageService;

pub struct TemplateManager {
    storage: StorageService,
}

impl TemplateManager {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    /// Create a new template
    pub fn create_template(
        &self,
        name: String,
        default_duration: u32,
        category_path: String,
        is_mergeable: bool,
        distribution_strategy: Option<DistributionStrategy>,
        settings: &Settings,
    ) -> Result<Template> {
        // Validate duration
        if default_duration < settings.minimum_task_duration {
            return Err(TimeFlowError::InvalidDuration(format!(
                "Duration must be at least {} minute(s)",
                settings.minimum_task_duration
            )));
        }

        // Validate mergeable templates have strategy
        if is_mergeable && distribution_strategy.is_none() {
            return Err(TimeFlowError::DistributionStrategyRequired);
        }

        let template = Template::new(
            name,
            default_duration,
            category_path,
            is_mergeable,
            distribution_strategy,
        );

        let mut templates = self.storage.load_templates()?;
        templates.push(template.clone());
        self.storage.save_templates(&templates)?;

        Ok(template)
    }

    /// Update an existing template
    pub fn update_template(
        &self,
        template_id: Uuid,
        updates: TemplateUpdate,
        settings: &Settings,
    ) -> Result<Template> {
        let mut templates = self.storage.load_templates()?;

        let template = templates
            .iter_mut()
            .find(|t| t.id == template_id)
            .ok_or_else(|| TimeFlowError::TemplateNotFound(template_id.to_string()))?;

        // Apply updates
        if let Some(name) = updates.name {
            template.name = name;
        }
        if let Some(duration) = updates.default_duration {
            if duration < settings.minimum_task_duration {
                return Err(TimeFlowError::InvalidDuration(format!(
                    "Duration must be at least {} minute(s)",
                    settings.minimum_task_duration
                )));
            }
            template.default_duration = duration;
        }
        if let Some(category_path) = updates.category_path {
            template.category_path = category_path;
        }
        if let Some(is_mergeable) = updates.is_mergeable {
            template.is_mergeable = is_mergeable;
        }
        if let Some(strategy) = updates.distribution_strategy {
            template.distribution_strategy = Some(strategy);
        }

        // Validate mergeable templates have strategy
        if template.is_mergeable && template.distribution_strategy.is_none() {
            return Err(TimeFlowError::DistributionStrategyRequired);
        }

        let updated_template = template.clone();
        self.storage.save_templates(&templates)?;

        Ok(updated_template)
    }

    /// Delete a template
    pub fn delete_template(&self, template_id: Uuid) -> Result<()> {
        let mut templates = self.storage.load_templates()?;

        let initial_len = templates.len();
        templates.retain(|t| t.id != template_id);

        if templates.len() == initial_len {
            return Err(TimeFlowError::TemplateNotFound(template_id.to_string()));
        }

        self.storage.save_templates(&templates)?;
        Ok(())
    }

    /// Get all templates
    pub fn get_templates(&self) -> Result<Vec<Template>> {
        self.storage.load_templates()
    }

    /// Apply a template to create a new task
    pub fn apply_template(
        &self,
        template_id: Uuid,
        overrides: Option<TaskOverrides>,
    ) -> Result<Task> {
        let templates = self.storage.load_templates()?;

        let template = templates
            .iter()
            .find(|t| t.id == template_id)
            .ok_or_else(|| TimeFlowError::TemplateNotFound(template_id.to_string()))?;

        let overrides = overrides.unwrap_or_default();

        let name = overrides.name.unwrap_or_else(|| template.name.clone());
        let duration = overrides
            .duration_minutes
            .unwrap_or(template.default_duration);
        let category_path = overrides
            .category_path
            .unwrap_or_else(|| template.category_path.clone());
        let is_mergeable = overrides.is_mergeable.unwrap_or(template.is_mergeable);
        let distribution_strategy = overrides
            .distribution_strategy
            .or_else(|| template.distribution_strategy.clone());

        let task_type = if is_mergeable {
            TaskType::Mergeable
        } else {
            TaskType::Direct
        };

        let task = Task::new(
            name,
            Local::now().date_naive(),
            duration,
            category_path,
            task_type,
            distribution_strategy,
        );

        Ok(task)
    }
}
