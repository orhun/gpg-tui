# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2021-06-09
### Fixed
- Expand tilde character to the home directory (fixes #22)

## [0.4.0] - 2021-06-07
### Changed
- Extract get_output_file from export_keys method
- Support exporting secret subkeys (#15)
- Update COMMANDS.md about export command
- Update README.md about exporting secret subkeys

## [0.3.0] - 2021-06-05
### Added
- Add packaging status badge to README.md
- Add Matrix room link to social media section in README.md

### Changed
- Display notations of the signatures (#8)
- Update README.md about Docker alias
- Update README.md about the format of notations
- Update the example notation in README.md
- Update the style of flags

### Fixed
- Run container process as unprivileged user

## [0.2.0] - 2021-06-02
### Added
- Add installation instructions for FreeBSD
- Add Homebrew instructions to README.md
- Add NetBSD instructions to README.md

### Changed
- Update table of contents
- Support xplr for file selection (closes #2)
- Use eprintln macro while printing errors
- Update README.md about the use of xplr

## [0.1.5] - 2021-05-31
### Added
- Add dependency installation instructions for Void Linux (#11)

### Changed
- Mention distribution-specific dependencies in README.md (#10)
- Update the formatting of requirements in README.md
- Update README.md about installation for Arch Linux

## [0.1.4] - 2021-05-29
### Removed
- Remove cargo-bloat workflow

## [0.1.3] - 2021-05-29
### Added
- Add Arch Linux installation instructions to README.md

### Changed
- Update the commands style in README.md
- Update link checker job to exclude AUR links
- Split audit workflow into two

## [0.1.2] - 2021-05-29
### Fixed
- Mark test_gpg_key as gpg-tests
- Test the detail commands if gpg-tests feature is enabled

## [0.1.1] - 2021-05-28
### Added
- Add release badges to README.md
- Add CD badge to README.md
- Add documentation badge to README.md

### Changed
- Use release flag while generating completions
- Update the release image
- Update documentation field in Cargo.toml
- Update Patreon badges in README.md
- Update CD workflow about the HTML syntax of release images
- Update FUNDING.yml

## [0.1.0] - 2021-05-28
Initial release.
