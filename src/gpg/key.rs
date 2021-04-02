use crate::gpg::handler;
use gpgme::{Key, Subkey, UserId, UserIdSignature};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Type of the key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyType {
	/// Public key.
	Public,
	/// Secret (private) key.
	Secret,
}

impl Display for KeyType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::Public => "pub",
				Self::Secret => "sec",
			}
		)
	}
}

impl FromStr for KeyType {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		for key_type in &[Self::Public, Self::Secret] {
			if key_type.to_string().matches(&s).count() >= 1 {
				return Ok(*key_type);
			}
		}
		Err(())
	}
}

/// Level of detail to show for key.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyDetail {
	/// Show only the primary key and user ID.
	Minimum = 0,
	/// Show all subkeys and user IDs.
	Standard = 1,
	/// Show signatures.
	Full = 2,
}

impl Display for KeyDetail {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", format!("{:?}", self).to_lowercase())
	}
}

impl FromStr for KeyDetail {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"1" | "min" | "minimum" => Ok(KeyDetail::Minimum),
			"2" | "standard" => Ok(KeyDetail::Standard),
			"3" | "full" => Ok(KeyDetail::Full),
			_ => Err(()),
		}
	}
}

impl KeyDetail {
	/// Increases the level of detail.
	pub fn increase(&mut self) {
		*self = match *self as i8 + 1 {
			1 => KeyDetail::Standard,
			2 => KeyDetail::Full,
			_ => KeyDetail::Minimum,
		}
	}
}

/// Representation of a key.
#[derive(Clone, Debug)]
pub struct GpgKey {
	/// GPGME Key type.
	inner: Key,
	/// Level of detail to show about key information.
	pub detail: KeyDetail,
}

impl From<Key> for GpgKey {
	fn from(key: Key) -> Self {
		Self {
			inner: key,
			detail: KeyDetail::Minimum,
		}
	}
}

impl GpgKey {
	/// Returns the key ID with '0x' prefix.
	pub fn get_id(&self) -> String {
		self.inner
			.id()
			.map_or(String::from("[?]"), |v| format!("0x{}", v))
	}

	/// Returns the key fingerprint.
	pub fn get_fingerprint(&self) -> String {
		self.inner
			.fingerprint()
			.map_or(String::from("[?]"), |v| v.to_string())
	}

	/// Returns the primary user of the key.
	pub fn get_user_id(&self) -> String {
		match self.inner.user_ids().next() {
			Some(user) => {
				user.id().map_or(String::from("[?]"), |v| v.to_string())
			}
			None => String::from("[?]"),
		}
	}

	/// Returns information about the subkeys.
	pub fn get_subkey_info(&self, truncate: bool) -> Vec<String> {
		let mut key_info = Vec::new();
		let subkeys = self.inner.subkeys().collect::<Vec<Subkey>>();
		for (i, subkey) in subkeys.iter().enumerate() {
			key_info.push(format!(
				"[{}] {}/{}",
				handler::get_subkey_flags(*subkey),
				subkey
					.algorithm_name()
					.unwrap_or_else(|_| { String::from("[?]") }),
				if truncate {
					subkey.id()
				} else {
					subkey.fingerprint()
				}
				.unwrap_or("[?]"),
			));
			if self.detail == KeyDetail::Minimum {
				break;
			}
			key_info.push(format!(
				"{}      └─{}",
				if i != subkeys.len() - 1 { "|" } else { " " },
				handler::get_subkey_time(
					*subkey,
					if truncate { "%Y" } else { "%F" }
				)
			));
		}
		key_info
	}

	/// Returns information about the users of the key.
	pub fn get_user_info(&self, truncate: bool) -> Vec<String> {
		let mut user_info = Vec::new();
		let user_ids = self.inner.user_ids().collect::<Vec<UserId>>();
		for (i, user) in user_ids.iter().enumerate() {
			user_info.push(format!(
				"{}[{}] {}",
				if i == 0 {
					""
				} else if i == user_ids.len() - 1 {
					" └─"
				} else {
					" ├─"
				},
				user.validity(),
				if truncate { user.email() } else { user.id() }
					.unwrap_or("[?]")
			));
			if self.detail == KeyDetail::Minimum {
				break;
			}
			if self.detail == KeyDetail::Full {
				user_info.extend(self.get_user_signatures(
					user,
					user_ids.len(),
					i,
					truncate,
				));
			}
		}
		user_info
	}

	/// Returns the signature information of an user.
	fn get_user_signatures(
		&self,
		user: &UserId,
		user_count: usize,
		user_index: usize,
		truncate: bool,
	) -> Vec<String> {
		let signatures = user.signatures().collect::<Vec<UserIdSignature>>();
		signatures
			.iter()
			.enumerate()
			.map(|(i, sig)| {
				format!(
					" {}  {}[{:x}] {} {}",
					if user_count == 1 {
						" "
					} else if user_index == user_count - 1 {
						"    "
					} else if user_index == 0 {
						"│"
					} else {
						"│   "
					},
					if i == signatures.len() - 1 {
						"└─"
					} else {
						"├─"
					},
					sig.cert_class(),
					if sig.signer_key_id() == self.inner.id() {
						String::from("selfsig")
					} else if truncate {
						sig.signer_key_id().unwrap_or("[?]").to_string()
					} else {
						format!(
							"{} {}",
							sig.signer_key_id().unwrap_or("[?]"),
							sig.signer_user_id().unwrap_or("[?]")
						)
					},
					handler::get_signature_time(
						*sig,
						if truncate { "%Y" } else { "%F" }
					)
				)
			})
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	use crate::gpg::config::GpgConfig;
	use crate::gpg::context::GpgContext;
	use anyhow::Result;
	use chrono::Utc;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_gpg_key() -> Result<()> {
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		let mut context = GpgContext::new(config)?;
		let mut keys = context.get_keys(KeyType::Public, None)?;
		if context.config.get_dir_info("homedir").ok()
			== dirs::cache_dir()
				.unwrap()
				.join(env!("CARGO_PKG_NAME"))
				.to_str()
		{
			let key = &mut keys[0];
			let date = Utc::now().format("%F").to_string();
			key.detail.increase();
			assert_eq!(KeyDetail::Standard, key.detail);
			assert_eq!(Ok(key.detail), KeyDetail::from_str("standard"));
			key.detail.increase();
			assert_eq!(KeyDetail::Full, key.detail);
			assert_eq!("full", key.detail.to_string());
			assert!(key.get_subkey_info(false).join("\n").contains(&date));
			assert!(key
				.get_subkey_info(true)
				.join("\n")
				.contains(&key.get_id().replace("0x", "")));
			assert!(key
				.get_subkey_info(false)
				.join("\n")
				.contains(&key.get_fingerprint()));
			assert_eq!(
				format!("[u] {}\n    └─[13] selfsig ({})", TEST_USER, date),
				key.get_user_info(false).join("\n")
			);
		}
		Ok(())
	}
}
