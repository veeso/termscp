use std::path::PathBuf;
use std::str::FromStr;

use tuirealm::{State, StateValue};

use super::*;
use crate::filetransfer::FileTransferParams;
use crate::filetransfer::params::{
    AwsS3Params, GenericProtocolParams, KubeProtocolParams, ProtocolParams, SmbParams,
    WebDAVProtocolParams,
};

impl AuthActivity {
    pub(in crate::ui::activities::auth) fn get_generic_params_input(
        &self,
        form_tab: FormTab,
    ) -> GenericProtocolParams {
        let addr = self.get_input_addr(form_tab);
        let port = self.get_input_port(form_tab);
        let username = self.get_input_username(form_tab);
        let password = self.get_input_password(form_tab);
        GenericProtocolParams::default()
            .address(addr)
            .port(port)
            .username(username)
            .password(password)
    }

    pub(in crate::ui::activities::auth) fn get_s3_params_input(
        &self,
        form_tab: FormTab,
    ) -> AwsS3Params {
        let bucket = self.get_input_s3_bucket(form_tab);
        let region = self.get_input_s3_region(form_tab);
        let endpoint = self.get_input_s3_endpoint(form_tab);
        let profile = self.get_input_s3_profile(form_tab);
        let access_key = self.get_input_s3_access_key(form_tab);
        let secret_access_key = self.get_input_s3_secret_access_key(form_tab);
        let security_token = self.get_input_s3_security_token(form_tab);
        let session_token = self.get_input_s3_session_token(form_tab);
        let new_path_style = self.get_input_s3_new_path_style(form_tab);
        AwsS3Params::new(bucket, region, profile)
            .endpoint(endpoint)
            .access_key(access_key)
            .secret_access_key(secret_access_key)
            .security_token(security_token)
            .session_token(session_token)
            .new_path_style(new_path_style)
    }

    pub(in crate::ui::activities::auth) fn get_kube_params_input(
        &self,
        form_tab: FormTab,
    ) -> KubeProtocolParams {
        let namespace = self.get_input_kube_namespace(form_tab);
        let cluster_url = self.get_input_kube_cluster_url(form_tab);
        let username = self.get_input_kube_username(form_tab);
        let client_cert = self.get_input_kube_client_cert(form_tab);
        let client_key = self.get_input_kube_client_key(form_tab);
        KubeProtocolParams {
            namespace,
            cluster_url,
            username,
            client_cert,
            client_key,
        }
    }

    #[cfg(posix)]
    pub(in crate::ui::activities::auth) fn get_smb_params_input(
        &self,
        form_tab: FormTab,
    ) -> SmbParams {
        let share = self.get_input_smb_share(form_tab);
        let workgroup = self.get_input_smb_workgroup(form_tab);
        let address = self.get_input_addr(form_tab);
        let port = self.get_input_port(form_tab);
        let username = self.get_input_username(form_tab);
        let password = self.get_input_password(form_tab);

        SmbParams::new(address, share)
            .port(port)
            .username(username)
            .password(password)
            .workgroup(workgroup)
    }

    #[cfg(win)]
    pub(in crate::ui::activities::auth) fn get_smb_params_input(
        &self,
        form_tab: FormTab,
    ) -> SmbParams {
        let share = self.get_input_smb_share(form_tab);
        let address = self.get_input_addr(form_tab);
        let username = self.get_input_username(form_tab);
        let password = self.get_input_password(form_tab);

        SmbParams::new(address, share)
            .username(username)
            .password(password)
    }

