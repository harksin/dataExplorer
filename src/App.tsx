import { useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable from 'material-react-table';

import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [parquetContent, setParquetContent] = useState([]);
  const [parquetColumns, setParquetColumns] = useState([]); 
  const [fileName, setFilename] = useState("");

  async function readParquetFile() {
    setParquetColumns(JSON.parse(await invoke("read_parquet_schema", { fileName: fileName })));
    setParquetContent(JSON.parse(await invoke("read_parquet", { fileName: fileName })));
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
      <MaterialReactTable columns={parquetColumns} data={parquetContent} />
    </div>
  );
}

export default App;
