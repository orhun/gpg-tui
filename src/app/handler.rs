use crate::app::clipboard::CopyType;
use crate::app::command::Command;
use crate::app::launcher::App;
use crate::app::mode::Mode;
use crate::app::prompt::OutputType;
use crate::app::tab::Tab;
use crate::term::tui::Tui;
use crate::widget::row::ScrollDirection;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};
use std::str::FromStr;
use tui::backend::Backend;

/// Handles the key events and
/// runs an application command if necessary.
pub fn handle_key_input<B: Backend>(
	key_event: KeyEvent,
	tui: &mut Tui<B>,
	app: &mut App,
) -> Result<()> {
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
				} else if let Ok(command) = Command::from_str(&app.prompt.text)
				{
					app.prompt.history.push(app.prompt.text.clone());
					app.prompt.clear();
					handle_command_execution(command, tui, app)?;
				} else {
					app.prompt.set_output((
						OutputType::Failure,
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
		handle_command_execution(
			match key_event.code {
				Key::Char('q') | Key::Char('Q') => Command::Quit,
				Key::Esc => {
					if app.mode != Mode::Normal {
						Command::SwitchMode(Mode::Normal)
					} else if app.state.show_options {
						Command::None
					} else if app.prompt.command.is_some() {
						app.prompt.clear();
						Command::None
					} else {
						Command::Quit
					}
				}
				Key::Char('d') | Key::Char('D') | Key::Backspace => {
					if key_event.modifiers == Modifiers::CONTROL
						&& key_event.code != Key::Backspace
					{
						Command::Quit
					} else {
						match app.keys_table.items.get(
							app.keys_table
								.state
								.tui
								.selected()
								.expect("invalid selection"),
						) {
							Some(selected_key) => {
								Command::Confirm(Box::new(Command::DeleteKey(
									match app.tab {
										Tab::Keys(key_type) => key_type,
									},
									selected_key.get_id(),
								)))
							}
							None => Command::ShowOutput(
								OutputType::Failure,
								String::from("invalid selection"),
							),
						}
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
						Command::SwitchMode(Mode::Visual)
					}
				}
				Key::Char('p') | Key::Char('P') => Command::Paste,
				Key::Char('r') | Key::Char('R') | Key::F(5) => {
					if key_event.modifiers == Modifiers::CONTROL {
						Command::RefreshKeys
					} else {
						Command::Refresh
					}
				}
				Key::Up | Key::Char('k') | Key::Char('K') => {
					if key_event.modifiers == Modifiers::CONTROL {
						Command::Scroll(ScrollDirection::Top, false)
					} else {
						Command::Scroll(
							ScrollDirection::Up(1),
							key_event.modifiers == Modifiers::ALT,
						)
					}
				}
				Key::Right | Key::Char('l') | Key::Char('L') => {
					if key_event.modifiers == Modifiers::ALT {
						Command::Scroll(ScrollDirection::Right(1), true)
					} else {
						Command::NextTab
					}
				}
				Key::Down | Key::Char('j') | Key::Char('J') => {
					if key_event.modifiers == Modifiers::CONTROL {
						Command::Scroll(ScrollDirection::Bottom, false)
					} else {
						Command::Scroll(
							ScrollDirection::Down(1),
							key_event.modifiers == Modifiers::ALT,
						)
					}
				}
				Key::Left | Key::Char('h') | Key::Char('H') => {
					if key_event.modifiers == Modifiers::ALT {
						Command::Scroll(ScrollDirection::Left(1), true)
					} else {
						Command::PreviousTab
					}
				}
				Key::Char('t') | Key::Char('T') => Command::ToggleDetail(true),
				Key::Tab => Command::ToggleDetail(false),
				Key::Char('`') => Command::Set(
					String::from("margin"),
					String::from(if app.keys_table_margin == 1 {
						"0"
					} else {
						"1"
					}),
				),
				Key::Char('s') | Key::Char('S') => {
					if key_event.modifiers == Modifiers::CONTROL {
						Command::Set(
							String::from("colored"),
							(!app.state.colored).to_string(),
						)
					} else {
						match app.keys_table.items.get(
							app.keys_table
								.state
								.tui
								.selected()
								.expect("invalid selection"),
						) {
							Some(selected_key) => {
								Command::SignKey(selected_key.get_id())
							}
							None => Command::ShowOutput(
								OutputType::Failure,
								String::from("invalid selection"),
							),
						}
					}
				}
				Key::Char('e') | Key::Char('E') => {
					match app.keys_table.items.get(
						app.keys_table
							.state
							.tui
							.selected()
							.expect("invalid selection"),
					) {
						Some(selected_key) => {
							Command::EditKey(selected_key.get_id())
						}
						None => Command::ShowOutput(
							OutputType::Failure,
							String::from("invalid selection"),
						),
					}
				}
				Key::Char('x') | Key::Char('X') => {
					if app.mode == Mode::Copy {
						Command::Copy(CopyType::Key)
					} else {
						match app.keys_table.items.get(
							app.keys_table
								.state
								.tui
								.selected()
								.expect("invalid selection"),
						) {
							Some(selected_key) => Command::ExportKeys(
								match app.tab {
									Tab::Keys(key_type) => key_type,
								},
								vec![selected_key.get_id()],
							),
							None => Command::ShowOutput(
								OutputType::Failure,
								String::from("invalid selection"),
							),
						}
					}
				}
				Key::Char('g') | Key::Char('G') => Command::GenerateKey,
				Key::Char('a') | Key::Char('A') => Command::Set(
					String::from("armor"),
					(!app.gpgme.config.armor).to_string(),
				),
				Key::Char('n') | Key::Char('N') => {
					if app.prompt.command.is_some() {
						app.prompt.clear();
						Command::None
					} else {
						Command::SwitchMode(Mode::Normal)
					}
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
						Command::Set(
							String::from("prompt"),
							String::from(":import "),
						)
					}
				}
				Key::Char('f') | Key::Char('F') => {
					if app.mode == Mode::Copy {
						Command::Copy(CopyType::KeyFingerprint)
					} else {
						Command::Set(
							String::from("prompt"),
							String::from(":receive "),
						)
					}
				}
				Key::Char('u') | Key::Char('U') => {
					if app.mode == Mode::Copy {
						Command::Copy(CopyType::KeyUserId)
					} else {
						match app.keys_table.items.get(
							app.keys_table
								.state
								.tui
								.selected()
								.expect("invalid selection"),
						) {
							Some(selected_key) => Command::Confirm(Box::new(
								Command::SendKey(selected_key.get_id()),
							)),
							None => Command::ShowOutput(
								OutputType::Failure,
								String::from("invalid selection"),
							),
						}
					}
				}
				Key::Char('m') | Key::Char('M') => {
					if app.state.minimized {
						Command::Maximize
					} else {
						Command::Minimize
					}
				}
				Key::Char('y') | Key::Char('Y') => {
					if let Some(command) = &app.prompt.command {
						command.clone()
					} else {
						Command::None
					}
				}
				Key::Char('o') | Key::Char(' ') | Key::Enter => {
					if app.state.show_options {
						app.options
							.items
							.get(
								app.options
									.state
									.selected()
									.expect("invalid selection"),
							)
							.cloned()
							.unwrap_or(Command::None)
					} else if !app.keys_table.items.is_empty() {
						Command::ShowOptions
					} else {
						Command::None
					}
				}
				Key::Char(':') => Command::EnableInput,
				Key::Char('/') => Command::Search(None),
				_ => Command::None,
			},
			tui,
			app,
		)?;
	}
	Ok(())
}

