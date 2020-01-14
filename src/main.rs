use mailparse::*;
use structopt::StructOpt;

mod settings;
use settings::Settings;

mod email;
use email::{extract, fetch};

mod blog;
mod conventions;
mod image;

fn main() {
    let settings = Settings::from_args();

    match fetch(&settings) {
        Err(err) => stop(err),   // Failed accessing mail box
        Ok(None) => complete(0), // No messages to process
        Ok(Some(mime_message)) => {
            match parse_mail(mime_message.as_bytes()).and_then(|m| extract(&settings, m)) {
                Err(err) => stop(err), // Message processing failed
                Ok(info) => match blog::write(&info) {
                    Err(err) => stop(err),
                    Ok(files) => {
                        println!("{:?}", files);
                        complete(1)
                    }
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
