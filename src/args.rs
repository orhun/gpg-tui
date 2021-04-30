//! Command-line argument parser.

use crate::app::banner::BANNERS;
use crate::widget::style::Color;
use std::path::PathBuf;
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
	/// Sets the GnuPG home directory.
	#[structopt(long, value_name = "dir", env)]
	pub homedir: Option<String>,
	/// Sets the default key to sign with.
	#[structopt(short, long, value_name = "key", env)]
	pub default_key: Option<String>,
	/// Sets the output directory.
	#[structopt(short, long, value_name = "dir", env)]
	pub output: Option<PathBuf>,
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
}

impl Args {
	/// Parses the command-line arguments.
	///
	/// See [`StructOpt::from_args`].
	pub fn parse() -> Self {
		Self::from_args()
	}
}
