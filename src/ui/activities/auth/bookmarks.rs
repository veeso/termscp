//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

// Locals
use super::{AuthActivity, FileTransferParams};
use crate::filetransfer::params::{AwsS3Params, GenericProtocolParams, ProtocolParams};

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
    pub(super) fn load_bookmark(&mut self, idx: usize) {
        if let Some(bookmarks_cli) = self.bookmarks_client() {
            // Iterate over bookmarks
            if let Some(key) = self.bookmarks_list.get(idx) {
                if let Some(bookmark) = bookmarks_cli.get_bookmark(key) {
                    // Load parameters into components
                    self.load_bookmark_into_gui(bookmark);
                }
            }
        }
    }

    /// Save current input fields as a bookmark
    pub(super) fn save_bookmark(&mut self, name: String, save_password: bool) {
        let params = match self.collect_host_params() {
            Ok(p) => p,
            Err(e) => {
                self.mount_error(e);
                return;
            }
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
    pub(super) fn load_recent(&mut self, idx: usize) {
        if let Some(client) = self.bookmarks_client() {
            // Iterate over bookmarks
            if let Some(key) = self.recents_list.get(idx) {
                if let Some(bookmark) = client.get_recent(key) {
                    // Load parameters
                    self.load_bookmark_into_gui(bookmark);
                }
            }
        }
    }

    /// Save current input fields as a "recent"
    pub(super) fn save_recent(&mut self) {
        let params = match self.collect_host_params() {
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
        // Conver to lowercase when sorting
        self.bookmarks_list
            .sort_by(|a, b| a.to_lowercase().as_str().cmp(b.to_lowercase().as_str()));
    }

    /// Sort recents in list
    fn sort_recents(&mut self) {
        // Reverse order
        self.recents_list.sort_by(|a, b| b.cmp(a));
    }

    /// Load bookmark data into the gui components
    fn load_bookmark_into_gui(&mut self, bookmark: FileTransferParams) {
        // Load parameters into components
        self.protocol = bookmark.protocol;
        self.mount_protocol(bookmark.protocol);
        self.mount_remote_directory(
            bookmark
                .entry_directory
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        match bookmark.params {
            ProtocolParams::AwsS3(params) => self.load_bookmark_s3_into_gui(params),
            ProtocolParams::Generic(params) => self.load_bookmark_generic_into_gui(params),
        }
    }

    fn load_bookmark_generic_into_gui(&mut self, params: GenericProtocolParams) {
        self.mount_address(params.address.as_str());
        self.mount_port(params.port);
        self.mount_username(params.username.as_deref().unwrap_or(""));
        self.mount_password(params.password.as_deref().unwrap_or(""));
    }

    fn load_bookmark_s3_into_gui(&mut self, params: AwsS3Params) {
        self.mount_s3_bucket(params.bucket_name.as_str());
        self.mount_s3_region(params.region.as_deref().unwrap_or(""));
        self.mount_s3_endpoint(params.endpoint.as_deref().unwrap_or(""));
        self.mount_s3_profile(params.profile.as_deref().unwrap_or(""));
        self.mount_s3_access_key(params.access_key.as_deref().unwrap_or(""));
        self.mount_s3_secret_access_key(params.secret_access_key.as_deref().unwrap_or(""));
        self.mount_s3_security_token(params.security_token.as_deref().unwrap_or(""));
        self.mount_s3_session_token(params.session_token.as_deref().unwrap_or(""));
        self.mount_s3_new_path_style(params.new_path_style);
    }
}
