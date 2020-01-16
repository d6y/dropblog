# Drop blog:  email-to-dropbox blog posting

- Connects to an IMAP account and reads an email
- Extracts images and creates thumbnail version
- Turns the email content into a Jerky-style markdown blog post
- Uploads the post and images to Dropbox

# Example

```
export OUT_DIR=/tmp/blog
export MEDIA_PATH=media
export POSTS_PATH=_posts
mkdir -p $OUT_DIR/$MEDIA_PATH
mkdir -p $OUT_DIR/$POSTS_PATH

export DROPBOX_ACCESS_TOKEN=sup3rsekr3t
export IMAP_PASSWORD=trustno1 

dropblog --user=hello@example.org -e
 ```

 This will write files into `/tmp/blog/media` and `/tmp/blog/_post`
 and then try to upload them to Dropbox.

 # Output

```
$ cat /tmp/blog/_posts/2020-01-15-colours.md
---
title: |
    colours
author: Richard Dallaway
date: 2020-01-15 21:26
layout: post
comments: true
permalink: /colours
---

<a href="/media/2020/2020-01-15-colours-0.jpg">
<img src="/media/2020/2020-01-15-colours-0-thumb.jpg" width="500" height="375">
</a>
```

# Usage

```
$ ./target/release/dropblog --help
dropblog 1.0.0

USAGE:
    dropblog [FLAGS] [OPTIONS] --dropbox-access-token <dropbox-access-token> --media-path <media-path> --out-dir <out-dir> --password <password> --posts-path <posts-path> --user <user>

FLAGS:
    -e, --expunge         Archive the email after processing
        --show-outline    Outline the structure of the email as additional output
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
        --hostname <hostname>
            IMAP hostname to connect to [env: IMAP_HOSTNAME=]  [default: imap.gmail.com]

        --port <port>                                    IMAP port number [env: IMAP_PORT=]  [default: 993]
        --user <user>
            Email address (or user account) to check on the IMAP server [env: IMAP_USER=]

        --password <password>                            Password for authentication [env: IMAP_PASSWORD]
        --dropbox-access-token <dropbox-access-token>    Dropbox access token [env: DROPBOX_ACCESS_TOKEN]
        --out-dir <out-dir>                              Existing directory for writing content [env: OUT_DIR=]
        --media-path <media-path>                        Path into media relative to OUT_DIR [env: MEDIA_PATH=]
        --posts-path <posts-path>                        Path into posts relative to OUT_DIR [env: POSTS_PATH=]
    -w, --width <width>                                  Thumbnail width [default: 500]
```

# Building

```
$ cargo build --release
```

## Dependencies

[Imagemagik]: https://imagemagick.org/
[Dropbox access token]: https://blogs.dropbox.com/developers/2014/05/generate-an-access-token-for-your-own-account/

- You need `convert` and `identify` from [Imagemagik] on your path.
- You'll want a [Dropbox access token], and specifically an API token for an App Folder.

## Linux binary

From MacOS:

```
docker pull clux/muslrust
docker run -v $PWD:/volume -i -t clux/muslrust cargo build --release
```

The binary will be:

```
target/x86_64-unknown-linux-musl/release/dropblog
```