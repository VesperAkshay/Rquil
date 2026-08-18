// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_requests(path: String) -> Result<Vec<String>, String> {
    let files = relay_core::discovery::find_rl_files(std::path::Path::new(&path));
    Ok(files.into_iter().map(|p| p.display().to_string()).collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_requests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
