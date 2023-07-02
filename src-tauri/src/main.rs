// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::hash::Hash;

use dataExplorer::__cmd__get_s3_endpoints;
use dataExplorer::__cmd__list_s3_files;
use dataExplorer::__cmd__read_s3_parquet;
use dataExplorer::__cmd__save_s3_endpoint;
use dataExplorer::dataframe::dto::DataExplorerDataframe;
use dataExplorer::dataframe::dto::MuiTableColumns;
use dataExplorer::dataframe::get_df_data;
use dataExplorer::dataframe::get_df_schema;
use dataExplorer::s3::s3_commands::get_s3_endpoints;
use dataExplorer::s3::s3_commands::list_s3_files;
use dataExplorer::s3::s3_commands::read_s3_parquet;
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

fn main() {
    tauri::Builder::default()
        .manage(DataExplorerState::default())
        .invoke_handler(tauri::generate_handler![
            read_parquet,
            query_parquet,
            save_s3_endpoint,
            get_s3_endpoints,
            list_s3_files,
            read_s3_parquet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
