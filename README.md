# WIP - DO NOT USE

Connects to an IMAP server and expects to find messages containing images.
Writes the images to disk, and creates a blog post markdown file.

# Usage

```
cargo run -- --help
```

# Example

```
export MEDIA_DIR=/tmp/media
export POSTS_DIR=/tmp/posts
mkdir -p $MEDIA_DIR
mkdir -p $POSTS_DIR

IMAP_PASSWORD=trustno1 cargo run -- --user=hello@example.org
```

# Linux binary

From MacOS:

```
docker pull clux/muslrust
docker run -v $PWD:/volume -t clux/muslrust cargo build --release
```

The binary will be:

```
target/x86_64-unknown-linux-musl/release/dropblog
```