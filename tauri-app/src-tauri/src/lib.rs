use ntfs_reader::{file, file_info::FileInfo, mft::Mft, volume::Volume};
use rusqlite::{params, Connection};
use tauri::{AppHandle, Emitter};
#[derive(serde::Serialize, Debug)]
struct FileObject {
    id: i64,
    name: String,
    size: i64,
    is_dir: bool,
    created: String,
    modified: String,
    accessed: String,
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
    let res: Vec<FileObject> = files
        .iter()
        .filter(|file| {
            let metadata = std::fs::metadata(file).unwrap();
            metadata.is_dir() || metadata.is_file()
        })
        .collect::<Vec<_>>()
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
                id: 0,
                name: file.clone().split('/').last().unwrap().to_string(),
                size: metadata.len() as i64,
                is_dir: metadata.is_dir(),
                created: created.elapsed().unwrap().as_millis().to_string(),
                modified: modified.elapsed().unwrap().as_millis().to_string(),
                accessed: metadata.accessed().unwrap().elapsed().unwrap().as_millis().to_string(),
                thumbnail,
                path: file.clone().to_string(),
            }
        })
        .collect();

    Ok(res)
}

#[tauri::command]
async fn print_ntfs(app: AppHandle) -> bool {
    app.emit("scanning-log", "Scanning NTFS filesystem").unwrap();

    let volume = Volume::new("\\\\.\\C:").unwrap();
    let mft = Mft::new(volume).unwrap();

    let mut db = get_connection().unwrap();
    // Créer la table si elle n'existe pas
    db.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            size INTEGER NOT NULL,
            isdir BOOLEAN NOT NULL,
            created TEXT NOT NULL,
            modified TEXT NOT NULL,
            accessed TEXT NOT NULL,
            path TEXT NOT NULL
        )", []
    ).unwrap();

    let mut tx = db.transaction().unwrap();

    mft.iterate_files(|file| {
        let file_info = FileInfo::new(&mft, file);
        let name = file_info.name;
        let size = file_info.size as i64;
        let is_dir = file_info.is_directory;
        let path = file_info.path.display().to_string();

        let mut created: String = "".to_string();
        match file_info.created {
            Some(c) => {
                 created = c.unix_timestamp().to_string();

                 let mut modified = "".to_string();

                 match file_info.modified {
                     Some(m) => {
                          modified = m.unix_timestamp().to_string();

                          let mut accessed = "".to_string();
         
                 match file_info.accessed {
                     Some(a) => {
                          accessed = a.unix_timestamp().to_string();

                          match tx.execute(
                            "INSERT INTO files (name, size, isdir, created, modified, accessed, path) VALUES (?, ?, ?, ?, ?, ?, ?)",
                            params![
                            name,
                            size,
                            is_dir,
                            created,
                            modified,
                            accessed,
                            path
                            ]
                        ) {
                            Ok(_) => {
                            app.emit("scanning-log", name).unwrap();
                            // println!("Inserted file: {}", name);
                            }
                            Err(e) => {
                            println!("Error: {}", e);
                            }
                        }
                            }
                            None => {
                                 modified = "null".to_string();
                            }
                        }
       
                     }
                     None => {
                          
                     }
                 }
            }
            None => {
                 created = "null".to_string();
            }
        }

    });

    tx.commit().unwrap();
    app.emit("scanning-log", "Scanning NTFS filesystem finished").unwrap();
    true
}


#[tauri::command]
fn get_files_from_sqlite() -> Result<Vec<FileObject>, String> {
    let db = get_connection().unwrap();
    let mut statement = db.prepare("SELECT * FROM files").unwrap();
    let mut files = vec![];
    // while let Ok(Some(row)) = statement.next() {
    //     files.push(FileObject {
    //         name: statement.read(0).unwrap(),
    //         size: statement.read::<i64, usize>(1).unwrap(),
    //         is_dir: statement.read::<String, usize>(2).unwrap() == "true",
    //         created: statement.read::<String, usize>(3).unwrap().parse().unwrap(),
    //         modified: statement.read::<String, usize>(4).unwrap().parse().unwrap(),
    //         accessed: statement.read::<String, usize>(5).unwrap().parse().unwrap(),
    //         thumbnail: "file".to_string(),
    //         path: statement.read(6).unwrap(),
    //     });
    // }
    Ok(files)
}

#[tauri::command]
fn search_filesystem(query: String) -> Vec<FileObject> {
    let db = get_connection().unwrap();
    // let mut statement = db.prepare("SELECT * FROM files WHERE name LIKE ?").unwrap();
    let mut files = vec![];

    // read all files that match the query

  
   let mut stmt = db.prepare("SELECT * FROM files WHERE name LIKE ?").unwrap();

   let file_iter = stmt.query_map(params![query], |row| {
       Ok(FileObject {
            id: row.get(0)?,
            name: row.get(1)?,
            size: row.get(2)?,
            is_dir: row.get(3)?,
            created: row.get(4)?,
            modified: row.get(5)?,
            accessed: row.get(6)?,
            thumbnail: "file".to_string(),
            path: row.get(7)?,
         })

    }).unwrap();


    for file in file_iter {
        let file = file.unwrap();
        println!("Found file: {:?}", file);
        files.push(file);
    }
    
    
    // return files

    files
    
    

}

#[tauri::command]
fn store_filesystem_in_sqlite() {}

#[tauri::command]
fn scan_filesystem() {}

fn get_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open("filesystem.db")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_files, print_ntfs, get_files_from_sqlite, store_filesystem_in_sqlite, scan_filesystem, search_filesystem])
        // .invoke_handler(tauri::generate_handler![print_ntfs])
        // .invoke_handler(tauri::generate_handler![get_files_from_sqlite])
        // .invoke_handler(tauri::generate_handler![store_filesystem_in_sqlite])
        // .invoke_handler(tauri::generate_handler![scan_filesystem])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
