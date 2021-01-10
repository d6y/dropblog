use structopt::StructOpt;

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
    let settings = Settings::from_args();

    if let Some(refresh) = &settings.dropbox_refresh_token {
        // If we have a refresh token, we're good to run
        dropblog(refresh, &settings);
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
        Err(err) => stop(err),
    }
}

fn dropblog(refresh: &str, settings: &Settings) {
    let extract = |msg| email::extract(&settings, msg);
    let upload = |post| dropbox::upload(refresh, &settings, post);

    match email::fetch(&settings) {
        Err(err) => stop(err),   // Failed accessing mail box
        Ok(None) => complete(0), // No messages to process
        Ok(Some(mime_message)) => {
            match email::parse(&mime_message).and_then(extract) {
                Err(err) => stop(err), // Message processing failed
                Ok(info) => match blog::write(&info).and_then(upload) {
                    Err(err) => stop(err),
                    Ok(_) => complete(1),
                },
            }
        }
    }
}

fn stop<E: std::fmt::Display>(err: E) -> ! {
    eprintln!("Failed: {}", err);
    std::process::exit(1)
}

fn complete(num_msgs: usize) -> ! {
    println!("{}", num_msgs);
    std::process::exit(0)
}
