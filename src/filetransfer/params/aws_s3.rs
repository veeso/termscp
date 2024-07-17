/// Connection parameters for AWS S3 protocol
#[derive(Debug, Clone)]
pub struct AwsS3Params {
    pub bucket_name: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    pub security_token: Option<String>,
    pub session_token: Option<String>,
    pub new_path_style: bool,
}

// -- S3 params

impl AwsS3Params {
    /// Instantiates a new `AwsS3Params` struct
    pub fn new<S: AsRef<str>>(bucket: S, region: Option<S>, profile: Option<S>) -> Self {
        Self {
            bucket_name: bucket.as_ref().to_string(),
            region: region.map(|x| x.as_ref().to_string()),
            profile: profile.map(|x| x.as_ref().to_string()),
            endpoint: None,
            access_key: None,
            secret_access_key: None,
            security_token: None,
            session_token: None,
            new_path_style: false,
        }
    }

    /// Construct aws s3 params with specified endpoint
    pub fn endpoint<S: AsRef<str>>(mut self, endpoint: Option<S>) -> Self {
        self.endpoint = endpoint.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided access key
    pub fn access_key<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.access_key = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided secret_access_key
    pub fn secret_access_key<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.secret_access_key = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided security_token
    pub fn security_token<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.security_token = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Construct aws s3 params with provided session_token
    pub fn session_token<S: AsRef<str>>(mut self, key: Option<S>) -> Self {
        self.session_token = key.map(|x| x.as_ref().to_string());
        self
    }

    /// Specify new path style when constructing aws s3 params
    pub fn new_path_style(mut self, new_path_style: bool) -> Self {
        self.new_path_style = new_path_style;
        self
    }

    /// Returns whether a password is supposed to be required for this protocol params.
    /// The result true is returned ONLY if the supposed secret is MISSING!!!
    pub fn password_missing(&self) -> bool {
        self.secret_access_key.is_none() && self.security_token.is_none()
    }

    /// Set password
    pub fn set_default_secret(&mut self, secret: String) {
        self.secret_access_key = Some(secret);
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_init_aws_s3_params() {
        let params: AwsS3Params = AwsS3Params::new("omar", Some("eu-west-1"), Some("test"));
        assert_eq!(params.bucket_name.as_str(), "omar");
        assert_eq!(params.region.as_deref().unwrap(), "eu-west-1");
        assert_eq!(params.profile.as_deref().unwrap(), "test");
        assert!(params.endpoint.is_none());
        assert!(params.access_key.is_none());
        assert!(params.secret_access_key.is_none());
        assert!(params.security_token.is_none());
        assert!(params.session_token.is_none());
        assert_eq!(params.new_path_style, false);
    }

    #[test]
    fn should_init_aws_s3_params_with_optionals() {
        let params: AwsS3Params = AwsS3Params::new("omar", Some("eu-west-1"), Some("test"))
            .endpoint(Some("http://omar.it"))
            .access_key(Some("pippo"))
            .secret_access_key(Some("pluto"))
            .security_token(Some("omar"))
            .session_token(Some("gerry-scotti"))
            .new_path_style(true);
        assert_eq!(params.bucket_name.as_str(), "omar");
        assert_eq!(params.region.as_deref().unwrap(), "eu-west-1");
        assert_eq!(params.profile.as_deref().unwrap(), "test");
        assert_eq!(params.endpoint.as_deref().unwrap(), "http://omar.it");
        assert_eq!(params.access_key.as_deref().unwrap(), "pippo");
        assert_eq!(params.secret_access_key.as_deref().unwrap(), "pluto");
        assert_eq!(params.security_token.as_deref().unwrap(), "omar");
        assert_eq!(params.session_token.as_deref().unwrap(), "gerry-scotti");
        assert_eq!(params.new_path_style, true);
    }
}
