use chrono::NaiveDate;
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, TimeFlowError};
use crate::models::{Category, DayTasks, Settings, Task, TemplateList, TimerState};

pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new() -> Result<Self> {
        let data_dir = data_dir_for("uk", "etheria-software", "TimeFlow")?;

        // Create directories if they don't exist
        fs::create_dir_all(&data_dir)?;
        fs::create_dir_all(data_dir.join("tasks"))?;

        Ok(Self { data_dir })
    }

    /// Returns whether data from the pre-release storage namespace can be imported.
    pub fn has_legacy_data(&self) -> Result<bool> {
        let legacy_dir = legacy_data_dir()?;

        Ok(legacy_dir != self.data_dir
            && has_user_data(&legacy_dir)
            && !has_user_data(&self.data_dir))
    }

    /// Copy data from the pre-release storage namespace into the current namespace.
    pub fn import_legacy_data(&self) -> Result<()> {
        let legacy_dir = legacy_data_dir()?;

        if legacy_dir == self.data_dir {
            return Err(TimeFlowError::ImportError(
                "Legacy and current data directories are the same".into(),
            ));
        }

        if !has_user_data(&legacy_dir) {
            return Err(TimeFlowError::ImportError(
                "No legacy TimeFlow data was found".into(),
            ));
        }

        if has_user_data(&self.data_dir) {
            return Err(TimeFlowError::ImportError(
                "Current TimeFlow data already exists".into(),
            ));
        }

        copy_directory_contents(&legacy_dir, &self.data_dir)?;
        Ok(())
    }

    /// Get the path for a specific date's tasks file
    fn tasks_path(&self, date: NaiveDate) -> PathBuf {
        self.data_dir
            .join("tasks")
            .join(format!("{}.yaml", date.format("%Y-%m-%d")))
    }

    /// Get the path for categories file
    fn categories_path(&self) -> PathBuf {
        self.data_dir.join("categories.yaml")
    }

    /// Get the path for templates file
    fn templates_path(&self) -> PathBuf {
        self.data_dir.join("templates.yaml")
    }

    /// Get the path for settings file
    fn settings_path(&self) -> PathBuf {
        self.data_dir.join("settings.yaml")
    }

    /// Get the path for timer state file (temp)
    fn timer_state_path(&self) -> PathBuf {
        self.data_dir.join("timer_state.yaml")
    }

    /// Atomic write - write to temp file then rename
    fn atomic_write(&self, path: &PathBuf, content: &str) -> Result<()> {
        let temp_path = path.with_extension("yaml.tmp");
        fs::write(&temp_path, content)?;
        fs::rename(&temp_path, path)?;
        Ok(())
    }

    // ==================== Tasks ====================

    pub fn load_tasks(&self, date: NaiveDate) -> Result<Vec<Task>> {
        let path = self.tasks_path(date);

        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)?;
        let day_tasks: DayTasks = serde_yaml::from_str(&content)?;
        Ok(day_tasks.tasks)
    }

    pub fn save_tasks(&self, date: NaiveDate, tasks: Vec<Task>) -> Result<()> {
        let path = self.tasks_path(date);
        let day_tasks = DayTasks { date, tasks };
        let content = serde_yaml::to_string(&day_tasks)?;
        self.atomic_write(&path, &content)
    }

    pub fn load_tasks_range(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        let mut current = start;

        while current <= end {
            let tasks = self.load_tasks(current)?;
            all_tasks.extend(tasks);
            current = current.succ_opt().unwrap_or(current);
        }

        Ok(all_tasks)
    }

    // ==================== Categories ====================

    pub fn load_categories(&self) -> Result<Category> {
        let path = self.categories_path();

        if !path.exists() {
            return Ok(crate::models::category::create_root());
        }

        let content = fs::read_to_string(&path)?;
        let categories: Category = serde_yaml::from_str(&content)?;
        Ok(categories)
    }

    pub fn save_categories(&self, categories: &Category) -> Result<()> {
        let path = self.categories_path();
        let content = serde_yaml::to_string(categories)?;
        self.atomic_write(&path, &content)
    }

    // ==================== Templates ====================

    pub fn load_templates(&self) -> Result<Vec<crate::models::Template>> {
        let path = self.templates_path();

        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path)?;
        let template_list: TemplateList = serde_yaml::from_str(&content)?;
        Ok(template_list.templates)
    }

    pub fn save_templates(&self, templates: &[crate::models::Template]) -> Result<()> {
        let path = self.templates_path();
        let template_list = TemplateList {
            templates: templates.to_vec(),
        };
        let content = serde_yaml::to_string(&template_list)?;
        self.atomic_write(&path, &content)
    }

    // ==================== Settings ====================

    pub fn load_settings(&self) -> Result<Settings> {
        let path = self.settings_path();

        if !path.exists() {
            return Ok(Settings::default());
        }

        let content = fs::read_to_string(&path)?;
        let settings: Settings = serde_yaml::from_str(&content)?;
        Ok(settings)
    }

    pub fn save_settings(&self, settings: &Settings) -> Result<()> {
        let path = self.settings_path();
        let content = serde_yaml::to_string(settings)?;
        self.atomic_write(&path, &content)
    }

    // ==================== Timer State ====================

    pub fn load_timer_state(&self) -> Result<Option<TimerState>> {
        let path = self.timer_state_path();

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)?;
        let state: TimerState = serde_yaml::from_str(&content)?;
        Ok(Some(state))
    }

    pub fn save_timer_state(&self, state: &TimerState) -> Result<()> {
        let path = self.timer_state_path();
        let content = serde_yaml::to_string(state)?;
        self.atomic_write(&path, &content)
    }

    pub fn clear_timer_state(&self) -> Result<()> {
        let path = self.timer_state_path();
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    // ==================== Export/Import ====================

    pub fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn export_all_data(&self) -> Result<String> {
        #[derive(serde::Serialize)]
        struct ExportData {
            categories: Category,
            templates: Vec<crate::models::Template>,
            settings: Settings,
            // Note: Tasks are stored per-day, would need to iterate all files
        }

        let data = ExportData {
            categories: self.load_categories()?,
            templates: self.load_templates()?,
            settings: self.load_settings()?,
        };

        Ok(serde_yaml::to_string(&data)?)
    }
}

