use chrono::{DateTime, Utc};
use gpgme::{Key, Subkey, UserId};
use std::str::Utf8Error;

/// Representation of a key.
pub struct GpgKey {
	/// GPGME Key type.
	pub inner: Key,
}

impl GpgKey {
	/// Constructs a new instance of `GpgKey`.
	pub fn new(key: Key) -> Self {
		Self { inner: key }
	}

	/// Returns the ID of the primary key.
	pub fn get_id(&self) -> Result<&str, Option<Utf8Error>> {
		self.inner.id()
	}

	/// Returns the subkeys.
	pub fn get_subkeys(&self) -> Vec<Subkey<'_>> {
		self.inner.subkeys().collect()
	}

	/// Returns the user IDs.
	pub fn get_user_ids(&self) -> Vec<UserId> {
		self.inner.user_ids().collect()
	}

	/// Returns the flags of the given Subkey.
	///
	/// * `S`: sign
	/// * `C`: certify
	/// * `E`: encrypt
	/// * `A`: authenticate
	pub fn get_flags(key: Subkey<'_>) -> String {
		format!(
			"{}{}{}{}",
			if key.can_sign() { "s" } else { "-" },
			if key.can_certify() { "c" } else { "-" },
			if key.can_encrypt() { "e" } else { "-" },
			if key.can_authenticate() { "a" } else { "-" },
		)
	}

	/// Returns the time information of the given Subkey.
	///
	/// * creation time
	/// * expiration time
	/// * is the key expired/revoked/disabled/invalid/qualified?
	pub fn get_time(key: Subkey<'_>) -> String {
		format!(
			"({}){}{}{}{}{}{}",
			if let Some(date) = key.creation_time() {
				DateTime::<Utc>::from(date).format("%F").to_string()
			} else {
				String::from("[?]")
			},
			if let Some(date) = key.expiration_time() {
				DateTime::<Utc>::from(date).format(" ─> (%F)").to_string()
			} else {
				String::new()
			},
			if key.is_expired() { " [expired]" } else { "" },
			if key.is_revoked() { " [revoked]" } else { "" },
			if key.is_disabled() { " ⊗" } else { "" },
			if key.is_invalid() { " ✗" } else { "" },
			if key.is_qualified() { " ✓" } else { "" }
		)
	}
}
