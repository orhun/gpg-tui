//! Configuration file parser.

use crate::app::command::Command;
use crate::app::style::Style;
use crate::args::Args;
use crate::gpg::key::KeyDetail;
use crate::widget::style::Color;
use anyhow::Result;
use clap::ValueEnum;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::str::FromStr;
use toml::value::Value;

/// Default color, style, and settings.
const DEFAULT_COLOR: &str = "gray";
const DEFAULT_STYLE: &str = "plain";
const DEFAULT_FILE_EXPLORER: &str = "xplr";
const DEFAULT_TICK_RATE: u64 = 250_u64;
const DEFAULT_SPLASH: bool = false;
const DEFAULT_ARMOR: bool = false;
const DEFAULT_DETAIL_LEVEL: &str = "minimum";
const DEFAULT_HOMEDIR: &str = "~/.gnupg";
const DEFAULT_OUTDIR: &str = "~/.gnupg";

/// Application configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
	/// General configuration.
	pub general: Option<GeneralConfig>,
	/// GnuPG configuration.
	pub gpg: Option<GpgConfig>,
}

/// General configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GeneralConfig {
	/// [`Args::splash`]
	pub splash: Option<bool>,
	/// [`Args::tick_rate`]
	pub tick_rate: Option<u64>,
	/// [`Args::color`]
	pub color: Option<String>,
	/// [`Args::style`]
	pub style: Option<String>,
	/// [`Args::file_explorer`]
	pub file_explorer: Option<String>,
	/// [`Args::detail_level`]
	pub detail_level: Option<KeyDetail>,
	/// Custom key bindings.
	#[serde(skip_serializing)]
	pub key_bindings: Option<Vec<CustomKeyBinding>>,
	/// File to save the logs.
	pub log_file: Option<String>,
}

/// Representation of custom key bindings.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CustomKeyBinding {
	/// Key events to check.
	#[serde(deserialize_with = "deserialize_keys")]
	pub keys: Vec<KeyEvent>,
	/// Command to run.
	#[serde(deserialize_with = "deserialize_command")]
	pub command: Command,
}

/// Custom deserializer for parsing a vector of [`KeyEvent`]s
fn deserialize_keys<'de, D>(deserializer: D) -> Result<Vec<KeyEvent>, D::Error>
where
	D: Deserializer<'de>,
{
	let mut key_bindings = Vec::new();
	let keys: Vec<Value> = Deserialize::deserialize(deserializer)?;
	for key in keys {
		if let Some(key_str) = key.as_str() {
			let mut modifiers = KeyModifiers::NONE;
			// parse a single character
			let key_code = if key_str.len() == 1 {
				KeyCode::Char(key_str.chars().collect::<Vec<char>>()[0])
			// parse function keys
			} else if key_str.len() == 2
				&& key_str.to_lowercase().starts_with('f')
			{
				let num = key_str
					.chars()
					.map(|v| v.to_string())
					.collect::<Vec<String>>()[1]
					.parse::<u8>()
					.map_err(de::Error::custom)?;
				KeyCode::F(num)
			// parse control/alt combinations
			} else if key_str.len() == 3 && key_str.contains('-') {
				if key_str.to_lowercase().starts_with("c-") {
					modifiers = KeyModifiers::CONTROL
				} else if key_str.to_lowercase().starts_with("a-") {
					modifiers = KeyModifiers::ALT
				}
				KeyCode::Char(key_str.chars().collect::<Vec<char>>()[2])
			// try parsing the keycode
			} else {
				let mut c = key_str.chars();
				let key_str = match c.next() {
					None => String::new(),
					Some(v) => {
						v.to_uppercase().collect::<String>() + c.as_str()
					}
				};
				Deserialize::deserialize(Value::String(key_str))
					.map_err(de::Error::custom)?
			};
			key_bindings.push(KeyEvent::new(key_code, modifiers))
		} else {
			return Err(de::Error::custom("invalid type"));
		}
	}
	Ok(key_bindings)
}

/// Custom deserializer for parsing [`Command`]s
fn deserialize_command<'de, D>(deserializer: D) -> Result<Command, D::Error>
where
	D: Deserializer<'de>,
{
	let s = String::deserialize(deserializer)?;
	Command::from_str(&s)
		.map_err(|_| de::Error::custom(format!("invalid command ({s})")))
}

