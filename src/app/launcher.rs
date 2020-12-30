use crate::app::renderer;
use crate::term::tui::Tui;
use anyhow::{Context, Result};
use std::io;
use tui::backend::CrosstermBackend;

/// Main application.
///
/// It operates the TUI.
pub struct App {
	/// Terminal user interface.
	pub tui: Tui<CrosstermBackend<io::Stdout>>,
	/// Is app running?
	pub running: bool,
}

impl App {
	/// Constructs a new instance of `App`.
	pub fn new(mut tui: Tui<CrosstermBackend<io::Stdout>>) -> Result<Self> {
		tui.init()?;
		Ok(Self { tui, running: true })
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
	/// the terminal loop via changing the [`running`] state.
	///
	/// [`exit`]: crate::term::tui::Tui::exit
	/// [`running`]: App::running
	pub fn exit(&mut self) -> Result<()> {
		self.running = false;
		self.tui.exit()
	}
}
