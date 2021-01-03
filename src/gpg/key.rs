use crate::gpg::subkey;
use gpgme::{Key, Subkey};
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

	/// Returns the fingerprint of the primary key.
	pub fn get_fingerprint(&self) -> String {
		Self::unwrap_value(self.inner.fingerprint_raw())
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
	pub fn get_user_ids(&self) -> Vec<String> {
		let mut user_ids = Vec::new();
		for user in self.inner.user_ids().into_iter() {
			user_ids.push(format!(
				"[{}] {}",
				user.validity(),
				Self::unwrap_value(user.id_raw())
			));
		}
		user_ids
	}

	/// Returns the information about subkeys.
	pub fn get_subkeys(&self) -> Vec<String> {
		let mut flags = Vec::new();
		let subkeys = self.inner.subkeys().skip(1).collect::<Vec<Subkey<'_>>>();
		for (i, key) in subkeys.iter().enumerate() {
			let time = subkey::get_time(*key);
			flags.push(format!(
				"[{}] {}/{}\n{}",
				subkey::get_flags(*key),
				key.algorithm_name().unwrap_or_else(|_| String::from("[?]")),
				Self::unwrap_value(key.fingerprint_raw()),
				format!(
					"{}      └─{}",
					if i != subkeys.len() - 1 { "|" } else { " " },
					time
				),
			))
		}
		flags
	}

	/// Unwraps the given [`CStr`] typed value as [`String`].
	///
	/// [`CStr`]: std::ffi::CStr
	/// [`String`]: std::string::String
	fn unwrap_value(value: Option<&'_ CStr>) -> String {
		match value {
			Some(v) => v.to_string_lossy().into_owned(),
			None => String::from("[?]"),
		}
	}
}
