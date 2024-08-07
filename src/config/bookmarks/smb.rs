use serde::{Deserialize, Serialize};

use crate::filetransfer::params::SmbParams as TransferSmbParams;

/// Extra Connection parameters for SMB protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct SmbParams {
    pub share: String,
    pub workgroup: Option<String>,
}

#[cfg(unix)]
impl From<TransferSmbParams> for SmbParams {
    fn from(params: TransferSmbParams) -> Self {
        Self {
            share: params.share,
            workgroup: params.workgroup,
        }
    }
}

#[cfg(windows)]
impl From<TransferSmbParams> for SmbParams {
    fn from(params: TransferSmbParams) -> Self {
        Self {
            share: params.share,
            workgroup: None,
        }
    }
}
