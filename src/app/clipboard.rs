use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Application property to copy to clipboard.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CopyType {
	/// Selected row of the keys table.
	TableRow(usize),
	/// Exported key.
	Key,
	/// ID of the selected key.
	KeyId,
	/// Fingerprint of the selected key.
	KeyFingerprint,
	/// User ID of the selected key.
	KeyUserId,
}

impl Display for CopyType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::TableRow(i) => format!("table row ({})", i),
				Self::Key => String::from("exported key"),
				Self::KeyId => String::from("key ID"),
				Self::KeyFingerprint => String::from("key fingerprint"),
				Self::KeyUserId => String::from("user ID"),
			}
		)
	}
}

impl FromStr for CopyType {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"row1" | "1" => Ok(Self::TableRow(1)),
			"row2" | "2" => Ok(Self::TableRow(2)),
			"key" => Ok(Self::Key),
			"key_id" | "id" => Ok(Self::KeyId),
			"key_fingerprint" | "fingerprint" => Ok(Self::KeyFingerprint),
			"key_user_id" | "user" => Ok(Self::KeyUserId),
			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_clipboard() {
		let copy_type = CopyType::from_str("row1").unwrap();
		assert_eq!(CopyType::TableRow(1), copy_type);
		assert_eq!(String::from("table row (1)"), copy_type.to_string());
		let copy_type = CopyType::from_str("key").unwrap();
		assert_eq!(CopyType::Key, copy_type);
		assert_eq!(String::from("exported key"), copy_type.to_string());
		let copy_type = CopyType::from_str("key_id").unwrap();
		assert_eq!(CopyType::KeyId, copy_type);
		assert_eq!(String::from("key ID"), copy_type.to_string());
		let copy_type = CopyType::from_str("key_fingerprint").unwrap();
		assert_eq!(CopyType::KeyFingerprint, copy_type);
		assert_eq!(String::from("key fingerprint"), copy_type.to_string());
		let copy_type = CopyType::from_str("key_user_id").unwrap();
		assert_eq!(CopyType::KeyUserId, copy_type);
		assert_eq!(String::from("user ID"), copy_type.to_string());
	}
}
