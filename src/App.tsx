import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable from 'material-react-table';

import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import "./App.css";

type DataExplorerDataFrame = {
  columns: any;
  data: any;
}

function App() {
  const [dataframe, setDataframe] = useState<DataExplorerDataFrame>({ columns: [], data: [] });
  const [fileName, setFilename] = useState("");
  const [sqlQuery, setSqlQuery] = useState("");

  async function readParquetFile() {
    let de_df: DataExplorerDataFrame = JSON.parse(await invoke("read_parquet", { fileName: fileName }))
    setDataframe(de_df)
  }

  async function queryParquetFile() {
    let de_df: DataExplorerDataFrame = JSON.parse(await invoke("query_parquet", { fileName: fileName, query: sqlQuery }))
    setDataframe(de_df)
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
    setDataframe({ columns: [], data: [] });
  }

  return (
    <div className="container">
      <h1>Welcome dataExplorer!</h1>

      <p>Start </p>
      <div className="row" >
        <button onClick={openFile} >Open Parquet File</button>
        <button onClick={cleanData} >Clean</button>
      </div>
      {fileName != "" &&
        <div>
          <div className="row">
            <form
              onSubmit={(e) => {
                e.preventDefault();
                queryParquetFile();
              }}
            >
              <p> Enter your SQL query bellow </p>
              <p> The table name is "data", here is an example query : SELECT * FROM data WHERE my_value == 100  </p>

              <input
                id="sql-input"
                onChange={(e) => setSqlQuery(e.currentTarget.value)}
                placeholder="Enter an sql query..."
              />
              <button type="submit">Run</button>
            </form>
          </div>
          <MaterialReactTable columns={dataframe?.columns} data={dataframe?.data} />
        </div>}
    </div>
  );
}

export default App;
