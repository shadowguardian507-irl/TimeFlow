use chrono::NaiveDate;
use tauri::State;

use crate::services::view_generator::{ActiTimeView, FullView, WeekView};
use crate::AppState;

#[tauri::command]
pub async fn get_full_view(state: State<'_, AppState>, date: String) -> Result<FullView, String> {
    let view_generator = state.view_generator.lock().unwrap();

    let parsed_date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    view_generator
        .get_full_view(parsed_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_actitime_view(
    state: State<'_, AppState>,
    date: String,
) -> Result<ActiTimeView, String> {
    let view_generator = state.view_generator.lock().unwrap();

    let parsed_date =
        NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| format!("Invalid date: {}", e))?;

    view_generator
        .get_actitime_view(parsed_date)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_week_view(
    state: State<'_, AppState>,
    week_start: String,
) -> Result<WeekView, String> {
    let view_generator = state.view_generator.lock().unwrap();

    let parsed_date = NaiveDate::parse_from_str(&week_start, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date: {}", e))?;

    view_generator
        .get_week_view(parsed_date)
        .map_err(|e| e.to_string())
}
