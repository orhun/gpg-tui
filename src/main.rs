//! dead simple TUI for GnuPG (WIP)

pub mod app;
pub mod args;
pub mod gpg;
pub mod term;

use self::app::launcher::App;
use self::args::Args;
use crate::app::handler;
use crate::term::event::{Event, EventHandler};
use crate::term::tui::Tui;
use anyhow::Result;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

/// Entry-point.
fn main() -> Result<()> {
	let args = Args::parse();
	let backend = CrosstermBackend::new(io::stdout());
	let terminal = Terminal::new(backend)?;
	let events = EventHandler::new(args.tick_rate);
	let tui = Tui::new(terminal, events);
	let mut app = App::new(tui)?;
	while app.running {
		app.draw_tui()?;
		match app.tui.events.next()? {
			Event::Key(key_event) => {
				handler::handle_key_input(&mut app, key_event)?
			}
			Event::Tick => {}
			_ => {}
		}
	}
	Ok(())
}
