//! Command-line argument parser.

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
    global_settings(&[
        AppSettings::ColorAuto,
        AppSettings::ColoredHelp,
        AppSettings::DeriveDisplayOrder,
    ])
)]
pub struct Args {
	/// Enables ASCII armored output.
	#[structopt(short, long)]
	pub armor: bool,
	/// Sets the GnuPG home directory.
	#[structopt(long, value_name = "dir")]
	pub homedir: Option<String>,
	/// Sets the output directory.
	#[structopt(short, long, value_name = "dir")]
	pub output: Option<PathBuf>,
	/// Sets the tick rate of the terminal.
	#[structopt(short, long, value_name = "ms", default_value = "250")]
	pub tick_rate: u64,
	/// Sets the accent color of the terminal.
	#[structopt(short, long, default_value = "gray", parse(from_str))]
	pub color: Color,
	/// Sets the style of the terminal.
	#[structopt(short, long, possible_values = &["plain", "colored"], default_value = "plain")]
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
