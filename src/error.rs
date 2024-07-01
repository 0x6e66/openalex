#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("GenericError {0}")]
    Generic(String),

    #[error("SerdeJsonError {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("ReqwestError {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("OpenAlexError {0}")]
    OpenAlex(String),
}
