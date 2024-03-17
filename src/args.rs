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
    version,
    author = clap::crate_authors!("\n"),
    about,
	rename_all_env = "screaming-snake",
	before_help = format!("\u{2800} {}", BANNERS[2]),
	help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
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
	#[clap(short, long, value_name = "color", default_value = "gray", env)]
	pub color: Color,
	/// Sets the style of the terminal.
	#[clap(short, long, value_name = "style", default_value = "colored", env)]
	pub style: Style,
	/// Sets the utility for file selection.
	#[clap(short, long, value_name = "app", default_value = "xplr", env)]
	pub file_explorer: String,
	/// Sets the detail level for the keys.
	#[clap(long, value_name = "level", default_value = "minimum", env)]
	pub detail_level: KeyDetail,
	/// Sets the file to save the logs.
	#[clap(long, value_name = "path", env)]
	pub log_file: Option<String>,
	/// Enables the selection mode.
	#[clap(long, value_name = "option", env)]
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
	use std::path::PathBuf;
	#[test]
	fn test_args() {
		Args::command().debug_assert();
	}
	#[test]
	fn test_tilde_expansion() {
		let home_dir =
			dirs_next::home_dir().expect("cannot retrieve home directory");
		let dir = Args::parse_dir("~/").expect("cannot expand tilde");
		assert_eq!(home_dir, PathBuf::from(dir));
	}
}
