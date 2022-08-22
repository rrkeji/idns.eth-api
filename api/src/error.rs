// use anyhow::{Context, Result};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Fetching runtime version failed. Are you connected to the correct endpoint?")]
    RuntimeVersion,
    #[error("Operation needs a signer to be set in the api")]
    NoSigner,
    #[error("Substrate Connect Failed")]
    SubstrateConnectFailed,
    #[error("IPFS Connect Failed")]
    IpfsConnectFailed,
    #[error("Not fountd service")]
    NotFoundService,
    #[error("Normal")]
    NormalError(i32, String),
    #[error("Send extrinsic Failed")]
    SendExtrinsicError,
    #[error("Error converting NumberOrHex to Balance")]
    TryFromIntError,
    #[error(transparent)]
    ProtoDecodeError(#[from] prost::DecodeError),
    #[error(transparent)]
    SqliteError(#[from] rusqlite::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
