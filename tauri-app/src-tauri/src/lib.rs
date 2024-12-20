// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_files() -> Result<Vec<String>, String> {
    let files = std::fs::read_dir("/").map_err(|e| e.to_string())?;
    let files = files
        .filter_map(|file| file.ok())
        .filter_map(|file| file.path().to_str().map(|s| s.to_string()))
        .collect();
    Ok(files)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
