//! GnuPG actions via GPGME.

/// Wrapper for [`Context`].
///
/// [`Context`]: gpgme::Context
pub mod context;

/// Wrapper for [`Key`].
///
/// [`Key`]: gpgme::Key
pub mod key;

/// Handler methods for [`Subkey`].
///
/// [`Subkey`]: gpgme::Subkey
pub mod subkey;
