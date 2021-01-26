//! GnuPG actions via GPGME.

/// Wrapper for [`Gpgme`].
///
/// [`Gpgme`]: gpgme::Gpgme
pub mod config;

/// Wrapper for [`Context`].
///
/// [`Context`]: gpgme::Context
pub mod context;

/// Wrapper for [`Key`].
///
/// [`Key`]: gpgme::Key
pub mod key;

/// Handler methods.
pub mod handler;