/// GnuPG configuration.
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct GpgConfig {
	/// [`Args::armor`]
	pub armor: Option<bool>,
	/// [`Args::homedir`]
	pub homedir: Option<String>,
	/// [`Args::outdir`]
	pub outdir: Option<String>,
	/// [`Args::outfile`]
	pub outfile: Option<String>,
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
			for config_file in [
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
	pub fn update_args(&self, mut args: Args) -> Args {
		let default_color: Color = Color::from(DEFAULT_COLOR);
		let default_style =
			Style::from_str(DEFAULT_STYLE, true).unwrap_or_default();
		let default_file_explorer: String = String::from(DEFAULT_FILE_EXPLORER);
		match self.gpg.as_ref() {
			Some(gpg) => {
				args.armor = gpg.armor.unwrap_or_default();
				args.homedir.clone_from(&gpg.homedir);
				args.outdir.clone_from(&gpg.outdir);
				if let Some(outfile) = &gpg.outfile {
					args.outfile = outfile.to_string();
				}
				if let Some(default_key) = &gpg.default_key {
					args.default_key = Some(default_key.clone());
				}
			}
			None => {
				args.armor = DEFAULT_ARMOR;
				args.homedir = Some(String::from(DEFAULT_HOMEDIR));
				args.outdir = Some(String::from(DEFAULT_OUTDIR));
			}
		}
		match self.general.as_ref() {
			Some(general) => {
				args.splash = general.splash.unwrap_or_default();
				args.tick_rate = general.tick_rate.unwrap_or(DEFAULT_TICK_RATE);
				args.color = general
					.color
					.as_ref()
					.map(|color| Color::from(color.as_ref()))
					.unwrap_or(default_color);
				args.style = general
					.style
					.as_ref()
					.map(|style| {
						Style::from_str(style.as_ref(), true)
							.unwrap_or_default()
					})
					.unwrap_or_default();
				args.file_explorer = general
					.file_explorer
					.as_ref()
					.cloned()
					.unwrap_or(default_file_explorer);
				args.detail_level = general.detail_level.unwrap_or(
					KeyDetail::from_str(DEFAULT_DETAIL_LEVEL, true)
						.unwrap_or_default(),
				);
				if general.log_file.is_some() {
					args.log_file.clone_from(&general.log_file);
				}
			}
			None => {
				args.splash = DEFAULT_SPLASH;
				args.tick_rate = DEFAULT_TICK_RATE;
				args.color = default_color;
				args.style = default_style;
				args.file_explorer = default_file_explorer;
			}
		}
		args
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::fs::{self, File};
	use std::io::Write;
	use std::path::PathBuf;

	#[test]
	fn test_parse_config() -> Result<()> {
		let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
			.join("config")
			.join(format!("{}.toml", env!("CARGO_PKG_NAME")))
			.to_string_lossy()
			.into_owned();
		let mut config = Config::parse_config(&path)?;
		if let Some(ref mut gpg) = config.gpg {
			gpg.default_key = Some(String::from("test_key"));
		}
		let args = config.update_args(Args::default());
		assert_eq!(Some(String::from("test_key")), args.default_key);
		Ok(())
	}

	#[test]
	fn test_args_partial_config_general() -> Result<()> {
		let mut temp_file = File::create("config/temp.toml")?;
		temp_file.write_all("[general]\n   splash = true\n".as_bytes())?;
		let tmp_path = PathBuf::from("config/temp.toml");
		if let Ok(config) = Config::parse_config(&tmp_path.to_string_lossy()) {
			let args = config.update_args(Args::default());
			// [general]
			assert_eq!(args.splash, true); // supplied
			assert_eq!(args.tick_rate, 250_u64);
			// [gpg]
			assert_eq!(args.armor, false);
			assert_eq!(args.default_key, None);
		}
		fs::remove_file(tmp_path)?;
		Ok(())
	}

	#[test]
	fn test_args_partial_config_gpg() -> Result<()> {
		let mut temp_file = File::create("config/temp2.toml")?;
		temp_file.write_all("[gpg]\n   armor = true\n".as_bytes())?;
		let tmp_path = PathBuf::from("config/temp2.toml");
		if let Ok(config) = Config::parse_config(&tmp_path.to_string_lossy()) {
			let args = config.update_args(Args::default());
			// [general]
			assert_eq!(args.splash, false);
			assert_eq!(args.tick_rate, 250_u64);
			// [gpg]
			assert_eq!(args.armor, true); // supplied
			assert_eq!(args.default_key, None);
		}
		fs::remove_file(tmp_path)?;
		Ok(())
	}

	#[test]
	fn test_parse_key_bindings() -> Result<()> {
		for (keys, cmd, config) in [
			(
				vec![
					KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
				],
				":visual",
				"keys = [ 'enter', 'v' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
					KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
				],
				"quit",
				"keys = [ 'C-c', 'Q', 'esc' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE),
				],
				":help",
				"keys = [ '?', 'h', 'f1' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Char('1'), KeyModifiers::ALT),
					KeyEvent::new(KeyCode::Char('R'), KeyModifiers::NONE),
				],
				":REFRESH",
				"keys = [ 'F5', 'A-1', 'R' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
				],
				":OPTIONS",
				"keys = [ 'O', ' ' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Char('p'), KeyModifiers::CONTROL),
				],
				":paste",
				"keys = [ 'p', 'c-p' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE),
				],
				":delete",
				"keys = [ 'backspace', 'Backspace', 'left', 'delete' ]",
			),
			(
				vec![
					KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
					KeyEvent::new(KeyCode::Char('D'), KeyModifiers::CONTROL),
					KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL),
					KeyEvent::new(KeyCode::Char('3'), KeyModifiers::ALT),
					KeyEvent::new(KeyCode::F(0), KeyModifiers::NONE),
				],
				":export",
				"keys = [ 'x', 'c-D', 'c-x', 'A-3', 'f0' ]",
			),
		] {
			assert_eq!(
				CustomKeyBinding {
					keys,
					command: Command::from_str(cmd).expect("invalid command"),
				},
				toml::from_str(&format!("{config}\ncommand = '{cmd}'"))?
			);
		}

		for config in &[
			"keys = [ 'x' ] \n command = ':x'",
			"keys = [ 'test' ] \n command = ':help'",
			"keys = [ '' ] \n command = ':help'",
			"keys = [ 'q' ] \n command = ':qx'",
		] {
			assert!(toml::from_str::<CustomKeyBinding>(config).is_err());
		}
		Ok(())
	}
}
