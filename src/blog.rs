use super::mishaps::Mishap;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PostInfo {
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: DateTime<Utc>,
    pub permalink: String,
    pub attachments: Vec<Image>,
    pub relative_path: String,
    pub filename: PathBuf,
}

#[derive(Debug)]
pub struct Image {
    pub file: PathBuf,
    pub relative_path: String,
    pub mimetype: String,
    pub thumbnail: Thumbnail,
}

#[derive(Debug)]
pub struct Thumbnail {
    pub file: PathBuf,
    pub relative_path: String,
    pub width: u16,
    pub height: u16,
}

impl PostInfo {
    pub fn new(
        title: String,
        author: String,
        content: Option<String>,
        date: DateTime<Utc>,
        permalink: String,
        attachments: Vec<Image>,
        relative_path: String,
        filename: PathBuf,
    ) -> PostInfo {
        PostInfo {
            title: title.trim().to_owned(),
            author: author.trim().to_owned(),
            content: content.map(|str| str.trim().to_owned()),
            date,
            permalink,
            attachments,
            relative_path,
            filename,
        }
    }
}

const JEKYL_SITE_URL: &str = "{{ site.url }}";

pub fn write(post: &PostInfo) -> Result<&PostInfo, Mishap> {
    let markdown = File::create(&post.filename)?;
    write!(&markdown, "{}", post_meta(post))?;
    write!(&markdown, "\n\n")?;

    match &post.content {
        Some(text) => write!(&markdown, "{}\n\n", text)?,
        None => {}
    };

    for image in post.attachments.iter() {
        write!(&markdown, "{}", image_to_markdown(image))?;
        write!(&markdown, "\n\n")?;
    }

    Ok(post)
}

fn image_to_markdown(image: &Image) -> String {
    format!(
        r#"<a href="{}{}"><img src="{}{}" width="{}" height="{}"></a>"#,
        &JEKYL_SITE_URL,
        image.relative_path,
        &JEKYL_SITE_URL,
        image.thumbnail.relative_path,
        image.thumbnail.width,
        image.thumbnail.height
    )
}

fn post_meta(post: &PostInfo) -> String {
    format!(
        r#"---
title: |
    {}
author: {}
date: {}
permalink: {}
layout: post
comments: true
---"#,
        post.title,
        post.author,
        post.date.format("%Y-%m-%d %H:%M"),
        post.permalink,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_markdown() {
        let img = Image {
            file: PathBuf::new(),
            relative_path: "/foo.jpg".to_string(),
            mimetype: "image/jpg".to_string(),
            thumbnail: Thumbnail {
                file: PathBuf::new(),
                relative_path: "/foo_thumb.jpg".to_string(),
                width: 640,
                height: 320,
            },
        };

        let markdown = r#"<a href="{{ site.url }}/foo.jpg"><img src="{{ site.url }}/foo_thumb.jpg" width="640" height="320"></a>"#;

        assert_eq!(markdown, image_to_markdown(&img));
    }
}