    pub(in crate::ui::activities::auth) fn get_webdav_params_input(
        &self,
        form_tab: FormTab,
    ) -> WebDAVProtocolParams {
        let uri = self.get_webdav_uri(form_tab);
        let username = self.get_input_username(form_tab).unwrap_or_default();
        let password = self.get_input_password(form_tab).unwrap_or_default();

        WebDAVProtocolParams {
            uri,
            username,
            password,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_remote_directory(
        &self,
        form_tab: FormTab,
    ) -> Option<PathBuf> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::RemoteDirectory))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => {
                Some(PathBuf::from(x.as_str()))
            }
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_local_directory(
        &self,
        form_tab: FormTab,
    ) -> Option<PathBuf> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::LocalDirectory))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => {
                Some(PathBuf::from(x.as_str()))
            }
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_webdav_uri(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::WebDAVUri))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_addr(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Address))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_port(&self, form_tab: FormTab) -> u16 {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Port))
        {
            Ok(State::One(StateValue::String(x))) => u16::from_str(x.as_str()).unwrap_or_default(),
            _ => 0,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_username(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Username))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_password(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::Password))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_bucket(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Bucket))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_region(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Region))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_endpoint(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Endpoint))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_profile(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3Profile))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_access_key(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3AccessKey))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_secret_access_key(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3SecretAccessKey))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_security_token(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3SecurityToken))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_session_token(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::S3SessionToken))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_s3_new_path_style(
        &self,
        form_tab: FormTab,
    ) -> bool {
        matches!(
            self.app
                .state(&Self::form_tab_id(form_tab, AuthFormId::S3NewPathStyle)),
            Ok(State::One(StateValue::Usize(0)))
        )
    }

    pub(in crate::ui::activities::auth) fn get_input_kube_namespace(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeNamespace))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_kube_cluster_url(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeClusterUrl))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_kube_username(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeUsername))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_kube_client_cert(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeClientCert))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_kube_client_key(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::KubeClientKey))
        {
            Ok(State::One(StateValue::String(x))) if !x.is_empty() => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_input_smb_share(&self, form_tab: FormTab) -> String {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::SmbShare))
        {
            Ok(State::One(StateValue::String(x))) => x,
            _ => String::new(),
        }
    }

    #[cfg(posix)]
    pub(in crate::ui::activities::auth) fn get_input_smb_workgroup(
        &self,
        form_tab: FormTab,
    ) -> Option<String> {
        match self
            .app
            .state(&Self::form_tab_id(form_tab, AuthFormId::SmbWorkgroup))
        {
            Ok(State::One(StateValue::String(x))) => Some(x),
            _ => None,
        }
    }

    pub(in crate::ui::activities::auth) fn get_new_bookmark(&self) -> (String, bool) {
        let name = match self.app.state(&Id::BookmarkName) {
            Ok(State::One(StateValue::String(name))) => name,
            _ => String::default(),
        };
        if matches!(
            self.app.state(&Id::BookmarkSavePassword),
            Ok(State::One(StateValue::Usize(0)))
        ) {
            (name, true)
        } else {
            (name, false)
        }
    }

    pub(in crate::ui::activities::auth) fn max_input_mask_size(&self) -> u16 {
        Self::input_mask_size(self.host_bridge_input_mask())
            .max(Self::input_mask_size(self.remote_input_mask()))
            + 3
    }

    fn input_mask_size(input_mask: InputMask) -> u16 {
        match input_mask {
            InputMask::AwsS3
            | InputMask::Generic
            | InputMask::Kube
            | InputMask::Smb
            | InputMask::WebDAV => 12,
            InputMask::Localhost => 3,
        }
    }

    pub(in crate::ui::activities::auth) fn fmt_bookmark(
        name: &str,
        b: FileTransferParams,
    ) -> String {
        let addr = Self::fmt_recent(b);
        format!("{name} ({addr})")
    }

    pub(in crate::ui::activities::auth) fn fmt_recent(b: FileTransferParams) -> String {
        let protocol = b.protocol.to_string().to_lowercase();
        match b.params {
            ProtocolParams::AwsS3(s3) => {
                let profile = match s3.profile {
                    Some(p) => format!("[{p}]"),
                    None => String::default(),
                };
                format!(
                    "{}://{}{} ({}) {}",
                    protocol,
                    s3.endpoint.unwrap_or_default(),
                    s3.bucket_name,
                    s3.region.as_deref().unwrap_or("custom"),
                    profile
                )
            }
            ProtocolParams::Generic(params) => {
                let username = match params.username {
                    None => String::default(),
                    Some(u) => format!("{u}@"),
                };
                format!(
                    "{}://{}{}:{}",
                    protocol, username, params.address, params.port
                )
            }
            ProtocolParams::Kube(params) => {
                format!(
                    "{}://{}{}",
                    protocol,
                    params
                        .namespace
                        .as_deref()
                        .map(|x| format!("/{x}"))
                        .unwrap_or_else(|| String::from("default")),
                    params
                        .cluster_url
                        .as_deref()
                        .map(|x| format!("@{x}"))
                        .unwrap_or_default()
                )
            }
            #[cfg(posix)]
            ProtocolParams::Smb(params) => {
                let username = match params.username {
                    None => String::default(),
                    Some(u) => format!("{u}@"),
                };
                format!(
                    "\\\\{username}{}:{}\\{}",
                    params.address, params.port, params.share
                )
            }
            #[cfg(win)]
            ProtocolParams::Smb(params) => {
                let username = match params.username {
                    None => String::default(),
                    Some(u) => format!("{u}@"),
                };
                format!("\\\\{username}{}\\{}", params.address, params.share)
            }
            ProtocolParams::WebDAV(params) => params.uri,
        }
    }
}
