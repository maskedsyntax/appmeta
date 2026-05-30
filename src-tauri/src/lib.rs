mod error;
mod scanner;
mod project;
mod storage;
mod ai;
mod export;
mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            pick_project_folder,
            pick_save_folder,
            scan_project_cmd,
            create_project_from_scan_cmd,
            refresh_project_from_scan_cmd,
            save_project_cmd,
            load_project_cmd,
            list_projects_cmd,
            delete_project_cmd,
            update_fact_cmd,
            answer_question_cmd,
            load_settings_cmd,
            save_settings_cmd,
            mask_api_key_cmd,
            generate_field_cmd,
            test_ai_connection_cmd,
            validate_project_cmd,
            export_markdown_cmd,
            export_json_cmd,
            save_submission_pack_cmd,
            get_ai_context_preview,
            update_generated_fields_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
