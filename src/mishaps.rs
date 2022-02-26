use thiserror::Error;

#[derive(Error, Debug)]
pub enum Mishap {
    #[error(transparent)]
    Imap(#[from] imap::error::Error),

    #[error(transparent)]
    Email(#[from] mailparse::MailParseError),

    #[error("Bad email field: {0}")]
    EmailField(String),

    #[error(transparent)]
    File(#[from] std::io::Error),

    #[error(transparent)]
    DropboxConnection(#[from] reqwest::Error),

    #[error("Upload failed: {0}")]
    UploadRejected(http::status::StatusCode),

    #[error("Json parsing failed, got: {0}. Reason: {1}")]
    JsonContent(String, String),
}
