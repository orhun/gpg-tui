use crate::app::clipboard::CopyType;
use crate::app::command::Command;
use crate::app::launcher::App;
use crate::app::mode::Mode;
use crate::app::prompt::OutputType;
use crate::gpg::key::KeyType;
use crate::term::tui::Tui;
use crate::widget::row::ScrollDirection;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};
use std::str::FromStr;
use tui::backend::Backend;

/// Handle key events.
pub fn handle_key_input<B: Backend>(
	key_event: KeyEvent,
	tui: &mut Tui<B>,
	app: &mut App,
) -> Result<()> {
	let mut toggle_pause = false;
	if app.prompt.is_enabled() {
		match key_event.code {
			Key::Char(c) => {
				app.prompt.text.push(c);
				if app.prompt.is_search_enabled() {
					app.keys_table.reset_state();
				}
			}
			Key::Up => app.prompt.previous(),
			Key::Down => app.prompt.next(),
			Key::Tab => {
				if app.prompt.is_command_input_enabled() {
					app.prompt.enable_search();
				} else if app.prompt.is_search_enabled() {
					app.prompt.enable_command_input();
					app.keys_table.items = app.keys_table.default_items.clone();
				}
			}
			Key::Backspace => {
				app.prompt.text.pop();
				if app.prompt.is_search_enabled() {
					app.keys_table.reset_state();
				}
			}
			Key::Esc => {
				app.prompt.clear();
				if app.prompt.is_search_enabled() {
					app.keys_table.reset_state();
				}
			}
			Key::Enter => {
				if app.prompt.is_search_enabled() || app.prompt.text.len() < 2 {
					app.prompt.clear();
				} else if let Ok(mut command) =
					Command::from_str(&app.prompt.text)
				{
					app.prompt.history.push(app.prompt.text.clone());
					app.prompt.clear();
					if let Command::ExportKeys(_, _) = command {
						tui.toggle_pause()?;
						toggle_pause = true;
					} else if let Command::SwitchMode(mode) = command {
						match mode {
							Mode::Normal => tui.enable_mouse_capture()?,
							Mode::Visual => tui.disable_mouse_capture()?,
							_ => {}
						}
					} else if let Command::Copy(CopyType::Key) = command {
						if app.gpgme.config.armor {
							tui.toggle_pause()?;
							toggle_pause = true;
						} else {
							command = Command::ShowOutput(
								OutputType::Warning,
								String::from("enable armored output for copying the exported key(s)"),
							);
						}
					} else if let Command::Refresh = command {
						tui.enable_mouse_capture()?;
					}
					app.run_command(command)?;
				} else {
					app.prompt.set_output((
						OutputType::Error,
						format!(
							"invalid command: {}",
							app.prompt.text.replacen(":", "", 1)
						),
					));
				}
			}
			_ => {}
		}
	} else {
		app.run_command(match key_event.code {
			Key::Char('q') | Key::Char('Q') => Command::Quit,
			Key::Esc => {
				if app.mode != Mode::Normal {
					tui.enable_mouse_capture()?;
					Command::SwitchMode(Mode::Normal)
				} else {
					Command::Quit
				}
			}
			Key::Char('d') | Key::Char('D') => {
				if key_event.modifiers == Modifiers::CONTROL {
					Command::Quit
				} else {
					Command::None
				}
			}
			Key::Char('c') | Key::Char('C') => {
				if key_event.modifiers == Modifiers::CONTROL {
					Command::Quit
				} else {
					Command::SwitchMode(Mode::Copy)
				}
			}
			Key::Char('v') | Key::Char('V') => {
				if key_event.modifiers == Modifiers::CONTROL {
					Command::Paste
				} else {
					tui.disable_mouse_capture()?;
					Command::SwitchMode(Mode::Visual)
				}
			}
			Key::Char('r') | Key::Char('R') | Key::F(5) => {
				tui.enable_mouse_capture()?;
				Command::Refresh
			}
			Key::Up | Key::Char('k') | Key::Char('K') => {
				if key_event.modifiers == Modifiers::ALT {
					Command::Scroll(ScrollDirection::Up(1))
				} else {
					Command::Previous
				}
			}
			Key::Right | Key::Char('l') | Key::Char('L') => {
				if key_event.modifiers == Modifiers::ALT {
					Command::Scroll(ScrollDirection::Right(1))
				} else {
					Command::None
				}
			}
			Key::Down | Key::Char('j') | Key::Char('J') => {
				if key_event.modifiers == Modifiers::ALT {
					Command::Scroll(ScrollDirection::Down(1))
				} else {
					Command::Next
				}
			}
			Key::Left | Key::Char('h') | Key::Char('H') => {
				if key_event.modifiers == Modifiers::ALT {
					Command::Scroll(ScrollDirection::Left(1))
				} else {
					Command::None
				}
			}
			Key::Char('t') | Key::Char('T') => Command::ToggleDetail(true),
			Key::Tab => Command::ToggleDetail(false),
			Key::Char('`') => match app.command {
				Command::ListKeys(KeyType::Public) => {
					Command::ListKeys(KeyType::Secret)
				}
				_ => Command::ListKeys(KeyType::Public),
			},
			Key::Char('p') | Key::Char('P') => {
				Command::ListKeys(KeyType::Public)
			}
			Key::Char('s') | Key::Char('S') => {
				Command::ListKeys(KeyType::Secret)
			}
			Key::Char('e') | Key::Char('E') => {
				if app.mode == Mode::Copy {
					if app.gpgme.config.armor {
						tui.toggle_pause()?;
						toggle_pause = true;
						Command::Copy(CopyType::Key)
					} else {
						Command::ShowOutput(
							OutputType::Warning,
							String::from("enable armored output for copying the exported key(s)"),
						)
					}
				} else {
					tui.toggle_pause()?;
					toggle_pause = true;
					Command::ExportKeys(
						match app.command {
							Command::ListKeys(key_type) => key_type,
							_ => KeyType::Public,
						},
						vec![app.keys_table.items[app
							.keys_table
							.state
							.selected()
							.expect("invalid selection")]
						.get_id()],
					)
				}
			}
			Key::Char('a') | Key::Char('A') => Command::Set(
				String::from("armor"),
				(!app.gpgme.config.armor).to_string(),
			),
			Key::Char('n') | Key::Char('N') => {
				tui.enable_mouse_capture()?;
				Command::SwitchMode(Mode::Normal)
			}
			Key::Char('1') => {
				if app.mode == Mode::Copy {
					Command::Copy(CopyType::TableRow(1))
				} else {
					Command::Set(
						String::from("detail"),
						String::from("minimum"),
					)
				}
			}
			Key::Char('2') => {
				if app.mode == Mode::Copy {
					Command::Copy(CopyType::TableRow(2))
				} else {
					Command::Set(
						String::from("detail"),
						String::from("standard"),
					)
				}
			}
			Key::Char('3') => {
				Command::Set(String::from("detail"), String::from("full"))
			}
			Key::Char('i') | Key::Char('I') => {
				if app.mode == Mode::Copy {
					Command::Copy(CopyType::KeyId)
				} else {
					Command::None
				}
			}
			Key::Char('f') | Key::Char('F') => {
				if app.mode == Mode::Copy {
					Command::Copy(CopyType::KeyFingerprint)
				} else {
					Command::None
				}
			}
			Key::Char('u') | Key::Char('U') => {
				if app.mode == Mode::Copy {
					Command::Copy(CopyType::KeyUserId)
				} else {
					Command::None
				}
			}
			Key::Char('m') | Key::Char('M') => {
				if app.state.minimized {
					Command::Maximize
				} else {
					Command::Minimize
				}
			}
			Key::Char(':') => Command::EnableInput,
			Key::Char('/') => Command::Search(None),
			_ => Command::None,
		})?;
	}
	if toggle_pause {
		tui.toggle_pause()?;
	}
	Ok(())
}
