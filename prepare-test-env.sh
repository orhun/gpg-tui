#!/usr/bin/env bash

test_dir="$HOME/.cache/gpg-tui/"
mkdir -p "$test_dir"
export GNUPGHOME="$test_dir"
openssl rand -base64 8 | gpg \
    --pinentry-mode loopback \
    --no-tty --passphrase-fd 0 \
    --quick-gen-key 'Test User <test@example.org>'
curl "https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x1bc755d9fbd24068" | \
    gpg --import
