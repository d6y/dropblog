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

<a href="{{ site.url }}/media/2020/2020-01-15-colours-0.jpg">
<img src="{{ site.url }}/media/2020/2020-01-15-colours-0-thumb.jpg" width="500" height="375">
</a>
```

# Usage

```
% ./target/release/dropblog --help
dropblog 2.0.2

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
    -m, --mailbox <mailbox>                                 [default: INBOX]
        --dropbox-refresh-token <dropbox-refresh-token>    Dropbox refresh token [env: DROPBOX_REFRESH_TOKEN]
        --dropbox-code <dropbox-code>
            Dropbox code (supplied by user, used once to fetch a refresh token) [env: DROPBOX_CODE]

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

[console]: https://www.dropbox.com/developers/apps

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

2. Run `cargo run` to get the URL for authentication. Follow this URL in a browser, and note the code Dropbox gives you.

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

## Docker build

```
docker build -t dropblog .
docker run -it --rm --name running-dropblog dropblog
```

## Linux binary

As we use rustls, the following is the fastest way to build for Linux on macOS:

```
cat .envrc
export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc
export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc

cargo build --target x86_64-unknown-linux-gnu --release
```

The binary will be in `target/x86_64-unknown-linux-gnu/release/`.

## Alternative Linux binary 

As a backup the following is also possible:

```
docker pull clux/muslrust
docker run -v $PWD:/volume -i -t clux/muslrust cargo build --release
```

The binary will be:

```
target/x86_64-unknown-linux-musl/release/dropblog
```

# Running as a Home Assistant app

The `homeassistant/` folder contains a local app for Home Assistant.

The app stays running. Each time an automation sends it a blog name (one line of input), it runs `dropblog` once for that blog.
Each run processes one email.
Several blogs (each with its own mailbox and Dropbox app) can share the one app.

Install steps from macOS:

## 1. Expose the Home Assistant `addons` folder to your Mac

If you have already done this for another app (e.g., dogpost), skip to step 2.

1. In Home Assistant go to _Settings > Apps_, and install **Samba share**.
2. On the app's _Configuration_ tab set a username and password, save, then **Start** it.
3. On the Mac, in Finder choose _Go > Connect to Server_ (⌘K) and enter `smb://homeassistant.local/addons` (or use IP address if that doesn't work).
   Log in with the Samba username and password. The share mounts at `/Volumes/addons`.
4. Check you can access the addons from your terminal: `ls /Volumes/addons`. If you can't check _System Settings > Privacy & Security > Files and Folders_ and for your terminal turn on "Network Volumes".

## 2. Copy the app across

From the root of this repository:

```
./homeassistant/install.sh
```

This copies the app files plus `Cargo.toml`, `Cargo.lock` and `src/` into `/Volumes/addons/dropblog`.
Pass a different folder as an argument if the share is mounted elsewhere.

## 3. Install the app

In Home Assistant go to _Settings > Apps_ and click **Install App**. Select `dropblog` and then press **Install**.

The Pi will now compile the code and this will take a while.

## 4. Configure

Each blog needs a Dropbox refresh token.
If you don't have one, get it on your Mac by following [Dropbox offline OAuth dance](#dropbox-offline-oauth-dance) above.

On the app's **Configuration** tab set the shared settings (`media_path`, `posts_path`, `expunge`, etc.).
`log_level` is passed through as `RUST_LOG`, so it can be a filter such as `DEBUG,rustls=off`.

Add one entry to `blogs` per blog. In YAML mode (the tab's "⋮" menu, _Edit in YAML_) that looks like:

```yaml
blogs:
  - name: richard
    imap_user: someone@example.com
    imap_password: ...
    mailbox: richard
    dropbox_app_key: ...
    dropbox_app_secret: ...
    dropbox_refresh_token: ...
```

Save, then **Start** the app. The **Log** tab should show `dropblog: waiting for triggers`.

## 5. Trigger it from an automation

Use the `hassio.addon_stdin` action (the app's id is `local_dropblog`), with the blog's `name` as the input.
For example:

```yaml
alias: Dropblog on new email
triggers:
  - trigger: event
    event_type: blog_email_tagged
    event_data:
      folder: richard
actions:
  - action: hassio.addon_stdin
    data:
      addon: local_dropblog
      input: richard
mode: queued
```

To test by hand, go to _Developer tools > Actions_, run `hassio.addon_stdin` with the data above, and watch the app's **Log** tab.
An input that doesn't match a blog name is logged as `dropblog: no blog named '...'`.

## Updating

After changing the code, run `./homeassistant/install.sh` again, press the "refresh" icon on the Apps page in HA, then select **Update** on the app's page.

After changing `homeassistant/dropblog/config.yaml` (e.g., the settings), Rebuild is not enough:

1. Increase `version` in `config.yaml`.
2. Run `./homeassistant/install.sh`.
3. In the Home Assistant _Terminal_:

   ```
   ha store reload
   ha apps update local_dropblog
   ```
