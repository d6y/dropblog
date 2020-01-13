use chrono::{DateTime, Utc};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct FileConventions {
    post_media_dir: PathBuf,
    post_media_url: String,
    stem: String,
}

impl FileConventions {
    pub fn new(
        media_root: &PathBuf,
        date: &DateTime<Utc>,
        slug: &str,
    ) -> Result<FileConventions, Error> {
        // Media (i.e., images) will be in seperate yearly subdirectories:
        let mut post_media_dir = media_root.clone();
        let year = date.format("%Y").to_string();
        post_media_dir.push(&year);

        if !post_media_dir.exists() {
            fs::create_dir(&post_media_dir)?;
        }

        // Corresponding URL:
        let post_media_url = format!("/media/{}", &year);

        // All filenames will start with this pattern:
        let stem = format!("{}-{}", date.format("%Y-%m-%d"), slug);

        Ok(FileConventions {
            post_media_dir,
            post_media_url,
            stem,
        })
    }

    // TODO: take mime type for file extension
    pub fn attachment_path(&self, count: usize) -> PathBuf {
        let mut image_file = self.post_media_dir.clone();
        image_file.push(format!("{}-{}.{}", self.stem, count, "jpg"));
        image_file
    }

    pub fn attachment_url(&self, count: usize) -> String {
        format!("{}/{}-{}.{}", &self.post_media_url, self.stem, count, "jpg")
    }
}
