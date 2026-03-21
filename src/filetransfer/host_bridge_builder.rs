//! ## Host Bridge Builder
//!
//! Builds host bridge implementations from persisted host bridge parameters and
//! the active configuration client.

use super::{HostBridgeParams, RemoteFsBuilder};
use crate::host::{HostBridge, Localhost, RemoteBridged};
use crate::system::config_client::ConfigClient;

/// Builds the host-side filesystem bridge used during file transfer sessions.
pub struct HostBridgeBuilder;

impl HostBridgeBuilder {
    /// Builds a host bridge from serialized parameters.
    ///
    /// Returns an error when the selected host protocol and parameters are inconsistent.
    pub fn build(
        params: HostBridgeParams,
        config_client: &ConfigClient,
    ) -> Result<Box<dyn HostBridge>, String> {
        match params {
            HostBridgeParams::Localhost(path) => Localhost::new(path)
                .map(|host| Box::new(host) as Box<dyn HostBridge>)
                .map_err(|e| e.to_string()),
            HostBridgeParams::Remote(protocol, params) => {
                RemoteFsBuilder::build(protocol, params, config_client)
                    .map(|host| Box::new(RemoteBridged::from(host)) as Box<dyn HostBridge>)
            }
        }
    }
}
