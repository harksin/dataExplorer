use std::sync::Arc;

use aws_config::{endpoint::Endpoint, meta::region::RegionProviderChain};
use aws_sdk_s3::{
    config::{Credentials, Region},
    meta::PKG_VERSION,
    Client, Error,
};
use datafusion::{
    datasource::listing::ListingTableConfig,
    prelude::{ParquetReadOptions, SessionContext},
};

use object_store::aws::AmazonS3Builder;

use log::{debug, info};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use url::Url;

use crate::{
    dataframe::{dto::DataExplorerDataframe, get_df_data, get_df_schema},
    state::data_explorer_state::DataExplorerState,
};

use super::domain::S3Bucket;

#[derive(Debug, Serialize, Deserialize)]
#[typeshare]
struct S3File {
    key: String,
}

#[tauri::command]
pub async fn list_s3_files(
    endpoint: String,
    state: tauri::State<'_, DataExplorerState>,
) -> Result<String, ()> {
    debug!("list s3 files");

    let rlock = state.s3_endpoints.read().await;

    //get bucket definition from endpoint lock
    let bucket = rlock.get(&endpoint).unwrap();

    //test data : http://127.0.0.1:9000
    //todo use nutype ?
    let s3_endpoint: Option<String> = match bucket.endpoint.is_empty() {
        true => None,
        false => Some(bucket.endpoint.clone()),
    };

    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let creds = Credentials::from_keys(bucket.access_key.clone(), bucket.secret_key.clone(), None);
    let shared_config = aws_config::from_env()
        .region(region_provider)
        .credentials_provider(creds)
        .load()
        .await;

    let s3_config = if let Some(endpoint_url) = s3_endpoint {
        aws_sdk_s3::config::Builder::from(&shared_config)
            .endpoint_url(endpoint_url)
            .build()
    } else {
        aws_sdk_s3::config::Builder::from(&shared_config).build()
    };

    let s3_client = Client::from_conf(s3_config);

    let mut files: Vec<S3File> = vec![];

    let mut result = s3_client
        .list_objects_v2()
        // .bucket(s3_bucket.bucket.clone())
        .bucket("test-bucket")
        .send()
        .await
        .unwrap();

    for content in result.contents.unwrap_or_default() {
        files.push(S3File {
            key: content.key.clone().unwrap(),
        });
    }

    debug!("{}", serde_json::to_string(&files).unwrap());

    Ok(serde_json::to_string(&files).unwrap())
}

#[tauri::command]
pub async fn save_s3_endpoint(
    endpoint: String,
    bucket: String,
    access_key: String,
    secret_key: String,
    state: tauri::State<'_, DataExplorerState>,
) -> Result<String, ()> {
    let s3_bucket = S3Bucket {
        endpoint: endpoint.clone(),
        bucket: bucket.clone(),
        access_key: access_key.clone(),
        secret_key: secret_key.clone(),
    };

    state
        .s3_endpoints
        .write()
        .await
        .insert(endpoint.clone(), s3_bucket);

    Ok("endpoint saved".to_string())
}

#[tauri::command]
pub async fn get_s3_endpoints(state: tauri::State<'_, DataExplorerState>) -> Result<String, ()> {
    let endpoints = state.s3_endpoints.read().await;

    let payload = serde_json::to_string(&Vec::from_iter(endpoints.values())).unwrap();
    Ok(payload)
}

#[tauri::command]
pub async fn read_s3_parquet(
    endpoint: String,
    file: String,
    state: tauri::State<'_, DataExplorerState>,
) -> Result<String, ()> {
    let rlock = state.s3_endpoints.read().await;
    let bucket = rlock.get(&endpoint).unwrap();

    info!("Reading parquet file from s3: {:?}", bucket);

    let ctx = SessionContext::new();

    let s3 = AmazonS3Builder::new()
        .with_allow_http(true)
        .with_endpoint(bucket.endpoint.clone())
        .with_bucket_name(bucket.bucket.clone())
        .with_region("us-east-1") //todo remove hardcoded region
        .with_access_key_id(bucket.access_key.clone())
        .with_secret_access_key(bucket.secret_key.clone())
        .build()
        .unwrap();

    let s3_url = Url::parse(&bucket.endpoint).unwrap();
    ctx.runtime_env()
        .register_object_store(&s3_url, Arc::new(s3));

    // register parquet file with the execution context
    let df = ctx
        .read_parquet(
            format!("{}/{}", bucket.endpoint, file),
            ParquetReadOptions::default(),
        )
        .await
        .unwrap();

    Ok(DataExplorerDataframe {
        columns: get_df_schema(&df),
        data: get_df_data(&df).await,
    }
    .into())
}
