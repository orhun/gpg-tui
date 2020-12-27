//! Command-line argument parser.

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
	/// Sets the tick rate of the terminal.
	#[structopt(short, long, value_name = "MS", default_value = "250")]
	pub tick_rate: u64,
}

impl Args {
	/// Parses the command-line arguments.
	///
	/// See [`StructOpt::from_args`].
	pub fn parse() -> Self {
		Self::from_args()
	}
}
