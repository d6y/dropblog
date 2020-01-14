use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PostInfo {
    pub slug: String,
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: DateTime<Utc>,
    pub attachments: Vec<Image>,
    pub filename: PathBuf,
}

#[derive(Debug)]
pub struct Image {
    pub file: PathBuf,
    pub relative_url: String,
    pub mimetype: String,
    pub thumbnail: Thumbnail,
}

#[derive(Debug)]
pub struct Thumbnail {
    pub file: PathBuf,
    pub relative_url: String,
    pub width: u16,
    pub height: u16,
}

impl PostInfo {
    pub fn new(
        slug: String,
        title: String,
        author: String,
        content: Option<String>,
        date: DateTime<Utc>,
        attachments: Vec<Image>,
        filename: PathBuf,
    ) -> PostInfo {
        PostInfo {
            slug,
            title: title.trim().to_owned(),
            author: author.trim().to_owned(),
            content: content.map(|str| str.trim().to_owned()),
            date,
            attachments,
            filename,
        }
    }
}

pub fn write(post: &PostInfo) -> Result<Vec<File>, Error> {
    println!("{:?}", &post);

    let markdown = File::create(&post.filename)?;
    write!(&markdown, "{}", post_meta(&post))?;
    write!(&markdown, "\n\n")?;

    match &post.content {
        Some(text) => write!(&markdown, "{}\n\n", text)?,
        None => {}
    };

    for image in post.attachments.iter() {
        write!(
            &markdown,
            r#"<a href="{}"><img src="{}" width="{}" height="{}"></a>"#,
            image.relative_url,
            image.thumbnail.relative_url,
            image.thumbnail.width,
            image.thumbnail.height
        )?;
        write!(&markdown, "\n\n")?;
    }

    Ok(Vec::new())
}

fn post_meta(post: &PostInfo) -> String {
    format!(
        r#"
---
title: |
    {}
author: {}
date: {}
layout: post
comments: true
---"#,
        post.title,
        post.author,
        post.date.format("%Y-%m-%d %H:%M")
    )
}
