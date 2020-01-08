use chrono::{DateTime, TimeZone, Utc};
use imap;
use mailparse::*;
use native_tls;
use structopt::StructOpt;

mod settings;
use settings::Settings;

fn main() {
    let settings = Settings::from_args();

    match fetch(&settings) {
        Err(err) => stop(err),   // Failed accessing mail box
        Ok(None) => complete(0), // No messages to process
        Ok(Some(mime_message)) => match parse_mail(mime_message.as_bytes()).and_then(extract) {
            Err(err) => stop(err), // Message processing failed
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
    content: Option<String>,
    date: DateTime<Utc>,
    //attachments:
}

fn extract(mail: ParsedMail) -> Result<PostInfo, MailParseError> {
    describe(&mail);

    let sender: Option<String> = sender(&mail)?;
    let date: Option<DateTime<Utc>> = date(&mail)?;
    let subject: Option<String> = mail.headers.get_first_value("Subject")?;
    let content: Option<String> = body(&mail)?;

    let title = subject
        .or(content.clone())
        .unwrap_or(String::from("Untitled"));
    let author = sender.unwrap_or(String::from("Someone"));

    Ok(PostInfo {
        title: title.trim().to_owned(),
        author: author.trim().to_owned(),
        content: content.map(|str| str.trim().to_owned()),
        date: date.unwrap_or(Utc::now()),
    })
}

fn date(mail: &ParsedMail) -> Result<Option<DateTime<Utc>>, MailParseError> {
    let date_header: Option<String> = mail.headers.get_first_value("Date")?;

    let timestamp: Result<Vec<i64>, &str> = date_header.iter().map(|str| dateparse(&str)).collect();

    let utc: Option<DateTime<Utc>> = timestamp
        .map_err(|e| MailParseError::Generic(e))?
        .first()
        .map(|&ts| Utc.timestamp(ts, 0));

    Ok(utc)
}

fn sender(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    let sender_text: Option<String> = mail.headers.get_first_value("From")?;
    match sender_text {
        None => Ok(None),
        Some(str) => match addrparse(&str) {
            Err(err) => Err(MailParseError::Generic(&err)),
            Ok(addrs) if addrs.is_empty() => Ok(None),
            Ok(addrs) => Ok(addrs
                .extract_single_info()
                .and_then(|info| info.display_name)),
        },
    }
}

fn body(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    if mail.ctype.mimetype == "text/plain" {
        mail.get_body().map(|str| Some(str))
    } else if mail.subparts.is_empty() {
        Ok(None)
    } else {
        let parts: Result<Vec<Option<String>>, MailParseError> =
            mail.subparts.iter().map(|m| body(&m)).collect();

        let valid_parts: Result<Vec<String>, MailParseError> =
            parts.map(|os| os.into_iter().flatten().collect());

        match valid_parts {
            Ok(vec) if vec.is_empty() => Ok(None),
            Err(err) => Err(err),
            Ok(vec) => Ok(Some(vec[0].clone())),
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
