//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

// Locals
use super::{AuthActivity, FileTransferParams, FormTab, HostBridgeProtocol};
use crate::filetransfer::HostBridgeParams;
use crate::filetransfer::params::{
    AwsS3Params, GenericProtocolParams, KubeProtocolParams, ProtocolParams, SmbParams,
    WebDAVProtocolParams,
};

impl AuthActivity {
    /// Delete bookmark
    pub(super) fn del_bookmark(&mut self, idx: usize) {
        let name = self.bookmarks_list.get(idx).cloned();
        if let Some(bookmarks_cli) = self.bookmarks_client_mut() {
            // Iterate over kyes
            if let Some(name) = name {
                bookmarks_cli.del_bookmark(&name);
                // Write bookmarks
                self.write_bookmarks();
            }
            // Delete element from vec
            self.bookmarks_list.remove(idx);
        }
    }

    /// Load selected bookmark (at index) to input fields
    pub(super) fn load_bookmark(&mut self, form_tab: FormTab, idx: usize) {
        if let Some(bookmarks_cli) = self.bookmarks_client() {
            // Iterate over bookmarks
            if let Some(key) = self.bookmarks_list.get(idx) {
                if let Some(bookmark) = bookmarks_cli.get_bookmark(key) {
                    // Load parameters into components
                    match form_tab {
                        FormTab::Remote => self.load_remote_bookmark_into_gui(bookmark),
                        FormTab::HostBridge => self.load_host_bridge_bookmark_into_gui(bookmark),
                    }
                }
            }
        }
    }

    /// Save current input fields as a bookmark
    pub(super) fn save_bookmark(&mut self, form_tab: FormTab, name: String, save_password: bool) {
        let params = match form_tab {
            FormTab::Remote => match self.collect_remote_host_params() {
                Ok(p) => p,
                Err(e) => {
                    self.mount_error(e);
                    return;
                }
            },
            FormTab::HostBridge => match self.collect_host_bridge_params() {
                Ok(HostBridgeParams::Remote(protocol, params)) => FileTransferParams {
                    protocol,
                    params,
                    remote_path: None,
                    local_path: None,
                },
                Ok(HostBridgeParams::Localhost(_)) => {
                    self.mount_error("You cannot save a localhost bookmark");
                    return;
                }
                Err(e) => {
                    self.mount_error(e);
                    return;
                }
            },
        };

        if let Some(bookmarks_cli) = self.bookmarks_client_mut() {
            bookmarks_cli.add_bookmark(name.clone(), params, save_password);
            // Save bookmarks
            self.write_bookmarks();
            // Remove `name` from bookmarks if exists
            self.bookmarks_list.retain(|b| b.as_str() != name.as_str());
            // Push bookmark to list and sort
            self.bookmarks_list.push(name);
            self.sort_bookmarks();
        }
    }
    /// Delete recent
    pub(super) fn del_recent(&mut self, idx: usize) {
        let name = self.recents_list.get(idx).cloned();
        if let Some(client) = self.bookmarks_client_mut() {
            if let Some(name) = name {
                client.del_recent(&name);
                // Write bookmarks
                self.write_bookmarks();
            }
            // Delete element from vec
            self.recents_list.remove(idx);
        }
    }

    /// Load selected recent (at index) to input fields
    pub(super) fn load_recent(&mut self, form_tab: FormTab, idx: usize) {
        if let Some(client) = self.bookmarks_client() {
            // Iterate over bookmarks
            if let Some(key) = self.recents_list.get(idx) {
                if let Some(bookmark) = client.get_recent(key) {
                    // Load parameters
                    match form_tab {
                        FormTab::Remote => self.load_remote_bookmark_into_gui(bookmark),
                        FormTab::HostBridge => self.load_host_bridge_bookmark_into_gui(bookmark),
                    }
                }
            }
        }
    }

    /// Save current input fields as a "recent"
    pub(super) fn save_recent(&mut self) {
        let params = match self.collect_remote_host_params() {
            Ok(p) => p,
            Err(e) => {
                self.mount_error(e);
                return;
            }
        };
        if let Some(bookmarks_cli) = self.bookmarks_client_mut() {
            bookmarks_cli.add_recent(params);
            // Save bookmarks
            self.write_bookmarks();
        }
    }

    /// Write bookmarks to file
    fn write_bookmarks(&mut self) {
        if let Some(bookmarks_cli) = self.bookmarks_client() {
            if let Err(err) = bookmarks_cli.write_bookmarks() {
                self.mount_error(format!("Could not write bookmarks: {err}").as_str());
            }
        }
    }

    /// Initialize bookmarks client
    pub(super) fn init_bookmarks_client(&mut self) {
        if let Some(cli) = self.bookmarks_client_mut() {
            // Load bookmarks into list
            let mut bookmarks_list: Vec<String> = Vec::with_capacity(cli.iter_bookmarks().count());
            for bookmark in cli.iter_bookmarks() {
                bookmarks_list.push(bookmark.clone());
            }
            // Load recents into list
            let mut recents_list: Vec<String> = Vec::with_capacity(cli.iter_recents().count());
            for recent in cli.iter_recents() {
                recents_list.push(recent.clone());
            }
            self.bookmarks_list = bookmarks_list;
            self.recents_list = recents_list;
            // Sort bookmark list
            self.sort_bookmarks();
            self.sort_recents();
        }
    }

    // -- privates

    /// Sort bookmarks in list
    fn sort_bookmarks(&mut self) {
        // Convert to lowercase when sorting
        self.bookmarks_list
            .sort_by(|a, b| a.to_lowercase().as_str().cmp(b.to_lowercase().as_str()));
    }

