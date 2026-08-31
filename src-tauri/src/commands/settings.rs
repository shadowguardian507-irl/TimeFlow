use tauri::State;

use crate::models::{Settings, SettingsUpdate};
use crate::AppState;

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    let settings_manager = state.settings_manager.lock().unwrap();
    settings_manager.get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    updates: SettingsUpdate,
) -> Result<Settings, String> {
    let settings_manager = state.settings_manager.lock().unwrap();
    settings_manager
        .update_settings(updates)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn is_first_run(state: State<'_, AppState>) -> Result<bool, String> {
    let settings_manager = state.settings_manager.lock().unwrap();
    settings_manager.is_first_run().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn complete_first_run(
    state: State<'_, AppState>,
    initial_settings: Settings,
) -> Result<Settings, String> {
    let settings_manager = state.settings_manager.lock().unwrap();
    settings_manager
        .complete_first_run(initial_settings)
        .map_err(|e| e.to_string())
}
