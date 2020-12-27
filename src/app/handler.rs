use crate::app::launcher::App;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};

/// Handle key inputs.
pub fn handle_key_input(app: &mut App<'_>, key_event: KeyEvent) -> Result<()> {
	println!("{:?}", key_event);
	match key_event.code {
		Key::Char('q') | Key::Char('Q') | Key::Esc => {
			app.tui.exit()?;
			app.state.running = false;
		}
		Key::Char('c') | Key::Char('d') => {
			if key_event.modifiers == Modifiers::CONTROL {
				app.tui.exit()?;
				app.state.running = false;
			}
		}
		_ => {}
	}
	Ok(())
}
