import { useEffect, useState } from "react";
import React, { useMemo } from 'react';
import MaterialReactTable, { MRT_ColumnDef } from 'material-react-table';

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

import "../App.css";
import { TextField } from "@mui/material";

//todo generate from rust structs
type S3Endpoints = {
    endpoint: String,
    bucket: String,
    access_key: String,
    secret_key: String,
}

function S3Explorer() {
    const [s3Endpoints, setS3Endpoints] = useState<S3Endpoints[]>([]);


    useEffect(() => {
        getS3Endpoints();
      }, []);


    async function save_s3_empoint(endpointToSave: S3Endpoints) {
        let result = await invoke("save_s3_endpoint", {
            endpoint: endpointToSave.endpoint,
            bucket: endpointToSave.bucket,
            accessKey: endpointToSave.access_key,
            secretKey: endpointToSave.secret_key
        })

        console.log(endpointToSave.endpoint + " : " + result)
        getS3Endpoints();
    }

    async function getS3Endpoints() {
        let s3_endpoints: S3Endpoints[] = JSON.parse(await invoke("get_s3_endpoints"))
        setS3Endpoints(s3_endpoints)
    }

    const columns = useMemo<MRT_ColumnDef<S3Endpoints>[]>(
        () => [
            {
                accessorKey: 'endpoint',
                header: 'endpoint',
            },
            {
                accessorKey: 'bucket',
                header: 'bucket',
            },
            {
                accessorKey: 'access_key',
                header: 'access_key',
            },
            {
                accessorKey: 'secret_key',
                header: 'secret_key',
            },
        ],
        [],
    );

    const handleSubmit = (event: React.SyntheticEvent) => {
        console.log('saving s3 endpoint');
        event.preventDefault();

        const target = event.target as typeof event.target & {
            endpoint: { value: string };
            bucket: { value: string };
            access_key: { value: string };
            secret_key: { value: string };
        };

        save_s3_empoint({
            endpoint: target.endpoint.value,
            bucket: target.bucket.value,
            access_key: target.access_key.value,
            secret_key: target.secret_key.value
        });

    };

    return (
        <div className="localPanel">
            <div>
                <div className="row">

                    <form onSubmit={handleSubmit}>
                        <TextField required
                            id="endpoint"
                            name="endpoint"
                            type="text"
                            variant="standard"
                            helperText="endpoint url" />
                        <br />
                        <TextField id="bucket"
                            name="bucket"
                            type="text"
                            variant="standard"
                            helperText="Bucket" />
                        <br />
                        <TextField id="access_key"
                            name="access_key"
                            type="text"
                            variant="standard"
                            helperText="Access Key" />
                        <br />
                        <TextField id="secret_key"
                            name="secret_key"
                            type="text"
                            variant="standard"
                            helperText="Secret Key" />
                        <br />
                        <button type="submit">Create new endpoint</button>
                    </form>
                </div>
                <MaterialReactTable columns={columns} data={s3Endpoints} />
            </div>
        </div>
    );
}

export default S3Explorer;
