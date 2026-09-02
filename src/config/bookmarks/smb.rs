//! ## Bookmark SMB Parameters
//!
//! Stores bookmark-specific SMB share configuration.

use serde::{Deserialize, Serialize};

use crate::filetransfer::params::{SmbDialect, SmbParams as TransferSmbParams};

/// Extra Connection parameters for SMB protocol
#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq, Default)]
pub struct SmbParams {
    /// SMB share name.
    pub share: String,
    /// Optional SMB workgroup used on POSIX platforms.
    pub workgroup: Option<String>,
    /// Requested SMB protocol family. `None` (older bookmarks) means `Auto`.
    #[serde(default)]
    pub dialect: Option<SmbDialect>,
}

#[cfg(posix)]
impl From<TransferSmbParams> for SmbParams {
    fn from(params: TransferSmbParams) -> Self {
        Self {
            share: params.share,
            workgroup: params.workgroup,
            dialect: Some(params.dialect),
        }
    }
}

#[cfg(win)]
impl From<TransferSmbParams> for SmbParams {
    fn from(params: TransferSmbParams) -> Self {
        Self {
            share: params.share,
            workgroup: None,
            dialect: Some(params.dialect),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_deserialize_missing_dialect_as_none() {
        let params: SmbParams = toml::from_str("share = \"temp\"").unwrap();
        assert_eq!(params.share.as_str(), "temp");
        assert_eq!(params.dialect, None);
    }

    #[test]
    fn should_deserialize_dialect() {
        let params: SmbParams = toml::from_str("share = \"temp\"\ndialect = \"smb1\"").unwrap();
        assert_eq!(params.dialect, Some(SmbDialect::Smb1));
    }

    #[test]
    fn should_round_trip_dialect() {
        let params = SmbParams {
            share: "temp".to_string(),
            workgroup: None,
            dialect: Some(SmbDialect::Smb2),
        };
        let toml_str = toml::to_string(&params).unwrap();
        let decoded: SmbParams = toml::from_str(&toml_str).unwrap();
        assert_eq!(decoded, params);
    }

    #[test]
    fn should_convert_transfer_params_with_dialect() {
        let transfer = TransferSmbParams::new("localhost", "temp").dialect(SmbDialect::Smb3);
        let params = SmbParams::from(transfer);
        assert_eq!(params.dialect, Some(SmbDialect::Smb3));
    }
}
