#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod xmind;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // macOS 下确保主窗口显示并前置，避免安装后打开无界面
            if let Some(w) = app.get_webview_window("main") {
                let _: Result<(), _> = w.show();
                let _: Result<(), _> = w.set_focus();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            convert_xmind_to_markdown,
            save_markdown_to_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_xmind_to_markdown(path: String) -> Result<String, String> {
    xmind::parse_and_convert(&path)
}

#[tauri::command]
fn save_markdown_to_file(path: String, content: String) -> Result<(), String> {
    xmind::save_markdown(&path, &content)
}
