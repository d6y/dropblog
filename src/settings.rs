use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_settings(&[clap::AppSettings::DeriveDisplayOrder]))]
pub struct Settings {
    /// IMAP hostname to connect to
    #[structopt(long, default_value = "imap.gmail.com", env = "IMAP_HOSTNAME")]
    pub hostname: String,

    /// IMAP port number
    #[structopt(long, default_value = "993", env = "IMAP_PORT")]
    pub port: u16,

    /// Email address (or user account) to check on the IMAP server
    #[structopt(long, env = "IMAP_USER")]
    pub user: String,

    /// Password for authentication
    #[structopt(long, env = "IMAP_PASSWORD", hide_env_values = true)]
    pub password: String,

    /// Dropbox access token
    #[structopt(long, env, hide_env_values = true)]
    pub dropbox_access_token: String,

    /// Existing directory for writing content
    #[structopt(long, env = "OUT_DIR")]
    pub out_dir: PathBuf,

    /// Path into media relative to OUT_DIR
    #[structopt(long, env = "MEDIA_PATH")]
    pub media_path: String,

    /// Path into posts relative to OUT_DIR
    #[structopt(long, env = "POSTS_PATH")]
    pub posts_path: String,

    /// Thumbnail width
    #[structopt(short, long, default_value = "500")]
    pub width: u16,

    /// Archive the email after processing
    #[structopt(short, long)]
    pub expunge: bool,

    /// Outline the structure of the email as additional output
    #[structopt(long)]
    pub show_outline: bool,
}
