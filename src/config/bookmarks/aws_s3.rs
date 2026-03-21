//! ## Bookmark S3 Parameters
//!
//! Stores the bookmark-specific representation of AWS S3 connection settings.

use serde::{Deserialize, Serialize};

use crate::filetransfer::params::AwsS3Params;

/// Connection parameters for Aws s3 protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct S3Params {
    /// Bucket name to open.
    pub bucket: String,
    /// AWS region used for the bucket.
    pub region: Option<String>,
    /// Custom endpoint URL for S3-compatible services.
    pub endpoint: Option<String>,
    /// Shared credentials profile name.
    pub profile: Option<String>,
    /// Static access key identifier.
    pub access_key: Option<String>,
    /// Static secret access key.
    pub secret_access_key: Option<String>,
    /// NOTE: there are no session token and security token since they are always temporary
    /// Whether to force path-style bucket addressing.
    pub new_path_style: Option<bool>,
}

impl From<AwsS3Params> for S3Params {
    fn from(params: AwsS3Params) -> Self {
        S3Params {
            bucket: params.bucket_name,
            region: params.region,
            endpoint: params.endpoint,
            profile: params.profile,
            access_key: params.access_key,
            secret_access_key: params.secret_access_key,
            new_path_style: Some(params.new_path_style),
        }
    }
}

impl From<S3Params> for AwsS3Params {
    fn from(params: S3Params) -> Self {
        AwsS3Params::new(params.bucket, params.region, params.profile)
            .endpoint(params.endpoint)
            .access_key(params.access_key)
            .secret_access_key(params.secret_access_key)
            .new_path_style(params.new_path_style.unwrap_or(false))
    }
}
