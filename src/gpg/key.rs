use crate::gpg::handler;
use clap::ValueEnum;
use gpgme::{Key, SignatureNotation, Subkey, UserId, UserIdSignature};
use serde::{Deserialize, Serialize};
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
#[derive(
	Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum KeyDetail {
	/// Show only the primary key and user ID.
	#[clap(aliases = ["min", "1"])]
	Minimum = 0,
	/// Show all subkeys and user IDs.
	#[clap(alias = "2")]
	Standard = 1,
	/// Show signatures.
	#[clap(alias = "3")]
	Full = 2,
}

impl Default for KeyDetail {
	fn default() -> Self {
		Self::Minimum
	}
}

impl Display for KeyDetail {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", format!("{self:?}").to_lowercase())
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

impl GpgKey {
	/// Constructs a new instance of `GpgKey`.
	pub fn new(key: Key, detail: KeyDetail) -> Self {
		Self { inner: key, detail }
	}

	/// Returns the key ID with '0x' prefix.
	pub fn get_id(&self) -> String {
		self.inner
			.id()
			.map_or(String::from("[?]"), |v| format!("0x{v}"))
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
	pub fn get_subkey_info(
		&self,
		default_key: Option<&str>,
		truncate: bool,
	) -> Vec<String> {
		let mut key_info = Vec::new();
		let subkeys = self.inner.subkeys().collect::<Vec<Subkey>>();
		for (i, subkey) in subkeys.iter().enumerate() {
			key_info.push(format!(
				"[{}]{}{}/{}",
				handler::get_subkey_flags(*subkey),
				if default_key.map(|v| v.trim_start_matches("0x"))
					== subkey.id().ok()
				{
					"*"
				} else {
					" "
				},
				if let Ok(algorithm_name) = subkey.algorithm_name() {
					if algorithm_name.len() == 7 {
						algorithm_name
					} else {
						String::from("unrecog")
					}
				} else {
					String::from("unknown")
				},
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
		let mut user_signatures = Vec::new();
		let signatures = user.signatures().collect::<Vec<UserIdSignature>>();
		for (i, sig) in signatures.iter().enumerate() {
			let notations = sig.notations().collect::<Vec<SignatureNotation>>();
			let padding = if user_count == 1 {
				" "
			} else if user_index == user_count - 1 {
				"    "
			} else if user_index == 0 {
				"│"
			} else {
				"│   "
			};
			user_signatures.push(format!(
				" {}  {}[{:x}] {} {}",
				padding,
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
					let user_id = sig.signer_user_id().unwrap_or("[-]");
					format!(
						"{} {}",
						sig.signer_key_id().unwrap_or("[?]"),
						if user_id.is_empty() { "[?]" } else { user_id }
					)
				},
				handler::get_signature_time(
					*sig,
					if truncate { "%Y" } else { "%F" }
				)
			));
			if !notations.is_empty() {
				user_signatures.extend(self.get_signature_notations(
					notations,
					format!(" {padding}  "),
					signatures.len(),
					i,
				));
			}
		}
		user_signatures
	}

	/// Returns the notations of the given signature.
	fn get_signature_notations(
		&self,
		notations: Vec<SignatureNotation>,
		padding: String,
		sig_count: usize,
		sig_index: usize,
	) -> Vec<String> {
		notations
			.iter()
			.enumerate()
			.map(|(i, notation)| {
				format!(
					"{}{}  {}[{}] {}={}",
					padding,
					if sig_index == sig_count - 1 {
						" "
					} else {
						"│"
					},
					if i == notations.len() - 1 {
						"└─"
					} else {
						"├─"
					},
					if notation.is_critical() {
						"!"
					} else if notation.is_human_readable() {
						"h"
					} else {
						"?"
					},
					notation.name().unwrap_or("?"),
					notation.value().unwrap_or("?"),
				)
			})
			.collect()
	}
}

#[cfg(feature = "gpg-tests")]
#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	use crate::gpg::config::GpgConfig;
	use crate::gpg::context::GpgContext;
	use anyhow::Result;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_gpg_key() -> Result<()> {
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		let mut context = GpgContext::new(config)?;
		let mut keys =
			context.get_keys(KeyType::Public, None, KeyDetail::default())?;
		let key = &mut keys[0];
		key.detail.increase();
		assert_eq!(KeyDetail::Standard, key.detail);
		assert_eq!(
			Ok(key.detail),
			<KeyDetail as std::str::FromStr>::from_str("standard")
		);
		key.detail.increase();
		assert_eq!(KeyDetail::Full, key.detail);
		assert_eq!("full", key.detail.to_string());
		assert!(key
			.get_subkey_info(Some(""), true)
			.join("\n")
			.contains(&key.get_id().replace("0x", "")));
		assert!(key
			.get_subkey_info(Some(""), false)
			.join("\n")
			.contains(&key.get_fingerprint()));
		assert!(key
			.get_user_info(false)
			.join("\n")
			.contains(&key.get_user_id()));
		Ok(())
	}
}
