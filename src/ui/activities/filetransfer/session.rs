//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

mod connection;
mod navigation;
mod transfer;

pub(super) use transfer::TransferPayload;
