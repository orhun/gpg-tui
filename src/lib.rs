//! A terminal user interface for managing GnuPG keys.
#![warn(missing_docs, clippy::unwrap_used)]

pub mod app;
pub mod args;
pub mod gpg;
pub mod term;
pub mod widget;

/// Minimum required version of the GPGME library.
pub const GPGME_REQUIRED_VERSION: &str = "1.7.0";
