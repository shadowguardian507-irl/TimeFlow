// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use timeflow_lib::commands;
use timeflow_lib::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // Task commands
            commands::create_task,
            commands::update_task,
            commands::delete_task,
            commands::get_tasks_for_date,
            commands::get_tasks_for_range,
            commands::validate_daily_time,
            // Timer commands
            commands::start_timer,
            commands::stop_timer,
            commands::get_timer_state,
            commands::discard_timer,
            commands::is_timer_running,
            // Category commands
            commands::get_category_tree,
            commands::get_visible_category_paths,
            commands::add_category,
            commands::hide_category,
            commands::unhide_category,
            commands::validate_category_path,
            // Template commands
            commands::create_template,
            commands::update_template,
            commands::delete_template,
            commands::get_templates,
            commands::apply_template,
            // View commands
            commands::get_full_view,
            commands::get_actitime_view,
            commands::get_week_view,
            // Settings commands
            commands::get_settings,
            commands::update_settings,
            commands::is_first_run,
            commands::complete_first_run,
            // Export commands
            commands::export_backup,
            commands::import_backup,
            commands::export_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
