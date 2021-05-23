# Creating a Release

[GitHub](https://github.com/orhun/gpg-tui/releases) and [crates.io](https://crates.io/crates/gpg-tui/) releases are automated via [GitHub actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Bump the version in [Cargo.toml](Cargo.toml) according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
2. Update [Cargo.lock](Cargo.lock) by building the project: `cargo build`
3. Ensure [CHANGELOG.md](CHANGELOG.md) is updated according to [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.
4. Commit and push the changes.
5. Check if [Continuous Integration](https://github.com/orhun/gpg-tui/actions) workflow is completed successfully.
6. Create a new tag: `git tag -s -a v[X.Y.Z]` ([signed](http://keys.gnupg.net/pks/lookup?search=0x1BC755D9FBD24068))
7. Push the tag: `git push --tags`
8. Wait for [Continuous Deployment](https://github.com/orhun/gpg-tui/actions) workflow to finish.
