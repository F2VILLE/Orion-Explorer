import { useEffect, useState } from "react";
import { listen } from '@tauri-apps/api/event';
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

/*
type FileObject as the following :
struct FileObject {
    name: String,
    size: u64,
    is_dir: bool,
    created: SystemTime,
    modified: SystemTime,
    thumbnail: String,
    path: String,
}

*/

type FileObject = {
  name: string,
  size: number,
  is_dir: boolean,
  created: string,
  modified: string,
  thumbnail: string,
  path: string,
}

function bytesToMo(bytes: number): string {
  return (bytes / (1024 * 1024)).toFixed(2);
}

function App() {
  const [files, setFiles] = useState<FileObject[]>([]);
  const [dirPath, setDirPath] = useState("/home/f2ville");
  const [scanningLog, setScanningLog] = useState("");
  const [search, setSearch] = useState("");
  const getFiles = async () => {
    const response = await invoke("get_files", {
      path: dirPath
    });
    const lesFichiers = (response as FileObject[]).sort((a, b) => {
      if (a.is_dir === b.is_dir) {
        return a.name.localeCompare(b.name);
      } else {
        return a.is_dir ? -1 : 1;
      }
    })
    console.log(lesFichiers);
    setFiles(lesFichiers);
  };

  listen<string>("scanning-log", (event) => { 
    setScanningLog(event.payload);
  })

  const searchFiles = async (search: string) => {
    const response = await invoke("search_filesystem", {
      query: search
    });
    console.log(response);
    const lesFichiers = (response as FileObject[]).sort((a, b) => {
      if (a.is_dir === b.is_dir) {
        return a.name.localeCompare(b.name);
      } else {
        return a.is_dir ? -1 : 1;
      }

    }
    )
    console.log(lesFichiers);
    setFiles(lesFichiers);

  }
  // useEffect(() => {
  //   searchFiles(search)
  // }, [search]);

  // useEffect(() => {
  //   // getFiles();

  // }, [dirPath]);



  return (
    <main className="container">
      <div>
        <p>State: {scanningLog}</p>
      </div>
      <div>
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search..."
        />

        <button onClick={
          async () => {
            await searchFiles(search);
          }
        }>search</button>
      </div>

      <button className="button" onClick={async () => {
        await invoke("print_ntfs");
      }}>Test</button>
      <h1>Orion File Explorer</h1>
      <div>
        {files.length > 0 ? (
          <ul className="files">
            {files.map((file, index) => (
              <li key={index} className="file">
                <img src={file.thumbnail} alt={file.thumbnail} />
                <p>{file.name}</p>
                <p className="little">{file.path.replace("\\\\.\\", "")}</p>
                <div>
                  <p>{bytesToMo(file.size )} Mo</p>
                  <p>Created: {file.created}</p>
                  <p>Modified: {file.modified}</p>

                </div>
              </li>

            ))}
          </ul>
        ) : (
          <></>
        )}

      </div>
    </main>
  );
}

export default App;
