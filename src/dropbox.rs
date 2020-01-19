use super::blog::PostInfo;
use super::mishaps::Mishap;
use super::settings::Settings;
use reqwest;
use std::fs::File;
use std::path::PathBuf;

pub fn upload(settings: &Settings, post: &PostInfo) -> Result<(), Mishap> {
    let dropbox = Dropbox::new(&settings.dropbox_access_token);
    let _blog_response = dbg!(dropbox.upload(&post.filename, &post.relative_path)?);

    for image in post.attachments.iter() {
        let _img_resp = dbg!(dropbox.upload(&image.file, &image.relative_path)?);

        let thumb = &image.thumbnail;
        let _thumb_resp = dbg!(dropbox.upload(&thumb.file, &thumb.relative_path)?);
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

    fn upload(&self, filename: &PathBuf, dropbox_path: &String) -> Result<(), Mishap> {
        let file = File::open(filename)?;

        // E.g. { "path": "/media/2020/foo.jpg" }
        let slash = if dropbox_path.starts_with("/") {
            ""
        } else {
            "/"
        };
        let api_args = dbg!(format!("{{\"path\":\"{}{}\"}}", slash, dropbox_path));

        let request = self
            .client
            .post("https://content.dropboxapi.com/2/files/upload")
            .bearer_auth(&self.token)
            .header("Content-Type", "application/octet-stream")
            .header("Dropbox-API-Arg", &api_args)
            .body(file);

        let resp = request.send()?;
        match resp.status() {
            reqwest::StatusCode::OK => Ok(()),
            code => Err(Mishap::UploadRejected(code)),
        }
    }
}
