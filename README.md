# Drop blog: email-to-dropbox blog posting

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

export DROPBOX_REFRESH_TOKEN=sup3rsekr3t
export IMAP_PASSWORD=trustno1

dropblog --user=hello@example.org -e
```

This will write files into `/tmp/blog/media` and `/tmp/blog/_post`
and then try to upload them to Dropbox.

## Output

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
% ./target/release/dropblog --help
dropblog 2.0.0

USAGE:
    dropblog [FLAGS] [OPTIONS] --dropbox-app-key <dropbox-app-key> --dropbox-app-secret <dropbox-app-secret> --media-path <media-path> --out-dir <out-dir> --password <password> --posts-path <posts-path> --user <user>

FLAGS:
    -e, --expunge         Archive the email after processing
        --show-outline    Outline the structure of the email as additional output
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
        --hostname <hostname>
            IMAP hostname to connect to [env: IMAP_HOSTNAME=]  [default: imap.gmail.com]

        --port <port>                                      IMAP port number [env: IMAP_PORT=]  [default: 993]
        --user <user>
            Email address (or user account) to check on the IMAP server [env: IMAP_USER=]

        --password <password>                              Password for authentication [env: IMAP_PASSWORD]
        --dropbox-refresh-token <dropbox-refresh-token>    Dropbox refresh token [env: DROPBOX_REFRESH_TOKEN]
        --dropbox-code <dropbox-code>
            Dropbox code (supplied by yser, used once to fetch a refresh token) [env: DROPBOX_CODE]

        --dropbox-app-key <dropbox-app-key>
            Dropbox app key (also called client ID) [env: DROPBOX_APP_KEY=]

        --dropbox-app-secret <dropbox-app-secret>
            Dropbox app secret (also called client secret) [env: DROPBOX_APP_SECRET]

        --out-dir <out-dir>                                Existing directory for writing content [env: OUT_DIR=]
        --media-path <media-path>                          Path into media relative to OUT_DIR [env: MEDIA_PATH=]
        --posts-path <posts-path>                          Path into posts relative to OUT_DIR [env: POSTS_PATH=]
    -w, --width <width>                                    Thumbnail width [default: 500]

```

# Set up

## Create a dropbox app

You need to create a Dropbox app for yourself, and note the app key and app secret (also called client id and client secret).

## Dropbox offline OAuth dance

You need to generate a long-lived refresh token. This is described in "Implement refresh tokens" at <https://dropbox.tech/developers/migrating-app-permissions-and-access-tokens>.


The sequence of commands (once you have an app key and secret) is:

1. Set up common environment variables (see `cargo run -- --help` for how to pass them as command line arguments):

    ```
    export OUT_DIR=tmp
    export MEDIA_PATH=media
    export POSTS_PATH=_posts

    export IMAP_USER=user@domain.com
    export IMAP_PASSWORD=???

    export DROPBOX_APP_KEY=???
    export DROPBOX_APP_SECRET=???
    ```

2. Run `cargo run` to get the URL for authenication. Follow this URL in a browser, and note the code Dropbox gives you.

3. Run again supplying the code Dropbox gave you:

    ```
    cargo run -- --dropbox-code=CODEHERE
    ```

4. The above gives you a refresh code. You can use this to write posts to Dropbox. E.g.,

    ```
    cargo run -- --dropbox-refresh-token=TOKEN_HERE
    ```

...and that's the command you can run repeatedly (e.g., in a cron job).


## Dependencies

[imagemagik]: https://imagemagick.org/

- You need `convert`, `mogrify` and `identify` from [Imagemagik] on your path.


# Build notes

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
