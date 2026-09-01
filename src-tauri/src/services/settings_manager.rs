use crate::error::Result;
use crate::models::{Settings, SettingsUpdate};
use crate::services::storage::StorageService;

pub struct SettingsManager {
    storage: StorageService,
}

impl SettingsManager {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    /// Get current settings
    pub fn get_settings(&self) -> Result<Settings> {
        self.storage.load_settings()
    }

    /// Update settings
    pub fn update_settings(&self, updates: SettingsUpdate) -> Result<Settings> {
        let mut settings = self.storage.load_settings()?;

        if let Some(theme) = updates.theme {
            settings.theme = theme;
        }
        if let Some(min_duration) = updates.minimum_task_duration {
            settings.minimum_task_duration = min_duration;
        }
        if let Some(work_hours) = updates.work_day_hours {
            settings.work_day_hours = work_hours;
        }
        if let Some(strategy) = updates.default_distribution_strategy {
            settings.default_distribution_strategy = strategy;
        }

        self.storage.save_settings(&settings)?;
        Ok(settings)
    }

    /// Check if this is the first run
    pub fn is_first_run(&self) -> Result<bool> {
        let settings = self.storage.load_settings()?;
        Ok(!settings.first_run_complete)
    }

    /// Check whether data from the pre-release storage namespace is available.
    pub fn has_legacy_data(&self) -> Result<bool> {
        self.storage.has_legacy_data()
    }

    /// Import data from the pre-release storage namespace.
    pub fn import_legacy_data(&self) -> Result<()> {
        self.storage.import_legacy_data()?;

        let mut settings = self.storage.load_settings()?;
        settings.first_run_complete = true;
        self.storage.save_settings(&settings)
    }

    /// Complete first run setup
    pub fn complete_first_run(&self, initial_settings: Settings) -> Result<Settings> {
        let mut settings = initial_settings;
        settings.first_run_complete = true;
        self.storage.save_settings(&settings)?;
        Ok(settings)
    }
}