/// Handles the execution of an application command.
///
/// It checks the additional conditions for determining
/// if the execution of the given command is applicable.
/// For example, depending on the command, it toggles the
/// [`paused`] state of [`Tui`] or enables/disables the mouse capture.
///
/// [`Tui`]: Tui
/// [`paused`]: Tui::paused
fn handle_command_execution<B: Backend>(
	mut command: Command,
	tui: &mut Tui<B>,
	app: &mut App,
) -> Result<()> {
	let mut toggle_pause = false;
	match command {
		Command::SwitchMode(Mode::Normal) | Command::Refresh => {
			tui.enable_mouse_capture()?
		}
		Command::SwitchMode(Mode::Visual) => tui.disable_mouse_capture()?,
		Command::Set(ref option, ref value) => {
			if option == "mode" {
				match Mode::from_str(value) {
					Ok(Mode::Normal) => tui.enable_mouse_capture()?,
					Ok(Mode::Visual) => tui.disable_mouse_capture()?,
					_ => {}
				}
			}
		}
		Command::ExportKeys(_, _)
		| Command::DeleteKey(_, _)
		| Command::GenerateKey
		| Command::RefreshKeys
		| Command::EditKey(_)
		| Command::SignKey(_)
		| Command::ImportKeys(_, true) => {
			tui.toggle_pause()?;
			toggle_pause = true;
		}
		Command::ListKeys(key_type) => match app.tab {
			Tab::Keys(tab_key_type) => {
				if key_type == tab_key_type {
					command = Command::Refresh
				}
			}
		},
		Command::Copy(CopyType::Key) => {
			if app.gpgme.config.armor {
				tui.toggle_pause()?;
				toggle_pause = true;
			} else {
				command = Command::ShowOutput(
					OutputType::Warning,
					String::from(
						"enable armored output for copying the exported key(s)",
					),
				);
			}
		}
		_ => {}
	}
	app.run_command(command)?;
	if toggle_pause {
		tui.toggle_pause()?;
	}
	Ok(())
}
