use crate::app::launcher::App;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};

/// Handle key inputs.
pub fn handle_key_input(app: &mut App, key_event: KeyEvent) -> Result<()> {
	match key_event.code {
		Key::Char('q') | Key::Char('Q') | Key::Esc => app.exit()?,
		Key::Char('c') | Key::Char('d') => {
			if key_event.modifiers == Modifiers::CONTROL {
				app.exit()?
			}
		}
		_ => {}
	}
	Ok(())
}
