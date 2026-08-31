use chrono::NaiveDate;
use std::path::PathBuf;
use tauri::State;

use crate::services::export_manager::ImportResult;
use crate::AppState;

#[tauri::command]
pub async fn export_backup(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let export_manager = state.export_manager.lock().unwrap();
    export_manager
        .export_backup(PathBuf::from(path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn import_backup(
    state: State<'_, AppState>,
    path: String,
) -> Result<ImportResult, String> {
    let export_manager = state.export_manager.lock().unwrap();
    export_manager
        .import_backup(PathBuf::from(path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_csv(
    state: State<'_, AppState>,
    start: String,
    end: String,
    path: String,
) -> Result<(), String> {
    let export_manager = state.export_manager.lock().unwrap();

    let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
        .map_err(|e| format!("Invalid start date: {}", e))?;
    let end_date =
        NaiveDate::parse_from_str(&end, "%Y-%m-%d").map_err(|e| format!("Invalid end date: {}", e))?;

    export_manager
        .export_csv(start_date, end_date, PathBuf::from(path))
        .map_err(|e| e.to_string())
}
