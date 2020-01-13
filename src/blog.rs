use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Error;
use std::io::Write;

use super::conventions::FileConventions;

#[derive(Debug)]
pub struct PostInfo {
    pub slug: String,
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: DateTime<Utc>,
    pub attachments: Vec<Image>,
    pub conventions: FileConventions,
}

#[derive(Debug)]
pub struct Image {
    pub file: File,
    pub relative_url: String,
    pub mimetype: String,
}

impl PostInfo {
    pub fn new(
        slug: String,
        title: String,
        author: String,
        content: Option<String>,
        date: DateTime<Utc>,
        attachments: Vec<Image>,
        conventions: FileConventions,
    ) -> PostInfo {
        PostInfo {
            slug,
            title: title.trim().to_owned(),
            author: author.trim().to_owned(),
            content: content.map(|str| str.trim().to_owned()),
            date,
            attachments,
            conventions,
        }
    }
}

pub fn write(post: &PostInfo) -> Result<Vec<File>, Error> {
    println!("{:?}", &post);

    let markdown = File::create(post.conventions.post_filename())?;
    write!(&markdown, "{}", post_meta(&post))?;
    write!(&markdown, "\n\n")?;

    match &post.content {
        Some(text) => write!(&markdown, "{}\n\n", text)?,
        None => {}
    };

    for image in post.attachments.iter() {
        write!(&markdown, "![image]({})\n\n", image.relative_url)?;
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
