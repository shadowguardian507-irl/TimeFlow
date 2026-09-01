use chrono::NaiveDate;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

use crate::error::{Result, TimeFlowError};
use crate::models::{Category, DayTasks, Settings, Task, TemplateList, TimerState};

pub struct StorageService {
    data_dir: PathBuf,
}

impl StorageService {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "timeflow", "TimeFlow")
            .ok_or_else(|| TimeFlowError::StorageError("Could not determine data directory".into()))?;
        
        let data_dir = project_dirs.data_dir().to_path_buf();
        
        // Create directories if they don't exist
        fs::create_dir_all(&data_dir)?;
        fs::create_dir_all(data_dir.join("tasks"))?;
        
        Ok(Self { data_dir })
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

impl Default for StorageService {
    fn default() -> Self {
        Self::new().expect("Failed to initialize storage service")
    }
}
