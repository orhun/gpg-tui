//! dead simple TUI for GnuPG (WIP)

pub mod app;
pub mod term;

use self::app::app::App;
use anyhow::Result;

/// Entry-point.
fn main() -> Result<()> {
	App::new()?.start()
}
