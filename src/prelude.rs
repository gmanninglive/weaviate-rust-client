pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function..
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("usage error: `{0}`")]
    Usage(&'static str),

    #[error("an error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("an error occurred")]
    Reqwest(#[from] reqwest::Error),
}
