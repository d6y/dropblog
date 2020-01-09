use chrono::{DateTime, Utc};
use std::fs::File;
// use std::io::Write;
use std::io::Error;

use super::settings::Settings;

#[derive(Debug)]
pub struct PostInfo {
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: DateTime<Utc>,
    pub attachments: Vec<Image>,
}

#[derive(Debug)]
pub struct Image {
    pub file: File,
    pub mimetype: String,
}

pub fn write(settings: &Settings, post: &PostInfo) -> Result<Vec<File>, Error> {
    println!("{}", post_meta(&post));

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
