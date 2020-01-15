use super::blog::PostInfo;
use super::settings::Settings;
use reqwest;
use std::error::Error;
use std::fs::File;

pub fn upload(settings: &Settings, post: &PostInfo) -> Result<(), Box<dyn Error>> {
    let api = "https://content.dropboxapi.com/2/files/upload";
    let file = File::open(&post.filename)?;

    let auth = format!("Bearer {}", &settings.dropbox_access_token);

    let api_args = format!( "{{\"path\":\"{}{}\"}}", "/", post.relative_path);

    let request = reqwest::blocking::Client::new()
        .post(api)
        .header("Content-Type", "application/octet-stream")
        .header("Authorization", auth)
        .header("Dropbox-API-Arg", api_args)
        .body(file);
    let response = request.send()?;
    println!("{:?}", response);

    Ok(())
}
