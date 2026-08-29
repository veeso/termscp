//! ## Google Cloud Storage Parameters
//!
//! Defines the runtime connection parameters used to build Google Cloud
//! Storage clients.

/// Google Cloud Storage's default JSON API endpoint.
pub const DEFAULT_GCS_ENDPOINT: &str = "https://storage.googleapis.com";

/// Connection parameters for Google Cloud Storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleCloudStorageParams {
    /// Target bucket name.
    pub bucket_name: String,
    /// Google Cloud Storage endpoint URL.
    pub endpoint: String,
    /// Optional path to a service-account JSON file.
    pub service_account_key: Option<String>,
}

impl GoogleCloudStorageParams {
    /// Creates Google Cloud Storage parameters using the default endpoint.
    pub fn new<S: Into<String>>(bucket_name: S) -> Self {
        Self {
            bucket_name: bucket_name.into(),
            endpoint: DEFAULT_GCS_ENDPOINT.to_string(),
            service_account_key: None,
        }
    }

    /// Sets the Google Cloud Storage endpoint.
    pub fn endpoint<S: Into<String>>(mut self, endpoint: S) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Sets the optional service-account JSON file path.
    pub fn service_account_key<S: Into<String>>(mut self, path: Option<S>) -> Self {
        self.service_account_key = path.map(Into::into);
        self
    }

    /// Reports whether the protocol's default secret is missing.
    pub fn password_missing(&self) -> bool {
        false
    }

    /// Ignores generic password secrets because GCS uses ADC or a credential file.
    pub fn set_default_secret(&mut self, _secret: String) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_use_google_storage_default_endpoint() {
        let params = GoogleCloudStorageParams::new("my-bucket");

        assert_eq!(params.bucket_name, "my-bucket");
        assert_eq!(params.endpoint, DEFAULT_GCS_ENDPOINT);
        assert_eq!(params.service_account_key, None);
        assert!(!params.password_missing());
    }

    #[test]
    fn should_override_endpoint_and_credentials_path() {
        let params = GoogleCloudStorageParams::new("my-bucket")
            .endpoint("http://127.0.0.1:4443")
            .service_account_key(Some("credentials.json"));

        assert_eq!(params.endpoint, "http://127.0.0.1:4443");
        assert_eq!(
            params.service_account_key.as_deref(),
            Some("credentials.json")
        );
    }
}
