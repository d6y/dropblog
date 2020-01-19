use chrono::{DateTime, Utc};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileConventions {
    post_media_dir: PathBuf,
    post_media_url: String,
    permalink: String,
    stem: String,
    post_path: String,
    post_filename: PathBuf,
}

impl FileConventions {
    pub fn new(
        output_dir: &PathBuf,
        media_path: &str,
        posts_path: &str,
        date: &DateTime<Utc>,
        slug: &str,
    ) -> Result<FileConventions, Error> {
        // Media (i.e., images) will be in seperate yearly subdirectories:
        let mut post_media_dir = output_dir.clone();
        post_media_dir.push(&media_path);
        let year = date.format("%Y").to_string();
        post_media_dir.push(&year);

        if !post_media_dir.exists() {
            fs::create_dir(&post_media_dir)?;
        }

        // Corresponding URL:
        let post_media_url = format!("/{}/{}", &media_path, &year);

        // All filenames will start with this pattern:
        let stem = format!("{}-{}", date.format("%Y-%m-%d"), slug);

        // The blog post is a single filename:
        let mut post_filename = output_dir.clone();
        post_filename.push(&posts_path);
        post_filename.push(format!("{}.md", stem));

        let post_path = format!("{}/{}.md", &posts_path, stem);

        // The permalink is the slug, with no date information:
        let permalink = format!("/{}", &slug);

        Ok(FileConventions {
            post_media_dir,
            post_media_url,
            permalink,
            stem,
            post_path,
            post_filename,
        })
    }

    pub fn post_filename(&self) -> PathBuf {
        self.post_filename.clone()
    }

    pub fn post_path(&self) -> String {
        self.post_path.clone()
    }

    pub fn permalink(&self) -> String {
        self.permalink.clone()
    }

    pub fn attachment_filename(&self, count: usize) -> PathBuf {
        let mut image_file = self.post_media_dir.clone();
        image_file.push(format!("{}-{}.{}", self.stem, count, "jpg"));
        image_file
    }

    pub fn attachment_url(&self, count: usize) -> String {
        format!("{}/{}-{}.{}", &self.post_media_url, self.stem, count, "jpg")
    }

    pub fn attachment_thumb_path(&self, count: usize) -> PathBuf {
        let mut image_file = self.post_media_dir.clone();
        image_file.push(format!("{}-{}-thumb.{}", self.stem, count, "jpg"));
        image_file
    }

    pub fn attachment_thumb_url(&self, count: usize) -> String {
        format!(
            "{}/{}-{}-thumb.{}",
            &self.post_media_url, self.stem, count, "jpg"
        )
    }
}
