use chrono::{DateTime, TimeZone, Utc};
use imap;
use mailparse::*;
use native_tls;
use tempfile;

use super::settings::Settings;

use super::blog::{Image, PostInfo};

use std::fs::File;
use std::io::Write;

pub fn fetch(settings: &Settings) -> imap::error::Result<Option<String>> {
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
    // via a setting perhaps -a or -e

    imap_session.logout()?;

    Ok(Some(body))
}

pub fn extract(settings: &Settings, mail: ParsedMail) -> Result<PostInfo, MailParseError> {
    if settings.show_outline {
        // Debug output to show the structure of the MIME message
        outline(&mail);
    }

    let sender: Option<String> = sender(&mail)?;
    let subject: Option<String> = mail.headers.get_first_value("Subject")?;
    let content: Option<String> = body(&mail)?;
    let date: DateTime<Utc> = date(&mail)?.unwrap_or(Utc::now());

    // The blog post title will be the subject line, and if that's missing use the body text
    let title = subject
        .filter(|str| !str.is_empty())
        .or(content.clone())
        .unwrap_or(String::from("Untitled"));
    let author = sender.unwrap_or(String::from("Someone"));

    let slug = slug::slugify(&title);

    let attachments = attachments(&mail)?;

    Ok(PostInfo::new(
        slug,
        title,
        author,
        content,
        date,
        attachments,
    ))
}

fn date(mail: &ParsedMail) -> Result<Option<DateTime<Utc>>, MailParseError> {
    let date_header: Option<String> = mail.headers.get_first_value("Date")?;

    let timestamp: Result<Vec<i64>, &str> = date_header.iter().map(|str| dateparse(&str)).collect();

    let utc: Option<DateTime<Utc>> = timestamp
        .map_err(|e| MailParseError::Generic(e))?
        .first()
        .map(|&seconds| Utc.timestamp_millis(1000 * seconds));

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

fn save_raw_body(bytes: Vec<u8>) -> Result<File, std::io::Error> {
    let mut file = tempfile::tempfile()?;
    file.write(bytes.as_slice())?;
    Ok(file)
}

fn find_attachemnts<'a>(mail: &'a ParsedMail<'a>) -> Vec<&'a ParsedMail<'a>> {
    let head: Vec<&ParsedMail> = vec![mail]
        .into_iter()
        .filter(|m| m.ctype.mimetype.starts_with("image"))
        .collect();

    let tail = mail.subparts.iter().map(|m| find_attachemnts(m)).flatten();

    head.into_iter().chain(tail).collect()
}

fn attachments(mail: &ParsedMail) -> Result<Vec<Image>, MailParseError> {
    let mut images = Vec::new();

    for (count, part) in find_attachemnts(&mail).iter().enumerate() {
        let bytes = part.get_body_raw()?;
        let file =
            save_raw_body(bytes).map_err(|_err| MailParseError::Generic(&"Failed to save file"));

        images.push(Image {
            file: file?,
            mimetype: mail.ctype.mimetype.clone(),
        });
    }

    Ok(images)
}

fn outline(mail: &ParsedMail) {
    describe_child("+", &mail);
}

fn describe_child(prefix: &str, mail: &ParsedMail) {
    println!(
        "{} {:?} (children: {})",
        &prefix,
        &mail.ctype,
        &mail.subparts.len()
    );
    let indent = String::from("--") + &prefix;
    for child in mail.subparts.iter() {
        describe_child(&indent, &child);
    }
}
