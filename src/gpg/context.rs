use crate::gpg::config::GpgConfig;
use crate::gpg::key::{GpgKey, KeyType};
use anyhow::{anyhow, Result};
use gpgme::context::Keys;
use gpgme::{Context, ExportMode, Key, KeyListMode, PinentryMode, Protocol};
use std::fs::{self, File};
use std::io::Write;

/// A context for cryptographic operations.
pub struct GpgContext {
	/// GPGME context type.
	inner: Context,
	/// GPGME configuration manager.
	pub config: GpgConfig,
}

impl GpgContext {
	/// Constructs a new instance of `Context`.
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

	/// Exports the public/secret keys matching
	/// on or more of the specified patterns.
	///
	/// It saves the output to the specified/default path.
	/// See [`save_exported_keys`].
	///
	/// [`save_exported_keys`]: GpgContext::save_exported_keys
	pub fn export_keys(
		&mut self,
		key_type: KeyType,
		patterns: Option<Vec<String>>,
	) -> Result<String> {
		let mut output = Vec::new();
		let keys = self
			.get_keys_iter(key_type, patterns.clone())?
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
			Err(anyhow!("failed to export keys"))
		} else {
			self.save_exported_keys(
				output,
				key_type,
				patterns.unwrap_or_default(),
			)
		}
	}

	/// Saves the exported key to the specified/default path.
	///
	/// File name is determined via given patterns.
	/// [`output_dir`] is used for output directory.
	///
	/// [`output_dir`]: GpgConfig::output_dir
	fn save_exported_keys(
		&self,
		output: Vec<u8>,
		key_type: KeyType,
		patterns: Vec<String>,
	) -> Result<String> {
		let path = self.config.output_dir.join(format!(
			"{}_{}.gpg",
			key_type,
			if patterns.len() == 1 {
				&patterns[0]
			} else {
				"out"
			}
		));
		if !path.exists() {
			fs::create_dir_all(path.parent().expect("path has no parent"))?;
		}
		File::create(&path)?.write_all(&output)?;
		Ok(path.to_string_lossy().to_string())
	}
}
