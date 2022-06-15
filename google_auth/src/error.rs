pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error occurred while fetching Google's public keys: {0}")]
    FetchPublicKeys(String),
    #[error("missing key id")]
    MissingKeyId,
    #[error("invalid key id")]
    InvalidKeyId,
    #[error("error occurred while decoding: {0}")]
    Decoding(String),
    #[error("token key type not supported: {0}")]
    NotSupported(String),
    #[error("this token id is not meant for this app: {0}")]
    InvalidTokenId(String),
}
