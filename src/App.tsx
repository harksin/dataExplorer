import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable from 'material-react-table';

import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import "./App.css";

function App() {
  const [parquetContent, setParquetContent] = useState([]);
  const [parquetColumns, setParquetColumns] = useState([]);
  const [fileName, setFilename] = useState("");

  async function readParquetFile() {
    setParquetColumns(JSON.parse(await invoke("read_parquet_schema", { fileName: fileName })));
    setParquetContent(JSON.parse(await invoke("read_parquet", { fileName: fileName })));
  }


  useEffect(() => {
    if (fileName !== "") { readParquetFile() };
  }, [fileName])


  async function openFile() {
    let properties = {
      defaultPath: 'C:\\',
      directory: false,
      multiple: false,
      filters: [{
        extensions: ['parquet'], name: "*"
      }]
    };
    const selected = await open(properties);

    if (Array.isArray(selected)) {
      console.log(`User selected ${selected.length} files, which is not yet allowed`);
    } else if (selected === null) {
      console.log('User cancelled the dialog');
    } else {
      setFilename(selected);
    }

  }

  async function cleanData() {

    setFilename("");
    setParquetColumns([]);
    setParquetContent([]);

  }

  return (
    <div className="container">
      <h1>Welcome dataExplorer!</h1>

      <p>Start </p>
      <div className="row" >
        <button onClick={openFile} >Open Parquet File</button>
        <button onClick={cleanData} >Clean</button>
      </div>
      {fileName != "" && <MaterialReactTable columns={parquetColumns} data={parquetContent} />}
    </div>
  );
}

export default App;
