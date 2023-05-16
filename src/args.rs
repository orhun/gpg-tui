//! Command-line argument parser.

use crate::app::banner::BANNERS;
use crate::app::selection::Selection;
use crate::app::style::Style;
use crate::gpg::key::KeyDetail;
use crate::widget::style::Color;
use clap::Parser;

/// Argument parser powered by [`clap`].
#[derive(Debug, Default, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
	before_help = BANNERS[2],
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
		value_parser = Args::parse_dir
	)]
	pub config: Option<String>,
	/// Sets the GnuPG home directory.
	#[clap(
		long,
		value_name = "dir",
		env = "GNUPGHOME",
		value_parser = Args::parse_dir
	)]
	pub homedir: Option<String>,
	/// Sets the output directory.
	#[clap(
		short,
		long,
		value_name = "dir",
		env,
		value_parser = Args::parse_dir
	)]
	pub outdir: Option<String>,
	/// Sets the template for the output file name.
	#[clap(
		long,
		value_name = "path",
		default_value = "{type}_{query}.{ext}",
		env,
		value_parser = Args::parse_dir
	)]
	pub outfile: String,
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
		env
	)]
	pub color: Color,
	/// Sets the style of the terminal.
	#[clap(
		short,
		long,
		value_name = "style",
		default_value = "plain",
		env
	)]
	pub style: Style,
	/// Sets the utility for file selection.
	#[clap(short, long, value_name = "app", default_value = "xplr", env)]
	pub file_explorer: String,
	/// Sets the detail level for the keys.
	#[clap(long, value_name = "level", default_value = "minimum", env)]
	pub detail_level: KeyDetail,
	/// Enables the selection mode.
	#[clap(
		long,
		value_name = "option",
		value_parser = ["key_id", "key_fpr", "user_id", "row1", "row2"],
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
	fn parse_dir(dir: &str) -> Result<String, String> {
		Ok(shellexpand::tilde(dir).to_string())
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
