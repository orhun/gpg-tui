//! dead simple TUI for GnuPG (WIP)

pub mod app;
pub mod args;
pub mod term;

use self::app::launcher::App;
use self::args::Args;
use anyhow::Result;

/// Entry-point.
fn main() -> Result<()> {
	let args = Args::parse();
	App::new(&args)?.start()
}
