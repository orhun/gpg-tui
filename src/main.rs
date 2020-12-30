//! dead simple TUI for GnuPG (WIP)

pub mod app;
pub mod args;
pub mod gpg;
pub mod term;
pub mod widget;

use self::app::launcher::App;
use self::args::Args;
use crate::app::handler;
use crate::term::event::{Event, EventHandler};
use crate::term::tui::Tui;
use anyhow::Result;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

/// Entry-point of the application.
fn main() -> Result<()> {
	// Parse command-line arguments.
	let args = Args::parse();
	// Initialize the text-based user interface.
	let backend = CrosstermBackend::new(io::stdout());
	let terminal = Terminal::new(backend)?;
	let events = EventHandler::new(args.tick_rate);
	let mut tui = Tui::new(terminal, events);
	tui.init()?;
	// Create an application for rendering.
	let mut app = App::new()?;
	app.refresh();
	// Start the main loop.
	while app.running {
		// Render the user interface.
		tui.draw(&mut app)?;
		// Handle events.
		match tui.events.next()? {
			Event::Key(key_event) => {
				handler::handle_key_input(key_event, &mut app)?
			}
			Event::Tick => {}
			_ => {}
		}
	}
	// Exit.
	tui.exit()
}
