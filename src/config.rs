//! Configuration file parser.

use crate::app::command::Command;
use crate::app::style::Style;
use crate::args::Args;
use crate::gpg::key::KeyDetail;
use crate::widget::style::Color;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::str::FromStr;
use toml::value::Value;

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
	/// [`Args::file_explorer`]
	pub file_explorer: String,
	/// [`Args::detail_level`]
	pub detail_level: KeyDetail,
	/// Custom key bindings.
	#[serde(skip_serializing)]
	pub key_bindings: Option<Vec<CustomKeyBinding>>,
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
		.map_err(|_| de::Error::custom(format!("invalid command ({})", s)))
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
	pub fn update_args(&self, mut args: Args) -> Args {
		args.armor = self.gpg.armor;
		args.splash = self.general.splash;
		args.homedir = self.gpg.homedir.clone();
		args.outdir = self.gpg.outdir.clone();
		if let Some(outfile) = &self.gpg.outfile {
			args.outfile = outfile.to_string();
		}
		args.default_key = self.gpg.default_key.clone();
		args.tick_rate = self.general.tick_rate;
		args.color = Color::from(self.general.color.as_ref());
		args.style = Style::from_str(&self.general.style).unwrap_or_default();
		args.detail_level = self.general.detail_level;
		args.file_explorer = self.general.file_explorer.clone();
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

	#[test]
	fn test_parse_key_bindings() -> Result<()> {
		for (keys, cmd, config) in vec![
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
				toml::from_str(&format!("{}\ncommand = '{}'", config, cmd))?
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
