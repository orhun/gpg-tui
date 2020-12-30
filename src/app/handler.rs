use crate::app::launcher::App;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};

/// Handle key inputs.
pub fn handle_key_input(key_event: KeyEvent, app: &mut App) -> Result<()> {
	match key_event.code {
		Key::Char('q') | Key::Char('Q') | Key::Esc => app.running = false,
		Key::Char('c') | Key::Char('d') => {
			if key_event.modifiers == Modifiers::CONTROL {
				app.running = false;
			}
		}
		Key::Up => app.key_list.previous(),
		Key::Down => app.key_list.next(),
		_ => {}
	}
	Ok(())
}
