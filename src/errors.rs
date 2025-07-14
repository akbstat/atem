use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to read xlsx: {0}")]
    ReadXlsxError(#[from] calamine::XlsxError),
    #[error("Failed to send http request: {0}")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("Failed to parse url: {0}")]
    UrlParseError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
