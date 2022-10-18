use clap::Parser;
mod settings;
use settings::Settings;

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

    if let Some(refresh) = &settings.dropbox_refresh_token {
        // If we have a refresh token, we're good to run
        match dropblog(refresh, &settings) {
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

fn show_token(code: &str, key: &str, secret: &str) {
    match dropbox::get_refresh_token(code, key, secret) {
        Ok(refresh) => println!("{}", refresh),
        Err(err) => stop("token refresh", err),
    }
}

fn dropblog(refresh: &str, settings: &Settings) -> Result<usize, mishaps::Mishap> {
    let extract = |msg| email::extract(settings, msg);
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
