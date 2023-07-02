use datafusion::prelude::DataFrame;
use serde_json::{Map, Value};

use self::dto::MuiTableColumns;

pub mod dto;

pub fn get_df_schema(df: &DataFrame) -> Vec<MuiTableColumns> {
    let schema = df
        .schema()
        .fields()
        .iter()
        .map(|f| MuiTableColumns::from(f.name()))
        .collect::<Vec<_>>();

    schema
}

pub async fn get_df_data(df: &DataFrame) -> Vec<Map<String, Value>> {
    let batches = df.clone().collect().await.unwrap();

    let json_data =
        datafusion::arrow::json::writer::record_batches_to_json_rows(&batches[..]).unwrap();

    json_data
}
