use anyhow::Result;
use clap::Parser;
use gpg_tui::app::handler;
use gpg_tui::app::launcher::App;
use gpg_tui::args::Args;
use gpg_tui::config::Config;
use gpg_tui::gpg::config::GpgConfig;
use gpg_tui::gpg::context::GpgContext;
use gpg_tui::term::event::{Event, EventHandler};
use gpg_tui::term::tui::Tui;
use gpg_tui::GPGME_REQUIRED_VERSION;
use std::io::{self, Write};
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> Result<()> {
	// Parse command-line arguments.
	let mut args = Args::parse();
	// Parse configuration file.
	if let Some(config_file) =
		args.config.to_owned().or_else(Config::get_default_location)
	{
		let config = Config::parse_config(&config_file)?;
		args = config.update_args(args);
	}
	// Initialize GPGME library.
	let gpg_config = GpgConfig::new(&args)?;
	gpg_config.check_gpgme_version(GPGME_REQUIRED_VERSION);
	let mut gpgme = GpgContext::new(gpg_config)?;
	// Create an application for rendering.
	let mut app = App::new(&mut gpgme, &args)?;
	// Initialize the text-based user interface.
	let backend = CrosstermBackend::new(io::stderr());
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
		writeln!(&mut io::stdout(), "{}", message)?;
	}
	Ok(())
}
