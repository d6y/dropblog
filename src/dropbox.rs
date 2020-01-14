/*
curl -X POST https://content.dropboxapi.com/2/files/upload \
  --header 'Authorization: Bearer ???' \
  --header 'Content-Type: application/octet-stream' \
  --header 'Dropbox-API-Arg: {"path":"/Apps/bibble/foo.jpg"}' \
  --data-binary @'EMDKZnUWsAQV9mg.jpg'
*/

use super::blog::PostInfo;
use super::settings::Settings;
use reqwest;
use std::error::Error;
use std::fs::File;

pub fn upload(settings: &Settings, post: &PostInfo) -> Result<(), Box<dyn Error>> {
    let api = "https://content.dropboxapi.com/2/files/upload";
    let file = File::open(&post.filename)?;

    let auth = format!("Bearer {}", &settings.dropbox_access_token);

    let optional_separater = if settings.dropbox_root.ends_with("/") {
        ""
    } else {
        "/"
    };

    let api_args = format!(
        "{{\"path\":\"{}{}{}\"}}",
        &settings.dropbox_root, optional_separater, post.relative_path
    );

    let request = reqwest::blocking::Client::new()
        .post(api)
        .header("Content-Type", "application/octet-stream")
        .header("Authorization", auth)
        .header("Dropbox-API-Arg", api_args)
        .body(file);
    println!("{:?}", request);
    let response = request.send()?;
    println!("{:?}", response);

    Ok(())
}
