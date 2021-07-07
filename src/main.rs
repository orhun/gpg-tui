use anyhow::Result;
use gpg_tui::app::handler;
use gpg_tui::app::launcher::App;
use gpg_tui::args::Args;
use gpg_tui::gpg::config::GpgConfig;
use gpg_tui::gpg::context::GpgContext;
use gpg_tui::term::event::{Event, EventHandler};
use gpg_tui::term::tui::Tui;
use gpg_tui::GPGME_REQUIRED_VERSION;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<()> {
	// Parse command-line arguments.
	let args = Args::parse();
	// Initialize GPGME library.
	let config = GpgConfig::new(&args).unwrap();
	config.check_gpgme_version(GPGME_REQUIRED_VERSION);
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
				handler::handle_events(key_event, &mut tui, &mut app)?
			}
			Event::Tick => app.tick(),
			_ => {}
		}
	}
	// Exit the user interface.
	tui.exit()?;
	// Print the exit message if any.
	if let Some(message) = app.state.exit_message {
		println!("{}", message);
	}
	Ok(())
}
