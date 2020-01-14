/*
curl -X POST https://content.dropboxapi.com/2/files/upload \
  --header 'Authorization: Bearer ???' \
  --header 'Content-Type: application/octet-stream' \
  --header 'Dropbox-API-Arg: {"path":"/Apps/bibble/foo.jpg"}' \
  --data-binary @'EMDKZnUWsAQV9mg.jpg'
*/

use super::blog::PostInfo;
use super::settings::Settings;

pub fn upload(settings: &Settings, post: &PostInfo) -> Result<(), std::io::Error> {
    Ok(())
}
