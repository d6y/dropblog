use imap;
use mailparse::*;
use native_tls;
use structopt::StructOpt;

mod settings;
use settings::Settings;

fn main() {
    let settings = Settings::from_args();

    match fetch(&settings) {
        Err(err) => stop(err),
        Ok(None) => complete(0),
        Ok(Some(mime_message)) => match parse_mail(mime_message.as_bytes()).and_then(extract) {
            Err(err) => stop(err),
            Ok(info) => {
                println!("{:?}", info);
                complete(1)
            }
        },
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

fn describe(mail: &ParsedMail) {
    println!("Found: {:?}", &mail.ctype);
    let parts = &mail.subparts;
    println!("Parts: {}", parts.len());
    for i in 0..parts.len() {
        describe(&parts[i]);
    }
}

#[derive(Debug)]
struct PostInfo {
    title: String,
    author: String,
    content: String,
    //date:
    //attachments:
    //content:
}

fn extract(mail: ParsedMail) -> Result<PostInfo, MailParseError> {
    describe(&mail);

    let subject: Option<String> = mail.headers.get_first_value("Subject")?;

    let sender_text: Option<String> = mail.headers.get_first_value("From")?;
    // let sender = sender_text.map(|t| addrparse(&t).unwrap()).unwrap();

    let _date = mail.headers.get_first_value("Date").unwrap();

    // pub fn dateparse(date: &str) -> Result<i64, &'static str>
    // println!("{:?}\n{:?}\n{:?}", sender, subject, date);
    // describe(&result);

    let title = subject.unwrap_or(String::from("untitled"));
    let author = sender_text.unwrap_or(String::from("someone"));

    Ok(PostInfo {
        title,
        author,
        content: body(&mail)?.unwrap_or(String::from("shrug")),
    })
}

fn body(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    if mail.ctype.mimetype == "text/plain" {
        mail.get_body().map(|s| Some(s))
    } else if mail.subparts.is_empty() {
        Ok(None)
    } else {
        let parts: Vec<Option<String>> = mail
            .subparts
            .iter()
            .flat_map(|m| body(&m))
            .filter(|o| o.is_some())
            .collect();

        if parts.is_empty() {
            Ok(None)
        } else {
            Ok(parts[0].clone())
        }
    }
}

fn fetch(settings: &Settings) -> imap::error::Result<Option<String>> {
    let tls = native_tls::TlsConnector::builder().build()?;
    let client = imap::connect(
        (&settings.hostname[..], settings.port),
        &settings.hostname,
        &tls,
    )?;

    let mut imap_session = client
        .login(&settings.user, &settings.password)
        .map_err(|(err, _client)| err)?;

    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // The body will be the mime content of the message (including heeader)
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // TODO: mark as archive and ex-purge

    imap_session.logout()?;

    Ok(Some(body))
}
