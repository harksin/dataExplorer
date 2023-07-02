import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable from 'material-react-table';

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import "../App.css";
import { DataExplorerDataframe } from "../bindings/commands";

function S3Explorer() {
  const [dataframe, setDataframe] = useState<DataExplorerDataframe>({ columns: [], data: [] });

 
  async function readS3ParquetFile() {
    let de_df: DataExplorerDataframe = JSON.parse(await invoke("read_s3_parquet", { url: "not implemented" }))
    console.log(de_df)
    setDataframe(de_df)
  }

  useEffect(() => {
     { readS3ParquetFile() };
  }, [])

  return (
    <div className="localPanel">
        <div>
          <MaterialReactTable columns={dataframe?.columns} data={dataframe?.data} />
        </div>
    </div>
  );
}

export default S3Explorer;
