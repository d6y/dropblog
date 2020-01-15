use super::blog::PostInfo;
use super::settings::Settings;
use reqwest;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn upload(settings: &Settings, post: &PostInfo) -> Result<(), Box<dyn Error>> {
    // TODO: Async
    let dropbox = Dropbox::new(&settings.dropbox_access_token);
    let blog_response = dropbox.upload(&post.filename, &post.relative_path)?;
    dbg!(blog_response);

    for image in post.attachments.iter() {
        let img_resp = dropbox.upload(&image.file, &image.relative_path)?;
        dbg!(img_resp);

        let thumb = &image.thumbnail;
        let thumb_resp = dropbox.upload(&thumb.file, &thumb.relative_path)?;
        dbg!(thumb_resp);
    }

    Ok(())
}

struct Dropbox {
    token: String,
    client: reqwest::blocking::Client,
}

impl Dropbox {
    fn new(token: &String) -> Dropbox {
        Dropbox {
            token: token.clone(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn upload(&self, filename: &PathBuf, dropbox_path: &String) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;

        let api_args = format!("{{\"path\":\"{}{}\"}}", "/", dropbox_path);

        let request = self
            .client
            .post("https://content.dropboxapi.com/2/files/upload")
            .bearer_auth(&self.token)
            .header("Content-Type", "application/octet-stream")
            .header("Dropbox-API-Arg", api_args)
            .body(file);

        let resp = request.send()?;
        match resp.status() {
            reqwest::StatusCode::OK => Ok(()),
            code => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Expected 200, not {}", code),
            ))),
        }
    }
}
