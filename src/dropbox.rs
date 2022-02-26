use super::blog::PostInfo;
use super::mishaps::Mishap;
use super::settings::Settings;
use std::fs::File;
use std::path::Path;

pub fn show_auth_url(app_key: &str) -> String {
    format!("https://www.dropbox.com/oauth2/authorize?token_access_type=offline&client_id={}&response_type=code",
        app_key)
}

pub fn get_refresh_token(code: &str, app_key: &str, app_secret: &str) -> Result<String, Mishap> {
    Dropbox::code_for_token(code, app_key, app_secret).map(|ar| ar.refresh_token)
}

pub fn upload(refresh_token: &str, settings: &Settings, post: &PostInfo) -> Result<usize, Mishap> {
    let dropbox = Dropbox::from_refresh_token(refresh_token, settings)?;
    let _blog_response = dropbox.upload(&post.filename, &post.relative_path)?;

    for image in post.attachments.iter() {
        let _img_resp = dropbox.upload(&image.file, &image.relative_path)?;

        let thumb = &image.thumbnail;
        let _thumb_resp = dropbox.upload(&thumb.file, &thumb.relative_path)?;
    }

    Ok(1)
}

struct Dropbox {
    token: String,
    client: reqwest::blocking::Client,
}

#[derive(Debug, serde::Deserialize)]
struct AuthResponse {
    refresh_token: String,
}
#[derive(Debug, serde::Deserialize)]
struct AccessResponse {
    access_token: String,
}

impl Dropbox {
    // Convert a manually requested code into a refresh token (which we can then reuse).
    fn code_for_token(code: &str, app_key: &str, app_secret: &str) -> Result<AuthResponse, Mishap> {
        let client = reqwest::blocking::Client::new();
        let query = [("grant_type", "authorization_code"), ("code", code)];
        let response = client
            .post("https://api.dropbox.com/oauth2/token")
            .basic_auth(app_key, Some(app_secret))
            .query(&query)
            .send()?;

        let body = response.text()?;
        let ar: Result<AuthResponse, _> = serde_json::from_str(&body);

        match ar {
            Ok(ar) => Ok(ar),
            Err(err) => Err(Mishap::JsonContent(body, err.to_string())),
        }
    }

    // Use a long-lived refresh token to fetch a short-lived bearer access token
    fn access_token(
        refresh_token: &str,
        app_key: &str,
        app_secret: &str,
    ) -> Result<AccessResponse, Mishap> {
        let client = reqwest::blocking::Client::new();

        let query = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        let response = client
            .post("https://api.dropbox.com/oauth2/token")
            .basic_auth(app_key, Some(app_secret))
            .query(&query)
            .send()?;

        let body = response.text()?;
        let ar: Result<AccessResponse, _> = serde_json::from_str(&body);

        match ar {
            Ok(ar) => Ok(ar),
            Err(err) => Err(Mishap::JsonContent(body, err.to_string())),
        }
    }

    fn from_refresh_token(refresh_token: &str, settings: &Settings) -> Result<Dropbox, Mishap> {
        let token = Dropbox::access_token(
            refresh_token,
            &settings.dropbox_app_key,
            &settings.dropbox_app_secret,
        )
        .map(|ar| ar.access_token)?;

        Ok(Dropbox {
            token,
            client: reqwest::blocking::Client::new(),
        })
    }

    fn upload(&self, filename: &Path, dropbox_path: &str) -> Result<(), Mishap> {
        let file = File::open(filename)?;

        // E.g. { "path": "/media/2020/foo.jpg" }
        let slash = if dropbox_path.starts_with('/') {
            ""
        } else {
            "/"
        };

        let api_args = format!("{{\"path\":\"{}{}\"}}", slash, dropbox_path);

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
