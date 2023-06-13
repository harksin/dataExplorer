use std::collections::HashMap;

use tauri::async_runtime::RwLock;

use crate::s3::domain::S3Bucket;

#[derive(Debug, Default)]
pub struct DataExplorerState {
    pub s3_endpoints: RwLock<HashMap<String, S3Bucket>>,
}
