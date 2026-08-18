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

#[tauri::command]
fn load_request(path: String) -> Result<relay_core::model::RelayFile, String> {
    relay_core::parser::parse_file(std::path::Path::new(&path))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_environments(path: String) -> Result<Vec<String>, String> {
    let config_path = std::path::Path::new(&path).join("relay.toml");
    if !config_path.exists() {
        return Ok(Vec::new());
    }
    
    match relay_core::parser::load_collection_config(&config_path) {
        Ok(config) => {
            let mut envs: Vec<String> = config.environments.keys().cloned().collect();
            envs.sort();
            Ok(envs)
        },
        Err(e) => Err(format!("Failed to parse relay.toml: {}", e)),
    }
}

#[tauri::command]
fn get_secrets(path: String) -> Result<std::collections::HashMap<String, String>, String> {
    let secrets_path = std::path::Path::new(&path).join(".relay-secrets.toml");
    if !secrets_path.exists() {
        return Ok(std::collections::HashMap::new());
    }
    
    relay_core::parser::load_secrets(&secrets_path)
        .map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
pub struct SendRequestArgs {
    method: String,
    url: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
}

#[derive(serde::Serialize)]
pub struct SendResponse {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
    time_ms: u128,
}

#[tauri::command]
async fn send_request(req: SendRequestArgs) -> Result<SendResponse, String> {
    let request = relay_core::model::Request {
        method: req.method,
        url: req.url,
        headers: req.headers,
        body: req.body.map(|content| relay_core::model::RequestBody {
            body_type: "raw".to_string(),
            content,
        }),
        auth: None,
    };
    
    let client = reqwest::Client::new();
    let response = relay_core::http::execute(&client, &request)
        .await
        .map_err(|e| e.to_string())?;
        
    Ok(SendResponse {
        status: response.status,
        headers: response.headers,
        body: response.body,
        time_ms: response.time_ms,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_requests, load_request, get_environments, get_secrets, send_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
