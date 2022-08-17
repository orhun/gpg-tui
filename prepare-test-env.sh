#!/usr/bin/env bash

set -e

test_dir="$HOME/.cache/gpg-tui"
for dir in "$test_dir" "$test_dir/private-keys-v1.d"; do
    mkdir -p "$dir"
    chmod 700 "$dir"
done
export GNUPGHOME="$test_dir"
openssl rand -base64 8 | gpg \
    --pinentry-mode loopback \
    --no-tty --passphrase-fd 0 \
    --quick-gen-key 'Test User <test@example.org>'
curl "https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x1bc755d9fbd24068" | \
    gpg --import
