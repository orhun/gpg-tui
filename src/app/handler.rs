use crate::app::command::Command;
use crate::app::launcher::App;
use crate::widget::row::ScrollDirection;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};
use std::str::FromStr;

/// Handle key events.
pub fn handle_key_input(key_event: KeyEvent, app: &mut App) -> Result<()> {
	if !app.state.input.is_empty() {
		match key_event.code {
			Key::Char(c) => {
				app.state.input.push(c);
			}
			Key::Enter => {
				if let Ok(command) = Command::from_str(&app.state.input) {
					app.command = command;
					app.state.input.clear();
				}
			}
			Key::Backspace => {
				app.state.input.pop();
			}
			Key::Esc => {
				app.state.input.clear();
			}
			_ => {}
		}
	} else {
		match key_event.code {
			Key::Char('q') | Key::Char('Q') | Key::Esc => {
				app.state.running = false
			}
			Key::Char('c') | Key::Char('d') => {
				if key_event.modifiers == Modifiers::CONTROL {
					app.state.running = false;
				}
			}
			Key::Char('r') | Key::Char('R') | Key::F(5) => app.refresh()?,
			Key::Up | Key::Char('k') | Key::Char('K') => {
				if key_event.modifiers == Modifiers::ALT {
					app.key_list.scroll(ScrollDirection::Up(1))
				} else {
					app.key_list.previous();
				}
			}
			Key::Right | Key::Char('l') | Key::Char('L') => {
				if key_event.modifiers == Modifiers::ALT {
					app.key_list.scroll(ScrollDirection::Right(1))
				}
			}
			Key::Down | Key::Char('j') | Key::Char('J') => {
				if key_event.modifiers == Modifiers::ALT {
					app.key_list.scroll(ScrollDirection::Down(1))
				} else {
					app.key_list.next();
				}
			}
			Key::Left | Key::Char('h') | Key::Char('H') => {
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
			Key::Char(':') => {
				app.state.input = String::from(":");
			}
			_ => {}
		}
	}
	Ok(())
}
