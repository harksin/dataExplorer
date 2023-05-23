// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use datafusion::error::Result;
use datafusion::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};

#[tauri::command]
async fn read_parquet(file_name: String) -> String {
    info!("Reading parquet file: {}", file_name);

    let ctx = SessionContext::new();

    // register parquet file with the execution context
    ctx.register_parquet("test_table", &file_name, ParquetReadOptions::default())
        .await
        .unwrap();

    // execute the query
    let df = ctx
        .sql(
            "SELECT * \
        FROM test_table",
        )
        .await
        .unwrap();

    // print the results
    let batches = df.collect().await.unwrap();

    let json_data =
        datafusion::arrow::json::writer::record_batches_to_json_rows(&batches[..]).unwrap();

    let serialised_json_data = serde_json::to_string(&json_data).unwrap();

    serialised_json_data
}

#[derive(Debug, Serialize, Deserialize)]
struct MuiTableColumns {
    accessorKey: String,
    header: String,
}

impl From<&String> for MuiTableColumns {
    fn from(s: &String) -> Self {
        MuiTableColumns {
            accessorKey: s.to_string(),
            header: s.to_string(),
        }
    }
}

#[tauri::command]
async fn read_parquet_schema(file_name: String) -> String {
    info!("Reading parquet file: {}", file_name);

    let ctx = SessionContext::new();

    // register parquet file with the execution context
    let df = ctx
        .read_parquet(&file_name, ParquetReadOptions::default())
        .await
        .unwrap();

    // execute the query
    let schema = df
        .schema()
        .fields()
        .iter()
        .map(|f| MuiTableColumns::from(f.name()))
        .collect::<Vec<_>>();

    let serialised_schema = serde_json::to_string(&schema).unwrap();

    serialised_schema
}

fn main() {
    env_logger::init();

    info!("Starting DataExplorer");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_parquet, read_parquet_schema])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
