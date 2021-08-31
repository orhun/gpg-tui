//! Configuration file parser.

use crate::args::Args;
use crate::widget::style::Color;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

/// Application configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
	/// General configuration.
	pub general: GeneralConfig,
	/// GnuPG configuration.
	pub gpg: GpgConfig,
}

/// General configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GeneralConfig {
	/// [`Args::splash`]
	pub splash: bool,
	/// [`Args::tick_rate`]
	pub tick_rate: u64,
	/// [`Args::color`]
	pub color: String,
	/// [`Args::style`]
	pub style: String,
}

/// GnuPG configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GpgConfig {
	/// [`Args::armor`]
	pub armor: bool,
	/// [`Args::homedir`]
	pub homedir: Option<String>,
	/// [`Args::outdir`]
	pub outdir: Option<String>,
	/// [`Args::default_key`]
	pub default_key: Option<String>,
}

impl Config {
	/// Checks the possible locations for the configuration file.
	///
	/// - `<config_dir>/gpg-tui.toml`
	/// - `<config_dir>/gpg-tui/gpg-tui.toml`
	/// - `<config_dir>/gpg-tui/config`
	///
	/// Returns the path if the configuration file is found.
	pub fn get_default_location() -> Option<String> {
		if let Some(config_dir) = dirs_next::config_dir() {
			let file_name = format!("{}.toml", env!("CARGO_PKG_NAME"));
			for config_file in vec![
				config_dir.join(&file_name),
				config_dir.join(env!("CARGO_PKG_NAME")).join(&file_name),
				config_dir.join(env!("CARGO_PKG_NAME")).join("config"),
			] {
				if config_file.exists() {
					return config_file.to_str().map(String::from);
				}
			}
		}
		None
	}

	/// Parses the configuration file.
	pub fn parse_config(file: &str) -> Result<Config> {
		let contents = fs::read_to_string(file)?;
		let config: Config = toml::from_str(&contents)?;
		Ok(config)
	}

	/// Update the command-line arguments based on configuration.
	pub fn update_args(self, mut args: Args) -> Args {
		args.armor = self.gpg.armor;
		args.splash = self.general.splash;
		args.homedir = self.gpg.homedir;
		args.outdir = self.gpg.outdir;
		args.default_key = self.gpg.default_key;
		args.tick_rate = self.general.tick_rate;
		args.color = Color::from(self.general.color.as_ref());
		args.style = self.general.style;
		args
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::path::PathBuf;
	#[test]
	fn test_parse_config() -> Result<()> {
		let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.join("config")
			.join(format!("{}.toml", env!("CARGO_PKG_NAME")))
			.to_string_lossy()
			.into_owned();
		if let Some(global_path) = Config::get_default_location() {
			path = global_path;
		}
		let mut config = Config::parse_config(&path)?;
		config.gpg.default_key = Some(String::from("test_key"));
		let args = config.update_args(Args::default());
		assert_eq!(Some(String::from("test_key")), args.default_key);
		Ok(())
	}
}
