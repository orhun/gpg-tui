use crate::gpg::subkey;
use gpgme::{Key, Subkey, UserId};

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

	/// Returns the fingerprint of the primary key.
	pub fn get_fingerprint(&self) -> String {
		match self.inner.fingerprint_raw() {
			Some(v) => v.to_string_lossy().into_owned(),
			None => String::from("[?]"),
		}
	}

	/// Returns the description of the primary keys algorithm.
	pub fn get_algorithm(&self) -> String {
		match self.inner.primary_key() {
			Some(key) => {
				key.algorithm_name().unwrap_or_else(|_| String::from("[?]"))
			}
			None => String::from("[?]"),
		}
	}

	/// Returns the flags of the primary key.
	pub fn get_flags(&self) -> String {
		match self.inner.primary_key() {
			Some(key) => subkey::get_flags(key),
			None => String::from("[?]"),
		}
	}

	/// Returns the time information of the primary key.
	pub fn get_time(&self) -> String {
		match self.inner.primary_key() {
			Some(key) => subkey::get_time(key),
			None => String::from("[?]"),
		}
	}

	/// Returns the user IDs.
	pub fn get_user_ids(&self) -> Vec<UserId> {
		self.inner.user_ids().collect()
	}

	/// Returns the subkeys without primary key.
	pub fn get_subkeys(&self) -> Vec<Subkey<'_>> {
		self.inner.subkeys().skip(1).collect()
	}
}
