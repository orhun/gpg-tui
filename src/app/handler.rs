use crate::app::launcher::App;
use crate::widget::row::ScrollDirection;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};

/// Handle key events.
pub fn handle_key_input(key_event: KeyEvent, app: &mut App) -> Result<()> {
	match key_event.code {
		Key::Char('q') | Key::Char('Q') | Key::Esc => app.state.running = false,
		Key::Char('c') | Key::Char('d') => {
			if key_event.modifiers == Modifiers::CONTROL {
				app.state.running = false;
			}
		}
		Key::Char('r') | Key::Char('R') | Key::F(5) => app.refresh()?,
		Key::Up => {
			if key_event.modifiers == Modifiers::ALT {
				app.key_list.scroll(ScrollDirection::Up(1))
			} else {
				app.key_list.previous();
			}
		}
		Key::Right => {
			if key_event.modifiers == Modifiers::ALT {
				app.key_list.scroll(ScrollDirection::Right(1))
			}
		}
		Key::Down => {
			if key_event.modifiers == Modifiers::ALT {
				app.key_list.scroll(ScrollDirection::Down(1))
			} else {
				app.key_list.next();
			}
		}
		Key::Left => {
			if key_event.modifiers == Modifiers::ALT {
				app.key_list.scroll(ScrollDirection::Left(1))
			}
		}
		Key::Char('t') | Key::Char('T') => {
			app.state.table_detail.increase();
			for key in app.key_list.items.iter_mut() {
				key.detail = app.state.table_detail;
			}
		}
		Key::Tab => {
			if let Some(index) = app.key_list.state.selected() {
				if let Some(key) = app.key_list.items.get_mut(index) {
					key.detail.increase()
				}
			}
		}
		_ => {}
	}
	Ok(())
}
