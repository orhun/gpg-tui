use crate::app::launcher::App;
use crate::app::renderer;
use crate::term::event::EventHandler;
use anyhow::{Context, Result};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;
use std::sync::atomic::Ordering;

/// Text-based user interface.
///
/// `Tui` is responsible for setting up the terminal
/// and initializing the interface. Terminal event
/// handler can be accessed via [`events`] field.
///
/// [`events`]: Tui::events
#[derive(Debug)]
pub struct Tui<B: Backend> {
	/// Interface to the Terminal.
	terminal: Terminal<B>,
	/// Terminal event handler.
	pub events: EventHandler,
	/// Is the interface paused?
	pub paused: bool,
}

impl<B: Backend> Tui<B> {
	/// Constructs a new instance of `Tui`.
	pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
		Self {
			terminal,
			events,
			paused: false,
		}
	}

	/// Initializes the terminal interface.
	///
	/// It enables the raw mode and sets terminal properties.
	pub fn init(&mut self) -> Result<()> {
		terminal::enable_raw_mode()?;
		crossterm::execute!(
			io::stderr(),
			EnterAlternateScreen,
			EnableMouseCapture
		)?;
		panic::set_hook(Box::new(move |panic| {
			Self::reset().expect("failed to reset the terminal");
			better_panic::Settings::auto()
				.most_recent_first(false)
				.lineno_suffix(true)
				.create_panic_handler()(panic);
			std::process::exit(1);
		}));
		self.terminal.hide_cursor()?;
		self.terminal.clear()?;
		Ok(())
	}

	/// Toggles the [`paused`] state of interface.
	///
	/// It disables the key input and exits the
	/// terminal interface on pause (and vice-versa).
	///
	/// [`paused`]: Tui::paused
	pub fn toggle_pause(&mut self) -> Result<()> {
		self.paused = !self.paused;
		if self.paused {
			self.exit()?;
		} else {
			self.init()?;
		}
		self.events
			.key_input_disabled
			.store(self.paused, Ordering::Relaxed);
		Ok(())
	}

	/// Enables the mouse capture.
	pub fn enable_mouse_capture(&mut self) -> Result<()> {
		Ok(crossterm::execute!(io::stderr(), EnableMouseCapture)?)
	}

	/// Disables the mouse capture.
	pub fn disable_mouse_capture(&mut self) -> Result<()> {
		Ok(crossterm::execute!(io::stderr(), DisableMouseCapture)?)
	}

	/// [`Draw`] the terminal interface by [`rendering`] the widgets.
	///
	/// [`Draw`]: tui::Terminal::draw
	/// [`rendering`]: crate::app::renderer::render
	pub fn draw(&mut self, app: &mut App) -> Result<()> {
		self.terminal
			.draw(|frame| renderer::render(app, frame))
			.context("failed to draw TUI")?;
		Ok(())
	}

	/// Exits the terminal interface.
	///
	/// It disables the raw mode and reverts back the terminal properties.
	pub fn exit(&mut self) -> Result<()> {
		Self::reset()?;
		self.terminal.show_cursor()?;
		Ok(())
	}

	/// Resets the terminal interface.
	fn reset() -> Result<()> {
		terminal::disable_raw_mode()?;
		crossterm::execute!(
			io::stderr(),
			LeaveAlternateScreen,
			DisableMouseCapture
		)?;
		Ok(())
	}
}

#[cfg(feature = "tui-tests")]
#[cfg(test)]
mod tests {
	use super::*;
	use ratatui::backend::TestBackend;
	#[test]
	fn test_term_tui() -> Result<()> {
		let backend = TestBackend::new(10, 10);
		let terminal = Terminal::new(backend)?;
		let mut tui = Tui::new(terminal, EventHandler::new(10));
		tui.init()?;
		tui.exit()?;
		Ok(())
	}
}
