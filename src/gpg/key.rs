use gpgme::Key;
use std::ffi::CStr;

/// Char representation for the [`None`] type.
///
/// [`None`]: std::option::Option::None
const NONE_CHAR: char = '-';

/// Representation of a key.
pub struct GpgKey {
	/// GPGME key type.
	inner: Key,
}

impl GpgKey {
	/// Constructs a new instance of `GpgKey`.
	pub fn new(key: Key) -> Self {
		Self { inner: key }
	}

	/// Returns the key fingerprint.
	pub fn get_fingerprint(&self) -> String {
		self.unwrap_value(self.inner.fingerprint_raw())
	}

	/// Returns the ID of the primary user.
	pub fn get_primary_user_id(&self) -> String {
		match self.inner.user_ids().into_iter().next() {
			Some(user) => self.unwrap_value(user.id_raw()),
			None => String::from(NONE_CHAR),
		}
	}

	/// Unwraps the given [`CStr`] typed value as [`String`].
	///
	/// [`CStr`]: std::ffi::CStr
	/// [`String`]: std::string::String
	fn unwrap_value<'a>(&self, value: Option<&'a CStr>) -> String {
		match value {
			Some(v) => v.to_string_lossy().into_owned(),
			None => String::from(NONE_CHAR),
		}
	}
}
