use super::{HostBridgeParams, RemoteFsBuilder};
use crate::host::{HostBridge, Localhost, RemoteBridged};
use crate::system::config_client::ConfigClient;

pub struct HostBridgeBuilder;

impl HostBridgeBuilder {
    /// Build Host Bridge from parms
    ///
    /// if protocol and parameters are inconsistent, the function will return an error.
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
