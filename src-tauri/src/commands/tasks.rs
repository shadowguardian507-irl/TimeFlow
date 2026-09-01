use chrono::NaiveDate;
use tauri::State;
use uuid::Uuid;

use crate::models::{DistributionStrategy, Task, TaskType, TaskUpdate};
use crate::AppState;

#[tauri::command]
pub async fn create_task(
    state: State<'_, AppState>,
    name: String,
    duration_minutes: u32,
    category_path: String,
    task_type: TaskType,
    distribution_strategy: Option<DistributionStrategy>,
    date: Option<String>,
) -> Result<Task, String> {
    let task_manager = state.task_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    let parsed_date = date
        .map(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| format!("Invalid date format: {}", e))?;

    task_manager
        .create_task(
            name,
            duration_minutes,
            category_path,
            task_type,
            distribution_strategy,
            parsed_date,
            &settings,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task(
    state: State<'_, AppState>,
    task_id: String,
    date: String,
    updates: TaskUpdate,
) -> Result<Task, String> {
    let task_manager = state.task_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    let uuid = Uuid::parse_str(&task_id).map_err(|e| format!("Invalid task ID: {}", e))?;
    let parsed_date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    task_manager
        .update_task(uuid, parsed_date, updates, &settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_task(
    state: State<'_, AppState>,
    task_id: String,
    date: String,
) -> Result<(), String> {
    let task_manager = state.task_manager.lock().unwrap();

    let uuid = Uuid::parse_str(&task_id).map_err(|e| format!("Invalid task ID: {}", e))?;
    let parsed_date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    task_manager
        .delete_task(uuid, parsed_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tasks_for_date(
    state: State<'_, AppState>,
    date: String,
) -> Result<Vec<Task>, String> {
    let task_manager = state.task_manager.lock().unwrap();

    let parsed_date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    task_manager
        .get_tasks_for_date(parsed_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tasks_for_range(
    state: State<'_, AppState>,
    start: String,
    end: String,
) -> Result<Vec<Task>, String> {
    let task_manager = state.task_manager.lock().unwrap();

    let start_date =
        NaiveDate::parse_from_str(&start, "%Y-%m-%d").map_err(|e| format!("Invalid start date: {}", e))?;
    let end_date =
        NaiveDate::parse_from_str(&end, "%Y-%m-%d").map_err(|e| format!("Invalid end date: {}", e))?;

    task_manager
        .get_tasks_for_range(start_date, end_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_daily_time(
    state: State<'_, AppState>,
    date: String,
) -> Result<crate::services::task_manager::DailyValidation, String> {
    let task_manager = state.task_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    let parsed_date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    task_manager
        .validate_daily_time(parsed_date, &settings)
        .map_err(|e| e.to_string())
}

// Timer commands
#[tauri::command]
pub async fn start_timer(
    state: State<'_, AppState>,
    task_name: String,
    category_path: Option<String>,
    task_type: TaskType,
    distribution_strategy: Option<DistributionStrategy>,
) -> Result<crate::models::TimerInfo, String> {
    let timer_service = state.timer_service.lock().unwrap();

    timer_service
        .start(task_name, category_path, task_type, distribution_strategy)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_timer(state: State<'_, AppState>) -> Result<Task, String> {
    let timer_service = state.timer_service.lock().unwrap();
    let task_manager = state.task_manager.lock().unwrap();
    let settings_manager = state.settings_manager.lock().unwrap();
    let settings = settings_manager.get_settings().map_err(|e| e.to_string())?;

    let (timer_state, elapsed_minutes) = timer_service.stop().map_err(|e| e.to_string())?;

    // Create task from timer
    task_manager
        .create_task(
            timer_state.task_name,
            elapsed_minutes,
            timer_state.category_path.unwrap_or_default(),
            timer_state.task_type,
            timer_state.distribution_strategy,
            None,
            &settings,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_timer_state(
    state: State<'_, AppState>,
) -> Result<Option<crate::models::TimerInfo>, String> {
    let timer_service = state.timer_service.lock().unwrap();
    Ok(timer_service.get_state())
}

#[tauri::command]
pub async fn discard_timer(state: State<'_, AppState>) -> Result<(), String> {
    let timer_service = state.timer_service.lock().unwrap();
    timer_service.discard().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn is_timer_running(state: State<'_, AppState>) -> Result<bool, String> {
    let timer_service = state.timer_service.lock().unwrap();
    Ok(timer_service.is_running())
}
