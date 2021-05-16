use chrono::{DateTime, Utc};
use gpgme::{Subkey, UserIdSignature};

/// Returns the flags of the given subkey.
///
/// * `S`: sign
/// * `C`: certify
/// * `E`: encrypt
/// * `A`: authenticate
pub fn get_subkey_flags(subkey: Subkey) -> String {
	format!(
		"{}{}{}{}",
		if subkey.can_sign() { "s" } else { "-" },
		if subkey.can_certify() { "c" } else { "-" },
		if subkey.can_encrypt() { "e" } else { "-" },
		if subkey.can_authenticate() { "a" } else { "-" },
	)
}

/// Returns time information about the given subkey.
///
/// * creation time
/// * expiration time
/// * is the key expired/revoked/disabled/invalid/qualified?
pub fn get_subkey_time(subkey: Subkey, format: &str) -> String {
	format!(
		"({}){}{}{}{}{}{}",
		if let Some(date) = subkey.creation_time() {
			DateTime::<Utc>::from(date).format(format).to_string()
		} else {
			String::from("[?]")
		},
		if let Some(date) = subkey.expiration_time() {
			DateTime::<Utc>::from(date)
				.format(&format!(" ─> ({})", format))
				.to_string()
		} else {
			String::new()
		},
		if subkey.is_expired() { " [exp]" } else { "" },
		if subkey.is_revoked() { " [rev]" } else { "" },
		if subkey.is_disabled() { " [d]" } else { "" },
		if subkey.is_invalid() { " [i]" } else { "" },
		if subkey.is_qualified() { " [q]" } else { "" }
	)
}

/// Returns time information about the given signature.
///
/// * creation time
/// * expiration time
/// * is the signature expired/revoked/invalid/non-exportable?
pub fn get_signature_time(signature: UserIdSignature, format: &str) -> String {
	format!(
		"({}){}{}{}{}{}",
		if let Some(date) = signature.creation_time() {
			DateTime::<Utc>::from(date).format(format).to_string()
		} else {
			String::from("[?]")
		},
		if let Some(date) = signature.expiration_time() {
			DateTime::<Utc>::from(date)
				.format(&format!(" ─> ({})", format))
				.to_string()
		} else {
			String::new()
		},
		if signature.is_expired() { " [exp]" } else { "" },
		if signature.is_revocation() {
			" [rev]"
		} else {
			""
		},
		if signature.is_invalid() { " [i]" } else { "" },
		if !signature.is_exportable() {
			" [!x]"
		} else {
			""
		},
	)
}
