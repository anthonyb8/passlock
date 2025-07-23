use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not Found Error: HOME directory not found.")]
    HomeNotFoundError,
    #[error("Aboard Error: {0}")]
    AboardError(#[from] arboard::Error),
    #[error("Inquire Error: {0}")]
    InquireError(#[from] inquire::InquireError),
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("AESGCM error")]
    AESGCMError,
    #[error("Decoding Error: {0}")]
    DecodeError(#[from] base64::DecodeError),
    #[error(" Error: {0}")]
    UTF8Error(#[from] FromUtf8Error),
}

// Then implement From manually
impl From<aes_gcm::Error> for Error {
    fn from(_: aes_gcm::Error) -> Self {
        Error::AESGCMError
    }
}

#[macro_export]
macro_rules! error{
    ($variant:ident, $($arg:tt)*) => {
        Error::$variant(format!($($arg)*))
    };
}

pub type Result<T> = std::result::Result<T, Error>;
