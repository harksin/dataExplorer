// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::hash::Hash;

use dataExplorer::__cmd__get_s3_endpoints;
use dataExplorer::__cmd__list_s3_files;
use dataExplorer::__cmd__save_s3_endpoint;
use dataExplorer::s3::s3_commands::get_s3_endpoints;
use dataExplorer::s3::s3_commands::list_s3_files;
use dataExplorer::s3::s3_commands::save_s3_endpoint;
use dataExplorer::state::data_explorer_state::DataExplorerState;
use datafusion::error::Result;
use datafusion::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;
use tauri::async_runtime::RwLock;

#[tauri::command]
async fn read_parquet(file_name: String) -> String {
    info!("Reading parquet file: {}", file_name);

    let ctx = SessionContext::new();

    // register parquet file with the execution context
    let df = ctx
        .read_parquet(&file_name, ParquetReadOptions::default())
        .await
        .unwrap();

    DataExplorerDataframe {
        columns: get_df_schema(&df),
        data: get_df_data(&df).await,
    }
    .into()
}

#[tauri::command]
async fn query_parquet(file_name: String, query: String) -> String {
    info!("Reading parquet file: {}", file_name);

    let ctx = SessionContext::new();

    // register parquet file with the execution context
    ctx.register_parquet("data", &file_name, ParquetReadOptions::default())
        .await
        .unwrap();

    // execute the query
    let df = ctx.sql(query.as_str()).await.unwrap();

    DataExplorerDataframe {
        columns: get_df_schema(&df),
        data: get_df_data(&df).await,
    }
    .into()
}

#[derive(Debug, Serialize, Deserialize)]
struct DataExplorerDataframe {
    columns: Vec<MuiTableColumns>,
    data: Vec<Map<String, Value>>,
}

impl From<DataExplorerDataframe> for String {
    fn from(df: DataExplorerDataframe) -> Self {
        serde_json::to_string(&df).unwrap()
    }
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

fn get_df_schema(df: &DataFrame) -> Vec<MuiTableColumns> {
    let schema = df
        .schema()
        .fields()
        .iter()
        .map(|f| MuiTableColumns::from(f.name()))
        .collect::<Vec<_>>();

    schema
}

async fn get_df_data(df: &DataFrame) -> Vec<Map<String, Value>> {
    let batches = df.clone().collect().await.unwrap();

    let json_data =
        datafusion::arrow::json::writer::record_batches_to_json_rows(&batches[..]).unwrap();

    json_data
}

fn main() {
    tauri::Builder::default()
        .manage(DataExplorerState::default())
        .invoke_handler(tauri::generate_handler![
            read_parquet,
            query_parquet,
            save_s3_endpoint,
            get_s3_endpoints,
            list_s3_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
