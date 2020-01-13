use std::io::Error;
use chrono::{DateTime, TimeZone, Utc};

pub struct FileConventions {

}

impl FileConventions {

    fn new(media_dir: &PathBuf, date: DateTime<Utc>, slug: &str) ->  Result<FileConventions, Error> {
        unimplemented!();
    }

    fn attachment_path(count: usize) -> PathBuf {
        unimplemented!();
    }

}