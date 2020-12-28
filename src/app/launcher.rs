use crate::app::handler;
use crate::app::renderer;
use crate::app::state::State;
use crate::args::Args;
use crate::term::event::{Event, EventHandler};
use crate::term::tui::Tui;
use anyhow::{Context, Result};
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

/// Main application.
///
/// It operates the TUI using event handler,
/// application flags and other properties.
pub struct App<'a> {
	/// Parsed command-line arguments.
	#[allow(dead_code)]
	args: &'a Args,
	/// Terminal user interface.
	pub tui: Tui<CrosstermBackend<io::Stdout>>,
	/// Terminal event handler.
	pub events: EventHandler,
	/// Application states.
	pub state: State,
}

impl<'a> App<'a> {
	/// Constructs a new instance of `App`.
	pub fn new(args: &'a Args) -> Result<Self> {
		let backend = CrosstermBackend::new(io::stdout());
		let terminal = Terminal::new(backend)?;
		Ok(Self {
			args,
			tui: Tui::new(terminal),
			events: EventHandler::new(args.tick_rate),
			state: State::default(),
		})
	}

	/// Initializes the [`Tui`] and handles events.
	///
	/// [`Tui`]: crate::term::tui::Tui
	pub fn start(&mut self) -> Result<()> {
		self.tui.init()?;
		while self.state.running {
			self.draw_tui()?;
			match self.events.next()? {
				Event::Key(key_event) => {
					handler::handle_key_input(self, key_event)?
				}
				Event::Tick => {}
				_ => {}
			}
		}
		Ok(())
	}

	/// [`Draw`] the terminal interface by [`rendering`] the widgets.
	///
	/// [`Draw`]: tui::Terminal::draw
	/// [`rendering`]: crate::app::renderer
	pub fn draw_tui(&mut self) -> Result<()> {
		self.tui
			.terminal
			.draw(|f| renderer::draw_test_block(f, f.size()))
			.context("failed to draw TUI")
	}

	/// Exits the application.
	///
	/// It calls the [`exit`] method of `Tui` and ends
	/// the terminal loop via changing the [`state`].
	///
	/// [`exit`]: crate::term::tui::Tui::exit
	/// [`state`]: State::running
	pub fn exit(&mut self) -> Result<()> {
		self.state.running = false;
		self.tui.exit()
	}
}
