use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct S3Bucket {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