fn data_dir_for(qualifier: &str, organization: &str, application: &str) -> Result<PathBuf> {
    ProjectDirs::from(qualifier, organization, application)
        .map(|project_dirs| project_dirs.data_dir().to_path_buf())
        .ok_or_else(|| TimeFlowError::StorageError("Could not determine data directory".into()))
}

fn legacy_data_dir() -> Result<PathBuf> {
    data_dir_for("com", "timeflow", "TimeFlow")
}

fn has_user_data(data_dir: &Path) -> bool {
    [
        "categories.yaml",
        "templates.yaml",
        "settings.yaml",
        "timer_state.yaml",
    ]
    .iter()
    .any(|file| data_dir.join(file).is_file())
        || contains_files(&data_dir.join("tasks"))
}

fn contains_files(directory: &Path) -> bool {
    let Ok(entries) = fs::read_dir(directory) else {
        return false;
    };

    entries.flatten().any(|entry| {
        let path = entry.path();
        match entry.file_type() {
            Ok(file_type) if file_type.is_file() => true,
            Ok(file_type) if file_type.is_dir() => contains_files(&path),
            _ => false,
        }
    })
}

fn copy_directory_contents(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_directory_contents(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new().expect("Failed to initialize storage service")
    }
}

#[cfg(test)]
mod tests {
    use super::{contains_files, copy_directory_contents};
    use std::fs;

    #[test]
    fn copies_nested_legacy_data_without_changing_file_contents() {
        let root =
            std::env::temp_dir().join(format!("timeflow-storage-test-{}", uuid::Uuid::new_v4()));
        let source = root.join("legacy");
        let destination = root.join("current");

        fs::create_dir_all(source.join("tasks")).unwrap();
        fs::write(source.join("settings.yaml"), "first_run_complete: true\n").unwrap();
        fs::write(source.join("tasks/2026-08-31.yaml"), "tasks: []\n").unwrap();

        copy_directory_contents(&source, &destination).unwrap();

        assert!(contains_files(&destination));
        assert_eq!(
            fs::read_to_string(destination.join("settings.yaml")).unwrap(),
            "first_run_complete: true\n"
        );
        assert_eq!(
            fs::read_to_string(destination.join("tasks/2026-08-31.yaml")).unwrap(),
            "tasks: []\n"
        );

        fs::remove_dir_all(root).unwrap();
    }
}
