import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable, { MRT_ColumnDef } from 'material-react-table';

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import "../App.css";
import { TextField } from "@mui/material";
import { S3Bucket, S3File } from "../bindings/commands";
import { useLocation, useNavigate } from "react-router-dom";



function S3FilesExplorer() {

    console.log("S3FilesExplorer")

    const navigate = useNavigate();
    const { state } = useLocation();
    const { s3_endpoint_name } = state;

    const [files, setFiles] = useState<S3File[]>([]);



    useEffect(() => {
        //todo subdirectory
        listS3Files("plop");
    }, []);

    //todo subdirectory
    async function listS3Files(subdirectory: string) {
        console.log(s3_endpoint_name)
        let files: S3File[] = JSON.parse(await invoke("list_s3_files", { endpoint: s3_endpoint_name }))
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
                            navigate("/s3-explorer",{ state: { s3_endpoint_name: s3_endpoint_name, file: row.original.key } })
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
