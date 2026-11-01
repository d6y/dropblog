#!/bin/sh
# Home Assistant app entry point.
# Each line received on stdin (sent by the hassio.addon_stdin action from an
# automation) names a blog from the options; dropblog is run once for that blog.
set -eu

OPTIONS="${OPTIONS:-/data/options.json}"

opt() { jq -r ".$1" "$OPTIONS"; }

export IMAP_HOSTNAME="$(opt imap_hostname)"
export IMAP_PORT="$(opt imap_port)"
export MEDIA_PATH="$(opt media_path)"
export POSTS_PATH="$(opt posts_path)"
export IMAGE_WIDTH="$(opt image_width)"
export EXPURGE="$(opt expunge)"
export TZ="$(opt timezone)"
export RUST_LOG="$(opt log_level)"

blog() { jq -c --arg name "$1" '.blogs[] | select(.name == $name)' "$OPTIONS"; }

# hassio.addon_stdin sends the input JSON-encoded (e.g. "jane" with quotes)
unquote() { printf '%s' "$1" | jq -er 'strings' 2>/dev/null || printf '%s' "$1"; }

run_blog() (
  field() { printf '%s' "$1" | jq -r ".$2"; }
  export IMAP_USER="$(field "$1" imap_user)"
  export IMAP_PASSWORD="$(field "$1" imap_password)"
  export MAILBOX="$(field "$1" mailbox)"
  export DROPBOX_APP_KEY="$(field "$1" dropbox_app_key)"
  export DROPBOX_APP_SECRET="$(field "$1" dropbox_app_secret)"
  export DROPBOX_REFRESH_TOKEN="$(field "$1" dropbox_refresh_token)"
  dropblog
)

echo "dropblog: waiting for triggers"
while read -r line; do
  name="$(unquote "$line")"
  settings="$(blog "$name")"
  if [ -z "$settings" ]; then
    echo "dropblog: no blog named '$name'"
    continue
  fi
  echo "dropblog: triggered for $name"
  run_blog "$settings" || echo "dropblog: failed for $name (exit $?)"
done
