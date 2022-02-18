//! Command-line argument parser.

use crate::app::banner::BANNERS;
use crate::app::selection::Selection;
use crate::app::style::Style;
use crate::widget::style::Color;
use clap::{AppSettings, Parser};

/// Argument parser powered by [`clap`].
#[derive(Debug, Default, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
	before_help = BANNERS[2],
    global_setting = AppSettings::DeriveDisplayOrder,
	rename_all_env = "screaming-snake",
)]
pub struct Args {
	/// Enables ASCII armored output.
	#[clap(short, long)]
	pub armor: bool,
	/// Shows the splash screen on startup.
	#[clap(long)]
	pub splash: bool,
	/// Sets the configuration file.
	#[clap(
		long,
		value_name = "path",
		env = "GPG_TUI_CONFIG",
		parse(from_str = Args::parse_dir)
	)]
	pub config: Option<String>,
	/// Sets the GnuPG home directory.
	#[clap(
		long,
		value_name = "dir",
		env = "GNUPGHOME",
		parse(from_str = Args::parse_dir)
	)]
	pub homedir: Option<String>,
	/// Sets the output directory.
	#[clap(
		short,
		long,
		value_name = "dir",
		env,
		parse(from_str = Args::parse_dir)
	)]
	pub outdir: Option<String>,
	/// Sets the default key to sign with.
	#[clap(short, long, value_name = "key", env)]
	pub default_key: Option<String>,
	/// Sets the tick rate of the terminal.
	#[clap(short, long, value_name = "ms", default_value = "250", env)]
	pub tick_rate: u64,
	/// Sets the accent color of the terminal.
	#[clap(
		short,
		long,
		value_name = "color",
		default_value = "gray",
		parse(from_str),
		env
	)]
	pub color: Color,
	/// Sets the style of the terminal.
	#[clap(
		short,
		long,
		value_name = "style",
		possible_values = &["plain", "colored"],
		default_value = "plain",
		env
	)]
	pub style: Style,
	/// Sets the utility for file selection.
	#[clap(short, long, value_name = "app", default_value = "xplr", env)]
	pub file_explorer: String,
	/// Enables the selection mode.
	#[clap(
		long,
		value_name = "option",
		possible_values = &["key_id", "key_fpr", "user_id", "row1", "row2"],
		env
	)]
	pub select: Option<Selection>,
}

impl Args {
	/// Custom string parser for directories.
	///
	/// Expands the tilde (`~`) character in the beginning of the
	/// input string into contents of the path returned by [`home_dir`].
	///
	/// [`home_dir`]: dirs_next::home_dir
	fn parse_dir(dir: &str) -> String {
		shellexpand::tilde(dir).to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;
	#[test]
	fn test_args() {
		Args::command().debug_assert();
	}
}
