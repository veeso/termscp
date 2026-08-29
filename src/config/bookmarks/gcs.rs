//! ## Bookmark Google Cloud Storage Parameters
//!
//! Stores bookmark-specific Google Cloud Storage connection settings.

use serde::{Deserialize, Serialize};

use crate::filetransfer::params::{DEFAULT_GCS_ENDPOINT, GoogleCloudStorageParams};

fn default_gcs_endpoint() -> String {
    DEFAULT_GCS_ENDPOINT.to_string()
}

/// Google Cloud Storage connection parameters stored in a bookmark.
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct GcsParams {
    /// Bucket name to open.
    pub bucket: String,
    /// Google Cloud Storage endpoint URL.
    #[serde(default = "default_gcs_endpoint")]
    pub endpoint: String,
    /// Optional path to a service-account JSON file.
    pub service_account_key: Option<String>,
}

impl From<GoogleCloudStorageParams> for GcsParams {
    fn from(params: GoogleCloudStorageParams) -> Self {
        Self {
            bucket: params.bucket_name,
            endpoint: if params.endpoint.is_empty() {
                default_gcs_endpoint()
            } else {
                params.endpoint
            },
            service_account_key: params.service_account_key,
        }
    }
}

impl From<GcsParams> for GoogleCloudStorageParams {
    fn from(params: GcsParams) -> Self {
        GoogleCloudStorageParams::new(params.bucket)
            .endpoint(if params.endpoint.is_empty() {
                default_gcs_endpoint()
            } else {
                params.endpoint
            })
            .service_account_key(params.service_account_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_normalize_empty_endpoint() {
        let params = GoogleCloudStorageParams::from(GcsParams {
            bucket: String::from("archive-bucket"),
            endpoint: String::new(),
            service_account_key: None,
        });

        assert_eq!(params.endpoint, DEFAULT_GCS_ENDPOINT);
    }
}
