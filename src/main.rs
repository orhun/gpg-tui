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
use log::LevelFilter;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::env;
use std::io::{self, Write};
use std::str::FromStr;

fn main() -> Result<()> {
	// Parse command-line arguments.
	let mut args = Args::parse();
	// Parse configuration file.
	let config = if let Some(config_file) =
		args.config.to_owned().or_else(Config::get_default_location)
	{
		let config = Config::parse_config(&config_file)?;
		args = config.update_args(args);
		config
	} else {
		Config::default()
	};
	// Initialize logger.
	tui_logger::init_logger(if let Ok(log_level) = env::var("RUST_LOG") {
		LevelFilter::from_str(&log_level)?
	} else {
		LevelFilter::Trace
	})?;
	tui_logger::set_default_level(LevelFilter::Trace);
	if let Some(ref log_file) = args.log_file {
		tui_logger::set_log_file(log_file)?;
	}
	log::debug!(target: "args", "{:?}", args);
	log::debug!(target: "config", "{:?}", config);
	// Set custom key bindings.
	let custom_key_bindings = config
		.general
		.unwrap_or_default()
		.key_bindings
		.unwrap_or_default();
	// Initialize GPGME library.
	let gpg_config = GpgConfig::new(&args)?;
	log::warn!(target: "gpg", "checking gpgme version: {:?}", GPGME_REQUIRED_VERSION);
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
			Event::Key(key_event) => handler::handle_events(
				key_event,
				&custom_key_bindings,
				&mut tui,
				&mut app,
			)?,
			Event::Tick => app.tick(),
			_ => {}
		}
	}
	// Exit the user interface.
	Tui::<CrosstermBackend<io::Stderr>>::reset()?;
	// Print the exit message if any.
	if let Some(message) = app.state.exit_message {
		writeln!(io::stdout(), "{message}")?;
	}
	Ok(())
}
