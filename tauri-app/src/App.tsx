import { useEffect, useState } from "react";

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

type FileObject =  {
  name: string,
  size: number,
  is_dir: boolean,
  created: string,
  modified: string,
  thumbnail: string,
  path: string,
}

function App() {
  const [files, setFiles] = useState<FileObject[]>([]);
  const [dirPath, setDirPath] = useState("/home/f2ville");
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

  useEffect(() => {
    getFiles();
  }, [dirPath]);

  return (
    <main className="container">
      <h1>Orion File Explorer</h1>
      <div>
        {files.length > 0 ? (
          <ul className="files">
            {files.map((file, index) => (
              <li key={index} className="file">
                <img src={file.thumbnail} alt={file.thumbnail} />
                <p>{file.name}</p>
                <div>
                <p>{file.size} bytes</p>
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
