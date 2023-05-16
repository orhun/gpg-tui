use std::fmt::{Display, Formatter, Result as FmtResult};
use clap::ValueEnum;

/// Application property to copy to clipboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Selection {
	/// One of the selected rows of the keys table
	#[clap(aliases = ["row_1", "row1", "1"])]
	Row1,
	/// The other selected row of the keys table
	#[clap(aliases = ["row_2", "row2", "2"])]
	Row2,
	/// Exported key.
	Key,
	/// ID of the selected key.
	#[clap(aliases = ["id", "key_id", "keyid"])]
	KeyId,
	/// Fingerprint of the selected key.
	#[clap(aliases = ["fingerprint", "key_fingerprint", "keyfingerprint", "fpr"])]
	KeyFingerprint,
	/// User ID of the selected key.
	#[clap(aliases = ["user", "user_id", "userid", "user-id", "key_user_id", "keyuserid"])]
	KeyUserId,
}

impl Display for Selection {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::Row1 => "table row (1)".to_string(),
				Self::Row2 => "table row (2)".to_string(),
				Self::Key => String::from("exported key"),
				Self::KeyId => String::from("key ID"),
				Self::KeyFingerprint => String::from("key fingerprint"),
				Self::KeyUserId => String::from("user ID"),
			}
		)
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_clipboard() -> Result<(), String> {
		let copy_type = Selection::from_str("row1", true)?;
		assert_eq!(Selection::Row1, copy_type);
		assert_eq!(String::from("table row (1)"), copy_type.to_string());
		let copy_type = Selection::from_str("row2", true)?;
		assert_eq!(Selection::Row2, copy_type);
		assert_eq!(String::from("table row (2)"), copy_type.to_string());
		let copy_type = Selection::from_str("key", true)?;
		assert_eq!(Selection::Key, copy_type);
		assert_eq!(String::from("exported key"), copy_type.to_string());
		let copy_type = Selection::from_str("key_id", true)?;
		assert_eq!(Selection::KeyId, copy_type);
		assert_eq!(String::from("key ID"), copy_type.to_string());
		let copy_type = Selection::from_str("key_fingerprint", true)?;
		assert_eq!(Selection::KeyFingerprint, copy_type);
		assert_eq!(String::from("key fingerprint"), copy_type.to_string());
		let copy_type = Selection::from_str("key_user_id", true)?;
		assert_eq!(Selection::KeyUserId, copy_type);
		assert_eq!(String::from("user ID"), copy_type.to_string());
		Ok(())
	}
}
