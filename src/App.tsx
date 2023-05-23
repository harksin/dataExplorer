import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [parquetContent, setParquetContent] = useState("");
  const [fileName, setFilename] = useState("");

  async function readParquetFile() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setParquetContent(await invoke("read_parquet", { fileName: fileName }));
  }

  return (
    <div className="container">
      <h1>Welcome dataExplorer!</h1>

      <p>put here the parquet file path you want to read.</p>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            readParquetFile();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setFilename(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="submit">Load</button>
        </form>
      </div>
      <p>{parquetContent}</p>
    </div>
  );
}

export default App;
