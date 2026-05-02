#[tauri::command]
fn set_click_through(window: tauri::Window, enabled: bool) {
    let _ = window.set_ignore_cursor_events(enabled);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_click_through])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
