use std::path::Path;

use clap::Parser;
mod settings;
use log::debug;
use settings::Settings;
use tempfile::TempDir;

mod blog;
mod conventions;
mod dropbox;
mod email;
mod image;
mod mishaps;
mod signatureblock;

fn main() {
    let settings = Settings::parse();

    env_logger::init();
    ensure_imagemagik_installed();

    let temp_dir = TempDir::new().expect("creating temporary directory");
    debug!("Writing to: {:?}", temp_dir.path());
    ensure_out_dir_exists(&settings, temp_dir.path());

    if let Some(refresh) = &settings.dropbox_refresh_token {
        // If we have a refresh token, we're good to run
        match dropblog(refresh, &settings, temp_dir.path()) {
            Ok(count) => complete(count),
            Err(err) => stop("dropblog processing", err),
        }
    } else if let Some(code) = &settings.dropbox_code {
        // If dropbox code is supplied, use it to fetch and print a refresh token
        show_token(
            code,
            &settings.dropbox_app_key,
            &settings.dropbox_app_secret,
        );
    } else {
        // Without a code or refresh token, show the URL for where a user should go to get a code
        println!("{}", dropbox::show_auth_url(&settings.dropbox_app_key));
    }
}

fn ensure_imagemagik_installed() {
    if !image::imagemagic_installed() {
        panic!("Did not find ImageMagik");
    }
}

fn ensure_out_dir_exists(settings: &Settings, out_dir: &Path) {
    let media_dir = out_dir.join(&settings.media_path);
    let posts_dir = out_dir.join(&settings.posts_path);

    if !media_dir.exists() {
        std::fs::create_dir_all(media_dir).expect("creating media dir")
    };

    if !posts_dir.exists() {
        std::fs::create_dir_all(posts_dir).expect("creating post dir")
    };
}

fn show_token(code: &str, key: &str, secret: &str) {
    match dropbox::get_refresh_token(code, key, secret) {
        Ok(refresh) => println!("{}", refresh),
        Err(err) => stop("token refresh", err),
    }
}

fn dropblog(refresh: &str, settings: &Settings, out_dir: &Path) -> Result<usize, mishaps::Mishap> {
    let extract = |msg| email::extract(settings, out_dir, msg);
    let upload = |post| dropbox::upload(refresh, settings, &post);

    let client = imap::ClientBuilder::new(&settings.hostname, settings.port).rustls()?;

    let mut imap_session = client
        .login(&settings.user, &settings.password)
        .map_err(|(err, _client)| err)?;

    imap_session.select(&settings.mailbox)?;

    let result = match email::fetch(settings, &mut imap_session)? {
        None => Ok(0), // No messages to process
        Some(mime_message) => email::parse(&mime_message)
            .and_then(extract)
            .and_then(blog::write)
            .and_then(upload),
    };

    imap_session.logout()?;

    result
}

fn stop<E: std::fmt::Display>(context: &str, err: E) -> ! {
    eprintln!("Failed: {} at {}", err, context);
    std::process::exit(1)
}

fn complete(num_msgs: usize) -> ! {
    println!("{}", num_msgs);
    std::process::exit(0)
}
