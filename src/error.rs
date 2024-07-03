use serde_derive::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("GenericError {0}")]
    Generic(String),

    #[error("SerdeJsonError {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("ReqwestError {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("OpenAlexError\n{0}")]
    OpenAlex(OpenAlexError),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenAlexError {
    pub error: String,
    pub message: String,
}

impl std::fmt::Display for OpenAlexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error:   {}\nMessage: {}", self.error, self.message)
    }
}
