use gpgme::Key;
use std::ffi::CStr;

/// Representation of a key.
pub struct GpgKey {
	/// GPGME Key type.
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

	/// Returns the description of the primary key algorithm.
	pub fn get_algorithm(&self) -> String {
		match self.inner.primary_key() {
			Some(key) => {
				key.algorithm_name().unwrap_or_else(|_| String::from("[?]"))
			}
			None => String::from("[?]"),
		}
	}

	/// Returns the user IDs.
	pub fn get_user_ids(&self) -> Vec<String> {
		let mut user_ids = Vec::new();
		for user in self.inner.user_ids().into_iter() {
			user_ids.push(format!(
				"[{}] {}",
				user.validity(),
				self.unwrap_value(user.id_raw())
			));
		}
		user_ids
	}

	/// Returns the flags of subkeys.
	pub fn get_flags(&self) -> Vec<String> {
		let mut flags = Vec::new();
		for key in self.inner.subkeys().into_iter() {
			flags.push(format!(
				"{}{}{}{}",
				if key.can_sign() { "s" } else { "-" },
				if key.can_certify() { "c" } else { "-" },
				if key.can_encrypt() { "e" } else { "-" },
				if key.can_authenticate() { "a" } else { "-" },
			))
		}
		flags
	}

	/// Unwraps the given [`CStr`] typed value as [`String`].
	///
	/// [`CStr`]: std::ffi::CStr
	/// [`String`]: std::string::String
	fn unwrap_value(&self, value: Option<&'_ CStr>) -> String {
		match value {
			Some(v) => v.to_string_lossy().into_owned(),
			None => String::from("[?]"),
		}
	}
}