    /// Sort recents in list
    fn sort_recents(&mut self) {
        // Reverse order
        self.recents_list.sort_by(|a, b| b.cmp(a));
    }

    /// Load bookmark data into the gui components
    fn load_host_bridge_bookmark_into_gui(&mut self, bookmark: FileTransferParams) {
        // Load parameters into components
        self.host_bridge_protocol = HostBridgeProtocol::Remote(bookmark.protocol);
        self.mount_host_bridge_protocol(self.host_bridge_protocol);
        self.mount_remote_directory(
            FormTab::HostBridge,
            bookmark
                .remote_path
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        self.mount_local_directory(
            FormTab::HostBridge,
            bookmark
                .local_path
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        match bookmark.params {
            ProtocolParams::AwsS3(params) => {
                self.load_bookmark_s3_into_gui(FormTab::HostBridge, params)
            }
            ProtocolParams::Kube(params) => {
                self.load_bookmark_kube_into_gui(FormTab::HostBridge, params)
            }

            ProtocolParams::Generic(params) => {
                self.load_bookmark_generic_into_gui(FormTab::HostBridge, params)
            }
            ProtocolParams::Smb(params) => {
                self.load_bookmark_smb_into_gui(FormTab::HostBridge, params)
            }
            ProtocolParams::WebDAV(params) => {
                self.load_bookmark_webdav_into_gui(FormTab::HostBridge, params)
            }
        }
    }

    /// Load bookmark data into the gui components
    fn load_remote_bookmark_into_gui(&mut self, bookmark: FileTransferParams) {
        // Load parameters into components
        self.remote_protocol = bookmark.protocol;
        self.mount_remote_protocol(bookmark.protocol);
        self.mount_remote_directory(
            FormTab::Remote,
            bookmark
                .remote_path
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        self.mount_local_directory(
            FormTab::Remote,
            bookmark
                .local_path
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        match bookmark.params {
            ProtocolParams::AwsS3(params) => {
                self.load_bookmark_s3_into_gui(FormTab::Remote, params)
            }
            ProtocolParams::Kube(params) => {
                self.load_bookmark_kube_into_gui(FormTab::Remote, params)
            }

            ProtocolParams::Generic(params) => {
                self.load_bookmark_generic_into_gui(FormTab::Remote, params)
            }
            ProtocolParams::Smb(params) => self.load_bookmark_smb_into_gui(FormTab::Remote, params),
            ProtocolParams::WebDAV(params) => {
                self.load_bookmark_webdav_into_gui(FormTab::Remote, params)
            }
        }
    }

    fn load_bookmark_generic_into_gui(&mut self, form_tab: FormTab, params: GenericProtocolParams) {
        self.mount_address(form_tab, params.address.as_str());
        self.mount_port(form_tab, params.port);
        self.mount_username(form_tab, params.username.as_deref().unwrap_or(""));
        self.mount_password(form_tab, params.password.as_deref().unwrap_or(""));
    }

    fn load_bookmark_s3_into_gui(&mut self, form_tab: FormTab, params: AwsS3Params) {
        self.mount_s3_bucket(form_tab, params.bucket_name.as_str());
        self.mount_s3_region(form_tab, params.region.as_deref().unwrap_or(""));
        self.mount_s3_endpoint(form_tab, params.endpoint.as_deref().unwrap_or(""));
        self.mount_s3_profile(form_tab, params.profile.as_deref().unwrap_or(""));
        self.mount_s3_access_key(form_tab, params.access_key.as_deref().unwrap_or(""));
        self.mount_s3_secret_access_key(
            form_tab,
            params.secret_access_key.as_deref().unwrap_or(""),
        );
        self.mount_s3_security_token(form_tab, params.security_token.as_deref().unwrap_or(""));
        self.mount_s3_session_token(form_tab, params.session_token.as_deref().unwrap_or(""));
        self.mount_s3_new_path_style(form_tab, params.new_path_style);
    }

    fn load_bookmark_kube_into_gui(&mut self, form_tab: FormTab, params: KubeProtocolParams) {
        self.mount_kube_cluster_url(form_tab, params.cluster_url.as_deref().unwrap_or(""));
        self.mount_kube_namespace(form_tab, params.namespace.as_deref().unwrap_or(""));
        self.mount_kube_client_cert(form_tab, params.client_cert.as_deref().unwrap_or(""));
        self.mount_kube_client_key(form_tab, params.client_key.as_deref().unwrap_or(""));
        self.mount_kube_username(form_tab, params.username.as_deref().unwrap_or(""));
    }

    fn load_bookmark_smb_into_gui(&mut self, form_tab: FormTab, params: SmbParams) {
        self.mount_address(form_tab, params.address.as_str());
        #[cfg(posix)]
        self.mount_port(form_tab, params.port);
        self.mount_username(form_tab, params.username.as_deref().unwrap_or(""));
        self.mount_password(form_tab, params.password.as_deref().unwrap_or(""));
        self.mount_smb_share(form_tab, &params.share);
        #[cfg(posix)]
        self.mount_smb_workgroup(form_tab, params.workgroup.as_deref().unwrap_or(""));
    }

    fn load_bookmark_webdav_into_gui(&mut self, form_tab: FormTab, params: WebDAVProtocolParams) {
        self.mount_webdav_uri(form_tab, &params.uri);
        self.mount_username(form_tab, &params.username);
        self.mount_password(form_tab, &params.password);
    }
}
