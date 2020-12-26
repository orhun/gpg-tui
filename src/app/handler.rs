use crate::app::app::App;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent};

/// Handle key inputs.
pub fn handle_key_input(app: &mut App, key_event: KeyEvent) -> Result<()> {
	match key_event.code {
		Key::Char('q') | Key::Char('Q') | Key::Esc => {
			app.tui.exit()?;
			app.state.running = false;
		}
		_ => {}
	}
	Ok(())
}
