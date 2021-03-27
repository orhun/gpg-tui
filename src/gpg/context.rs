use crate::gpg::config::GpgConfig;
use crate::gpg::key::{GpgKey, KeyType};
use anyhow::{anyhow, Result};
use gpgme::context::Keys;
use gpgme::{
	Context, Data, ExportMode, Key, KeyListMode, PinentryMode, Protocol,
};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

/// A context for cryptographic operations.
pub struct GpgContext {
	/// GPGME context type.
	inner: Context,
	/// GPGME configuration manager.
	pub config: GpgConfig,
}

impl GpgContext {
	/// Constructs a new instance of `GpgContext`.
	pub fn new(config: GpgConfig) -> Result<Self> {
		let mut context = Context::from_protocol(Protocol::OpenPgp)?;
		context.set_key_list_mode(KeyListMode::LOCAL)?;
		context.set_key_list_mode(KeyListMode::SIGS)?;
		context.set_armor(config.armor);
		context.set_offline(false);
		context.set_pinentry_mode(PinentryMode::Ask)?;
		Ok(Self {
			inner: context,
			config,
		})
	}

	/// Applies the current configuration values to the context.
	pub fn apply_config(&mut self) {
		self.inner.set_armor(self.config.armor);
	}

	/// Returns the public/secret key with the specified ID.
	pub fn get_key(
		&mut self,
		key_type: KeyType,
		key_id: String,
	) -> Result<Key> {
		match key_type {
			KeyType::Public => Ok(self.inner.get_key(key_id)?),
			KeyType::Secret => Ok(self.inner.get_secret_key(key_id)?),
		}
	}

	/// Returns an iterator over a list of all public/secret keys
	/// matching one or more of the specified patterns.
	fn get_keys_iter(
		&mut self,
		key_type: KeyType,
		patterns: Option<Vec<String>>,
	) -> Result<Keys> {
		Ok(match key_type {
			KeyType::Public => {
				self.inner.find_keys(patterns.unwrap_or_default())?
			}
			KeyType::Secret => {
				self.inner.find_secret_keys(patterns.unwrap_or_default())?
			}
		})
	}

	/// Returns a list of all public/secret keys matching
	/// one or more of the specified patterns.
	pub fn get_keys(
		&mut self,
		key_type: KeyType,
		patterns: Option<Vec<String>>,
	) -> Result<Vec<GpgKey>> {
		Ok(self
			.get_keys_iter(key_type, patterns)?
			.filter_map(|key| key.ok())
			.map(GpgKey::from)
			.collect())
	}

	/// Returns the all available keys and their types in a HashMap.
	pub fn get_all_keys(&mut self) -> Result<HashMap<KeyType, Vec<GpgKey>>> {
		let mut keys = HashMap::new();
		keys.insert(KeyType::Public, self.get_keys(KeyType::Public, None)?);
		keys.insert(KeyType::Secret, self.get_keys(KeyType::Secret, None)?);
		Ok(keys)
	}

	/// Reads the keys from given files and adds them to the keyring.
	pub fn import_keys(&mut self, files: Vec<PathBuf>) -> Result<u32> {
		let mut imported_keys = 0;
		for file in files {
			let input = File::open(file)?;
			let mut data = Data::from_seekable_stream(input)?;
			imported_keys += self.inner.import(&mut data)?.imported();
		}
		Ok(imported_keys)
	}

	/// Returns the exported public/secret keys
	/// matching one or more of the specified patterns.
	pub fn get_exported_keys(
		&mut self,
		key_type: KeyType,
		patterns: Option<Vec<String>>,
	) -> Result<Vec<u8>> {
		let mut output = Vec::new();
		let keys = self
			.get_keys_iter(key_type, patterns)?
			.filter_map(|key| key.ok())
			.collect::<Vec<Key>>();
		self.inner.export_keys(
			&keys,
			if key_type == KeyType::Secret {
				ExportMode::SECRET
			} else {
				ExportMode::empty()
			},
			&mut output,
		)?;
		if output.is_empty() {
			Err(anyhow!("nothing exported"))
		} else {
			Ok(output)
		}
	}

	/// Exports keys and saves them to the specified/default path.
	///
	/// File name is determined via given patterns.
	/// [`output_dir`] is used for output directory.
	///
	/// [`output_dir`]: GpgConfig::output_dir
	pub fn export_keys(
		&mut self,
		key_type: KeyType,
		patterns: Option<Vec<String>>,
	) -> Result<String> {
		let output = self.get_exported_keys(key_type, patterns.clone())?;
		let patterns = patterns.unwrap_or_default();
		let path = self.config.output_dir.join(format!(
			"{}_{}.{}",
			key_type,
			if patterns.len() == 1 {
				&patterns[0]
			} else {
				"out"
			},
			if self.config.armor { "asc" } else { "pgp" }
		));
		if !path.exists() {
			fs::create_dir_all(path.parent().expect("path has no parent"))?;
		}
		File::create(&path)?.write_all(&output)?;
		Ok(path.to_string_lossy().to_string())
	}

	/// Sends the given key to the default keyserver.
	pub fn send_key(&mut self, key_id: String) -> Result<()> {
		let keys = self
			.get_keys_iter(KeyType::Public, Some(vec![key_id]))?
			.filter_map(|key| key.ok())
			.collect::<Vec<Key>>();
		if let Some(key) = keys.first() {
			self.inner
				.export_keys_extern(vec![key], ExportMode::EXTERN)
				.map_err(|e| anyhow!("failed to send key(s): {:?}", e))
		} else {
			Err(anyhow!("key not found"))
		}
	}

	/// Deletes the specified public/secret key.
	///
	/// Searches the keyring for finding the specified
	/// key ID for deleting it.
	pub fn delete_key(
		&mut self,
		key_type: KeyType,
		key_id: String,
	) -> Result<()> {
		match self.get_key(key_type, key_id) {
			Ok(key) => match key_type {
				KeyType::Public => {
					self.inner.delete_key(&key)?;
					Ok(())
				}
				KeyType::Secret => {
					self.inner.delete_secret_key(&key)?;
					Ok(())
				}
			},
			Err(e) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::args::Args;
	use pretty_assertions::assert_eq;
	use std::fs;
	#[test]
	fn test_gpg_context() -> Result<()> {
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		let mut context = GpgContext::new(config)?;
		assert_eq!(false, context.config.armor);
		context.config.armor = true;
		context.apply_config();
		assert_eq!(true, context.config.armor);
		context.get_keys_iter(KeyType::Public, None)?;
		context.get_keys(KeyType::Public, None)?;
		fs::remove_file(context.export_keys(KeyType::Public, None)?)?;
		Ok(())
	}
}
