import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable, { MRT_ColumnDef } from 'material-react-table';

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import "../App.css";
import { TextField } from "@mui/material";
import { S3Bucket, S3File } from "../bindings/commands";
import { useNavigate } from "react-router-dom";



function S3FilesExplorer() {

    console.log("S3FilesExplorer")
    
    const navigate = useNavigate();

    const [files, setFiles] = useState<S3File[]>([]);



    useEffect(() => {
        listS3Files("plop");
      }, []);


    async function listS3Files(endpoint:string) {
        let files: S3File[] = JSON.parse(await invoke("list_s3_files", { endpoint }))
        console.log(files)
        setFiles(files)
      }
    

    const columns = useMemo<MRT_ColumnDef<S3File>[]>(
        () => [
            {
                accessorKey: 'key',
                header: 'object',
            },
        ],
        [],
    );

    return (
        <div className="localPanel">
            <div>
                <MaterialReactTable columns={columns} data={files}
                muiTableBodyRowProps={({ row }) => ({
                    onClick: (_) => {
                      console.info(row.original);
                      navigate("/s3-explorer")
                    },
                    sx: {
                      cursor: 'pointer', //you might want to change the cursor too when adding an onClick
                    },
                  })}
                />
            </div>
        </div>
    );
}

export default S3FilesExplorer;
