mod commands;

use commands::send_ble_command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::missing_panics_doc)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_blec::init())
        .invoke_handler(tauri::generate_handler![send_ble_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
