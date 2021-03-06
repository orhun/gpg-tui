//! dead simple TUI for GnuPG (WIP)

pub mod app;
pub mod args;
pub mod gpg;
pub mod term;
pub mod widget;

use self::app::launcher::App;
use self::args::Args;
use crate::app::handler;
use crate::gpg::config::GpgConfig;
use crate::gpg::context::GpgContext;
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
	// Initialize GPGME library.
	let config = GpgConfig::new(&args)?;
	config.check_gpgme_version("1.7.0");
	let mut gpgme = GpgContext::new(config)?;
	// Create an application for rendering.
	let mut app = App::new(&mut gpgme, &args)?;
	// Initialize the text-based user interface.
	let backend = CrosstermBackend::new(io::stdout());
	let terminal = Terminal::new(backend)?;
	let events = EventHandler::new(args.tick_rate);
	let mut tui = Tui::new(terminal, events);
	tui.init()?;
	// Start the main loop.
	while app.state.running {
		// Render the user interface.
		tui.draw(&mut app)?;
		// Handle events.
		match tui.events.next()? {
			Event::Key(key_event) => {
				handler::handle_key_input(key_event, &mut tui, &mut app)?
			}
			Event::Tick => app.tick(),
			_ => {}
		}
	}
	// Exit.
	tui.exit()
}
