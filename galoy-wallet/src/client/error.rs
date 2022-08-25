use thiserror::Error;

#[derive(Error, Debug)]
pub enum GaloyWalletError {
    #[error("GaloyWalletError: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("GaloyWalletError: {0}")]
    UnknownResponse(String),
}