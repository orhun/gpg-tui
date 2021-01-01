//! GPGME wrappers for GnuPG actions.

/// Wrapper for [`Context`].
///
/// [`Context`]: gpgme::Context
pub mod context;

/// Wrapper for [`Key`].
///
/// [`Key`]: gpgme::Key
pub mod key;

/// Char representation for the [`None`] type.
///
/// [`None`]: std::option::Option::None
pub const NONE_CHAR: char = '?';
