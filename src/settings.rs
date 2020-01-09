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

    /// Email address (or user account) to check on the IMAP server.
    #[structopt(long, env = "IMAP_USER")]
    pub user: String,

    /// Password for authentication.
    #[structopt(long, env = "IMAP_PASSWORD", hide_env_values = true)]
    pub password: String,

    /// Existing directory for writing blog posts markdown.
    #[structopt(long, env = "POSTS_DIR")]
    pub posts_dir: PathBuf,

    /// Existing directory for writing media files.
    #[structopt(long, env = "MEDIA_DIR")]
    pub media_dir: PathBuf,

    /// Outline the structure of the email as additional output
    #[structopt(long)]
    pub show_outline: bool,
}
