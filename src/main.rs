use mailparse::*;
use structopt::StructOpt;

mod settings;
use settings::Settings;

mod blog;
mod conventions;
mod dropbox;
mod email;
mod image;
mod signatureblock;

fn main() {
    let settings = Settings::from_args();

    let extract = |msg| email::extract(&settings, msg);
    let upload = |post| dropbox::upload(&settings, post);

    //TODO: Remove temporary files unless --retain-files

    match email::fetch(&settings) {
        Err(err) => stop(err),   // Failed accessing mail box
        Ok(None) => complete(0), // No messages to process
        Ok(Some(mime_message)) => {
            match parse_mail(mime_message.as_bytes()).and_then(extract) {
                Err(err) => stop(err), // Message processing failed
                Ok(info) => match blog::write(&info).and_then(upload) {
                    Err(err) => stop(err),
                    Ok(_) => complete(1),
                },
            }
        }
    }
}

fn stop<E: std::fmt::Debug>(err: E) -> ! {
    eprintln!("{:?}", err);
    std::process::exit(1)
}

fn complete(num_msgs: usize) -> ! {
    println!("{}", num_msgs);
    std::process::exit(0)
}
