use serde::Serialize;
use typeshare::typeshare;

#[derive(Debug, Serialize, Default)]
#[typeshare]
pub struct S3Bucket {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
