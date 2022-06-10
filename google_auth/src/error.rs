pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error occurred while fetching Google's public keys")]
    FetchPublicKeys,
    #[error("missing key id")]
    MissingKeyId,
    #[error("invalid key id")]
    InvalidKeyId,
    #[error("error occurred while decoding: {0}")]
    Decoding(String),
}
