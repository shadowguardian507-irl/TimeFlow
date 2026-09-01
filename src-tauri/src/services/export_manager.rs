use chrono::NaiveDate;
use std::fs;
use std::path::PathBuf;

use crate::error::{Result, TimeFlowError};
use crate::models::{Category, Settings, Task, Template};
use crate::services::storage::StorageService;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupData {
    pub version: String,
    pub categories: Category,
    pub templates: Vec<Template>,
    pub settings: Settings,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ImportResult {
    pub categories_imported: bool,
    pub templates_count: usize,
    pub settings_imported: bool,
}

pub struct ExportManager {
    storage: StorageService,
}

impl ExportManager {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    /// Export all data to a backup file
    pub fn export_backup(&self, path: PathBuf) -> Result<()> {
        let backup = BackupData {
            version: "1.0.0".to_string(),
            categories: self.storage.load_categories()?,
            templates: self.storage.load_templates()?,
            settings: self.storage.load_settings()?,
        };

        let content = serde_yaml::to_string(&backup)?;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Import data from a backup file
    pub fn import_backup(&self, path: PathBuf) -> Result<ImportResult> {
        let content = fs::read_to_string(&path)
            .map_err(|e| TimeFlowError::ImportError(e.to_string()))?;

        let backup: BackupData = serde_yaml::from_str(&content)
            .map_err(|e| TimeFlowError::ImportError(e.to_string()))?;

        // Import categories
        self.storage.save_categories(&backup.categories)?;

        // Import templates
        self.storage.save_templates(&backup.templates)?;

        // Import settings (preserve first_run_complete)
        let mut settings = backup.settings;
        settings.first_run_complete = true; // Don't show first run after import
        self.storage.save_settings(&settings)?;

        Ok(ImportResult {
            categories_imported: true,
            templates_count: backup.templates.len(),
            settings_imported: true,
        })
    }

    /// Export time data to CSV format
    pub fn export_csv(&self, start: NaiveDate, end: NaiveDate, path: PathBuf) -> Result<()> {
        let tasks = self.storage.load_tasks_range(start, end)?;

        let mut csv_content = String::from("Date,Task Name,Category,Duration (minutes),Type\n");

        for task in tasks {
            let task_type = if task.is_direct() { "Direct" } else { "Mergeable" };
            csv_content.push_str(&format!(
                "{},{},{},{},{}\n",
                task.date,
                Self::escape_csv(&task.name),
                Self::escape_csv(&task.category_path),
                task.duration_minutes,
                task_type
            ));
        }

        fs::write(&path, csv_content)?;
        Ok(())
    }

    /// Escape a string for CSV (handle commas and quotes)
    fn escape_csv(s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }

    /// Get tasks for CSV export preview
    pub fn get_export_preview(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Task>> {
        self.storage.load_tasks_range(start, end)
    }
}
