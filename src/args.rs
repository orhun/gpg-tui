//! Command-line argument parser.

use crate::app::banner::BANNERS;
use crate::app::selection::Selection;
use crate::widget::style::Color;
use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Argument parser powered by [`structopt`].
#[derive(Debug, Default, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
	before_help = BANNERS[2],
    global_settings(&[
        AppSettings::ColorAuto,
        AppSettings::ColoredHelp,
        AppSettings::DeriveDisplayOrder,
    ]),
	rename_all_env = "screaming-snake",
)]
pub struct Args {
	/// Enables ASCII armored output.
	#[structopt(short, long)]
	pub armor: bool,
	/// Shows the splash screen on startup.
	#[structopt(long)]
	pub splash: bool,
	/// Sets the configuration file.
	#[structopt(long, value_name = "path", env = "GPG_TUI_CONFIG", parse(from_str = Args::parse_dir))]
	pub config: Option<String>,
	/// Sets the GnuPG home directory.
	#[structopt(long, value_name = "dir", env = "GNUPGHOME", parse(from_str = Args::parse_dir))]
	pub homedir: Option<String>,
	/// Sets the output directory.
	#[structopt(short, long, value_name = "dir", env, parse(from_str = Args::parse_dir))]
	pub outdir: Option<String>,
	/// Sets the default key to sign with.
	#[structopt(short, long, value_name = "key", env)]
	pub default_key: Option<String>,
	/// Sets the tick rate of the terminal.
	#[structopt(short, long, value_name = "ms", default_value = "250", env)]
	pub tick_rate: u64,
	/// Sets the accent color of the terminal.
	#[structopt(short, long, default_value = "gray", parse(from_str), env)]
	pub color: Color,
	/// Sets the style of the terminal.
	#[structopt(
		short, long, possible_values = &["plain", "colored"],
		default_value = "plain", env
	)]
	pub style: String,
	/// Enables the selection mode.
	#[structopt(
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

	/// Parses the command-line arguments.
	///
	/// See [`StructOpt::from_args`].
	pub fn parse() -> Self {
		Self::from_args()
	}
}
