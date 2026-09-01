pub mod commands;
pub mod error;
pub mod models;
pub mod services;

use std::sync::Mutex;

use services::{
    CategoryManager, ExportManager, SettingsManager, StorageService, TaskManager, TemplateManager,
    TimerService, ViewGenerator,
};

pub struct AppState {
    pub task_manager: Mutex<TaskManager>,
    pub category_manager: Mutex<CategoryManager>,
    pub template_manager: Mutex<TemplateManager>,
    pub view_generator: Mutex<ViewGenerator>,
    pub export_manager: Mutex<ExportManager>,
    pub settings_manager: Mutex<SettingsManager>,
    pub timer_service: Mutex<TimerService>,
}

impl AppState {
    pub fn new() -> Result<Self, error::TimeFlowError> {
        let storage = StorageService::new()?;

        Ok(Self {
            task_manager: Mutex::new(TaskManager::new(StorageService::new()?)),
            category_manager: Mutex::new(CategoryManager::new(StorageService::new()?)),
            template_manager: Mutex::new(TemplateManager::new(StorageService::new()?)),
            view_generator: Mutex::new(ViewGenerator::new(StorageService::new()?)),
            export_manager: Mutex::new(ExportManager::new(StorageService::new()?)),
            settings_manager: Mutex::new(SettingsManager::new(StorageService::new()?)),
            timer_service: Mutex::new(TimerService::new(storage)?),
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new().expect("Failed to initialize app state")
    }
}
