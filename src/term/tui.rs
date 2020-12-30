use crate::app::launcher::App;
use crate::term::event::EventHandler;
use anyhow::{Context, Result};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{self, Write};
use tui::backend::Backend;
use tui::Terminal;

/// Text-based user interface.
///
/// `Tui` is responsible for setting up the terminal
/// and initializing the interface. Terminal event
/// handler can be accessed via [`events`] field.
///
/// [`events`]: Tui::events
pub struct Tui<B: Backend> {
	/// Interface to the Terminal.
	terminal: Terminal<B>,
	/// Terminal event handler.
	pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
	/// Constructs a new instance of `Tui`.
	pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
		Self { terminal, events }
	}

	/// Initializes the terminal interface.
	///
	/// It enables the raw mode and sets terminal properties.
	pub fn init(&mut self) -> Result<()> {
		terminal::enable_raw_mode()?;
		crossterm::execute!(
			io::stdout(),
			EnterAlternateScreen,
			EnableMouseCapture
		)?;
		self.terminal.hide_cursor()?;
		self.terminal.clear()?;
		Ok(())
	}

	/// [`Draw`] the terminal interface by [`rendering`] the widgets.
	///
	/// [`Draw`]: tui::Terminal::draw
	/// [`rendering`]: crate::app::launcher::App::render
	pub fn draw(&mut self, app: &mut App) -> Result<()> {
		self.terminal
			.draw(|f| app.render(f))
			.context("failed to draw TUI")
	}

	/// Exits the terminal interface.
	///
	/// It disables the raw mode and reverts back the terminal properties.
	pub fn exit(&mut self) -> Result<()> {
		terminal::disable_raw_mode()?;
		crossterm::execute!(
			io::stdout(),
			LeaveAlternateScreen,
			DisableMouseCapture
		)?;
		self.terminal.show_cursor()?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tui::backend::TestBackend;
	#[test]
	fn test_tui() -> Result<()> {
		let backend = TestBackend::new(10, 10);
		let terminal = Terminal::new(backend)?;
		let mut tui = Tui::new(terminal, EventHandler::new(10));
		tui.init()?;
		tui.exit()?;
		Ok(())
	}
}
