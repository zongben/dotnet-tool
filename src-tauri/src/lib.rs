use tauri::generate_handler;
use tools::stringbuilder;

mod tools;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![
            stringbuilder::convert_text,
            stringbuilder::revert_stringbuilder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
