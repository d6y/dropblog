use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Settings {
    /// IMAP hostname to connect to
    #[arg(long, default_value = "imap.gmail.com", env = "IMAP_HOSTNAME")]
    pub hostname: String,

    /// IMAP port number
    #[arg(long, default_value = "993", env = "IMAP_PORT")]
    pub port: u16,

    /// Email address (or user account) to check on the IMAP server
    #[arg(long, env = "IMAP_USER")]
    pub user: String,

    /// Password for authentication
    #[arg(long, env = "IMAP_PASSWORD", hide_env_values = true)]
    pub password: String,

    // The mailbox to read from
    #[arg(short, long, default_value = "INBOX")]
    pub mailbox: String,

    /// Dropbox refresh token
    #[arg(long, env, hide_env_values = true)]
    pub dropbox_refresh_token: Option<String>,

    /// Dropbox code (supplied by user, used once to fetch a refresh token)
    #[arg(long, env, hide_env_values = true)]
    pub dropbox_code: Option<String>,

    /// Dropbox app key (also called client ID)
    #[arg(long, env)]
    pub dropbox_app_key: String,

    /// Dropbox app secret (also called client secret)
    #[arg(long, env, hide_env_values = true)]
    pub dropbox_app_secret: String,

    /// Existing directory for writing content
    #[arg(long, env = "OUT_DIR")]
    pub out_dir: PathBuf,

    /// Path into media relative to OUT_DIR
    #[arg(long, env = "MEDIA_PATH")]
    pub media_path: String,

    /// Path into posts relative to OUT_DIR
    #[arg(long, env = "POSTS_PATH")]
    pub posts_path: String,

    /// Thumbnail width
    #[arg(short, long, default_value = "500")]
    pub width: u16,

    /// Archive the email after processing
    #[arg(short, long)]
    pub expunge: bool,

    /// Outline the structure of the email as additional output
    #[arg(long)]
    pub show_outline: bool,
}
