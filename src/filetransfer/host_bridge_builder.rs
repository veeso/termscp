use super::{HostBridgeParams, RemoteFsBuilder};
use crate::host::{HostBridge, Localhost, RemoteBridged};
use crate::system::config_client::ConfigClient;

pub struct HostBridgeBuilder;

impl HostBridgeBuilder {
    /// Build Host Bridge from parms
    ///
    /// if protocol and parameters are inconsistent, the function will panic.
    pub fn build(params: HostBridgeParams, config_client: &ConfigClient) -> Box<dyn HostBridge> {
        match params {
            HostBridgeParams::Localhost(path) => {
                Box::new(Localhost::new(path).expect("Failed to create Localhost"))
            }
            HostBridgeParams::Remote(protocol, params) => Box::new(RemoteBridged::from(
                RemoteFsBuilder::build(protocol, params, config_client),
            )),
        }
    }
}
