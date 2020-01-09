use chrono::{DateTime, Utc};
use std::fs::File;
// use std::io::Write;

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

pub fn write(post: &PostInfo) {}
