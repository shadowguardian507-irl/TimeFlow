use tauri::State;

use crate::models::Category;
use crate::AppState;

#[tauri::command]
pub async fn get_category_tree(state: State<'_, AppState>) -> Result<Category, String> {
    let category_manager = state.category_manager.lock().unwrap();
    category_manager
        .get_category_tree()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_visible_category_paths(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let category_manager = state.category_manager.lock().unwrap();
    category_manager
        .get_visible_paths()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_category(state: State<'_, AppState>, path: String) -> Result<Category, String> {
    let category_manager = state.category_manager.lock().unwrap();
    category_manager
        .add_category(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn hide_category(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let category_manager = state.category_manager.lock().unwrap();
    category_manager
        .hide_category(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn unhide_category(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let category_manager = state.category_manager.lock().unwrap();
    category_manager
        .unhide_category(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_category_path(
    state: State<'_, AppState>,
    path: String,
) -> Result<bool, String> {
    let category_manager = state.category_manager.lock().unwrap();
    category_manager
        .validate_category_path(&path)
        .map_err(|e| e.to_string())
}
