import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type FileObject = {
  name: string;
  size: number;
  created: string;
  modified: string;
  thumbnail: string;
  path: string;
}

function App() {
  const [files, setFiles] = useState<string[]>([]);

  const getFiles = async () => {
    const response = await invoke("get_files");
    setFiles(response as string[]);
  };

  useEffect(() => {
    getFiles();
  }, []);

  return (
    <main className="container">
      <h1>Orion File Explorer</h1>
      <div>
        {files.length > 0 ? (
          <ul>
            {files.map((file, index) => (
              <li key={index}>
                {/* <img src={file.thumbnail} alt={file.name} />
                <p>{file.name}</p>
                <p>{file.size} bytes</p>
                <p>Created: {file.created}</p>
                <p>Modified: {file.modified}</p> */}
                <p>{file}</p>
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
