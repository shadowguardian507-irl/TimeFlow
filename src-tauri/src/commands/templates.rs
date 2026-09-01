use tauri::State;
use uuid::Uuid;

use crate::models::{DistributionStrategy, Task, TaskOverrides, Template, TemplateUpdate};
use crate::AppState;

#[tauri::command]
pub async fn create_template(
    state: State<'_, AppState>,
    name: String,
    default_duration: u32,
    category_path: String,
    is_mergeable: bool,
    distribution_strategy: Option<DistributionStrategy>,
) -> Result<Template, String> {
    let template_manager = state.template_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    template_manager
        .create_template(
            name,
            default_duration,
            category_path,
            is_mergeable,
            distribution_strategy,
            &settings,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_template(
    state: State<'_, AppState>,
    template_id: String,
    updates: TemplateUpdate,
) -> Result<Template, String> {
    let template_manager = state.template_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    let uuid = Uuid::parse_str(&template_id).map_err(|e| format!("Invalid template ID: {}", e))?;

    template_manager
        .update_template(uuid, updates, &settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_template(state: State<'_, AppState>, template_id: String) -> Result<(), String> {
    let template_manager = state.template_manager.lock().unwrap();

    let uuid = Uuid::parse_str(&template_id).map_err(|e| format!("Invalid template ID: {}", e))?;

    template_manager
        .delete_template(uuid)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_templates(state: State<'_, AppState>) -> Result<Vec<Template>, String> {
    let template_manager = state.template_manager.lock().unwrap();
    template_manager.get_templates().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn apply_template(
    state: State<'_, AppState>,
    template_id: String,
    overrides: Option<TaskOverrides>,
) -> Result<Task, String> {
    let template_manager = state.template_manager.lock().unwrap();
    let task_manager = state.task_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    let uuid = Uuid::parse_str(&template_id).map_err(|e| format!("Invalid template ID: {}", e))?;

    // Get task from template
    let task = template_manager
        .apply_template(uuid, overrides)
        .map_err(|e| e.to_string())?;

    // Save the task
    task_manager
        .create_task(
            task.name,
            task.duration_minutes,
            task.category_path,
            task.task_type,
            task.distribution_strategy,
            Some(task.date),
            &settings,
        )
        .map_err(|e| e.to_string())
}
