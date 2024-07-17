use serde::{Deserialize, Serialize};

use crate::filetransfer::params::AwsS3Params;

/// Connection parameters for Aws s3 protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct S3Params {
    pub bucket: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    /// NOTE: there are no session token and security token since they are always temporary
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
