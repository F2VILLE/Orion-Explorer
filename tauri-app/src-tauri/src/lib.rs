use std::time::SystemTime;

#[derive(serde::Serialize)]
struct FileObject {
    name: String,
    size: u64,
    is_dir: bool,
    created: u64,
    modified: u64,
    thumbnail: String,
    path: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_files(path: String) -> Result<Vec<FileObject>, String> {
    let files = std::fs::read_dir(path).map_err(|e| e.to_string())?;
    let files: Vec<String> = files
        .filter_map(|file: Result<std::fs::DirEntry, std::io::Error>| file.ok())
        .filter_map(|file| file.path().to_str().map(|s| s.to_string()))
        .collect();
    let res: Vec<FileObject> = files.iter().filter(|file| {
        let metadata = std::fs::metadata(file).unwrap();
        metadata.is_dir() || metadata.is_file()
    }).collect::<Vec<_>>()
    .iter()
        .map(|file| {
            let metadata = std::fs::metadata(file).unwrap();
            let created = metadata.created().unwrap();
            let modified = metadata.modified().unwrap();
            let thumbnail = if metadata.is_dir() {
                "folder".to_string()
            } else {
                "file".to_string()
            };
            FileObject {
                name: file.clone().split('/').last().unwrap().to_string(),
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                created: created.elapsed().unwrap().as_millis() as u64,
                modified: modified.elapsed().unwrap().as_millis() as u64,
                thumbnail,
                path: file.clone().to_string(),
            }
        })
        .collect();
    

    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
