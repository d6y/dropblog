use thiserror::Error;

use http;
use imap;
use mailparse;
use native_tls;
use reqwest;

#[derive(Error, Debug)]
pub enum Mishap {
    #[error(transparent)]
    Network(#[from] native_tls::Error),

    #[error(transparent)]
    Imap(#[from] imap::error::Error),

    #[error(transparent)]
    GenericMailParse(#[from] mailparse::MailParseError),

    #[error(transparent)]
    File(#[from] std::io::Error),

    #[error(transparent)]
    DropboxConnection(#[from] reqwest::Error),

    #[error("Upload failed: {0}")]
    UploadRejected(http::status::StatusCode),
}
