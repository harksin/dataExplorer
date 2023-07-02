use aws_config::{endpoint::Endpoint, meta::region::RegionProviderChain};
use aws_sdk_s3::{
    config::{Credentials, Region},
    meta::PKG_VERSION,
    Client, Error,
};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::state::data_explorer_state::DataExplorerState;

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

    let s3_endpoint: Option<String> = Some("http://127.0.0.1:9000".to_owned());

    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let creds = Credentials::from_keys("minio", "minio123", None);
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
