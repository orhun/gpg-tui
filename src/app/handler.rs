use crate::app::command::Command;
use crate::app::launcher::App;
use crate::app::mode::Mode;
use crate::app::prompt::OutputType;
use crate::app::selection::Selection;
use crate::app::tab::Tab;
use crate::app::util;
use crate::config::CustomKeyBinding;
use crate::gpg::key::KeyType;
use crate::term::tui::Tui;
use crate::widget::row::ScrollDirection;
use anyhow::Result;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};
use std::str::FromStr;
use tui::backend::Backend;

/// Handles the key events and executes the application command.
pub fn handle_events<B: Backend>(
	key_event: KeyEvent,
	key_bindings: &[CustomKeyBinding],
	tui: &mut Tui<B>,
	app: &mut App,
) -> Result<()> {
	handle_command_execution(
		handle_key_event(key_event, key_bindings, app),
		tui,
		app,
	)
}

/// Returns the corresponding application command for a key event.
fn handle_key_event(
	key_event: KeyEvent,
	key_bindings: &[CustomKeyBinding],
	app: &mut App,
) -> Command {
	let mut command = Command::None;
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
				} else if let Ok(cmd) = Command::from_str(&app.prompt.text) {
					app.prompt.history.push(app.prompt.text.clone());
					app.prompt.clear();
					command = cmd;
				} else {
					app.prompt.set_output((
						OutputType::Failure,
						format!(
							"invalid command: {}",
							app.prompt.text.replacen(':', "", 1)
						),
					));
				}
			}
			_ => {}
		}
	} else if let Some(key_binding) = key_bindings
		.iter()
		.find(|key_binding| key_binding.keys.contains(&key_event))
	{
		command = key_binding.command.clone();
	} else {
		command = match key_event.code {
			Key::Char('?') => Command::ShowHelp,
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
					match app.keys_table.selected() {
						Some(selected_key) => {
							Command::Confirm(Box::new(Command::DeleteKey(
								match app.tab {
									Tab::Keys(key_type) => key_type,
									_ => KeyType::Public,
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
			Key::PageUp => Command::Scroll(ScrollDirection::Top, false),
			Key::PageDown => Command::Scroll(ScrollDirection::Bottom, false),
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
					Command::ChangeStyle(app.state.style.next())
				} else {
					match app.keys_table.selected() {
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
				match app.keys_table.selected() {
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
					Command::Copy(Selection::Key)
				} else {
					match app.keys_table.selected() {
						Some(selected_key) => Command::ExportKeys(
							match app.tab {
								Tab::Keys(key_type) => key_type,
								_ => KeyType::Public,
							},
							vec![selected_key.get_id()],
							false,
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
					Command::Copy(Selection::Row1)
				} else {
					Command::Set(
						String::from("detail"),
						String::from("minimum"),
					)
				}
			}
			Key::Char('2') => {
				if app.mode == Mode::Copy {
					Command::Copy(Selection::Row2)
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
					Command::Copy(Selection::KeyId)
				} else {
					Command::Set(
						String::from("prompt"),
						String::from(":import "),
					)
				}
			}
			Key::Char('f') | Key::Char('F') => {
				if app.mode == Mode::Copy {
					Command::Copy(Selection::KeyFingerprint)
				} else {
					Command::Set(
						String::from("prompt"),
						String::from(":receive "),
					)
				}
			}
			Key::Char('u') | Key::Char('U') => {
				if app.mode == Mode::Copy {
					Command::Copy(Selection::KeyUserId)
				} else {
					match app.keys_table.selected() {
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
			Key::Char('m') | Key::Char('M') => Command::ToggleTableSize,
			Key::Char('y') | Key::Char('Y') => {
				if let Some(command) = &app.prompt.command {
					command.clone()
				} else {
					Command::None
				}
			}
			Key::Char('o') | Key::Char(' ') | Key::Enter => {
				if let Some(select_type) = app.state.select {
					Command::Copy(select_type)
				} else if app.state.show_options {
					app.options.selected().cloned().unwrap_or(Command::None)
				} else {
					Command::ShowOptions
				}
			}
			Key::Char(':') => Command::EnableInput,
			Key::Char('/') => Command::Search(None),
			_ => Command::None,
		};
	}
	command
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
	if app.state.show_splash && command != Command::Quit {
		command = Command::None;
	}
	if let Tab::Help = app.tab {
		match command {
			Command::ShowOptions
			| Command::ChangeStyle(_)
			| Command::Scroll(_, _)
			| Command::ListKeys(_)
			| Command::SwitchMode(_)
			| Command::Paste
			| Command::EnableInput
			| Command::NextTab
			| Command::PreviousTab
			| Command::Refresh
			| Command::Quit
			| Command::None => {}
			Command::Set(ref option, _) => {
				if option != "style" {
					command = Command::None
				}
			}
			_ => command = Command::None,
		}
	}
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
			} else if option == "prompt" && value == ":import " {
				tui.toggle_pause()?;
				toggle_pause = true;
				match util::run_os_command(&app.state.file_explorer) {
					Ok(files) => {
						command = Command::ImportKeys(files, false);
					}
					Err(e) => eprintln!("{e:?}"),
				}
			}
		}
		Command::ExportKeys(_, _, _)
		| Command::DeleteKey(_, _)
		| Command::GenerateKey
		| Command::RefreshKeys
		| Command::EditKey(_)
		| Command::SignKey(_)
		| Command::ImportKeys(_, true) => {
			tui.toggle_pause()?;
			toggle_pause = true;
		}
		Command::Copy(Selection::Key) => {
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

#[cfg(feature = "gpg-tests")]
#[cfg(test)]
mod tests {
	use super::*;
	use crate::app::command::Command;
	use crate::app::style::Style;
	use crate::args::Args;
	use crate::gpg::config::GpgConfig;
	use crate::gpg::context::GpgContext;
	use pretty_assertions::assert_eq;
	use std::env;
	#[test]
	fn test_app_handler() -> Result<()> {
		env::set_var(
			"GNUPGHOME",
			dirs_next::cache_dir()
				.unwrap()
				.join(env!("CARGO_PKG_NAME"))
				.to_str()
				.unwrap(),
		);
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		let mut context = GpgContext::new(config)?;
		let mut app = App::new(&mut context, &args)?;
		let key_id =
			app.gpgme.get_all_keys(None)?.get(&KeyType::Public).unwrap()[0]
				.get_id();
		let test_cases = vec![
			(
				Command::Confirm(Box::new(Command::DeleteKey(
					KeyType::Public,
					key_id.to_string(),
				))),
				vec![
					KeyEvent::new(Key::Char('d'), Modifiers::NONE),
					KeyEvent::new(Key::Backspace, Modifiers::NONE),
				],
			),
			(
				Command::Confirm(Box::new(Command::SendKey(
					key_id.to_string(),
				))),
				vec![KeyEvent::new(Key::Char('u'), Modifiers::NONE)],
			),
			(
				Command::ExportKeys(
					KeyType::Public,
					vec![key_id.to_string()],
					false,
				),
				vec![KeyEvent::new(Key::Char('x'), Modifiers::NONE)],
			),
			(
				Command::EditKey(key_id.to_string()),
				vec![KeyEvent::new(Key::Char('e'), Modifiers::NONE)],
			),
			(
				Command::SignKey(key_id),
				vec![KeyEvent::new(Key::Char('s'), Modifiers::NONE)],
			),
			(
				Command::ShowHelp,
				vec![KeyEvent::new(Key::Char('?'), Modifiers::NONE)],
			),
			(
				Command::ShowOptions,
				vec![
					KeyEvent::new(Key::Char('o'), Modifiers::NONE),
					KeyEvent::new(Key::Char(' '), Modifiers::NONE),
					KeyEvent::new(Key::Enter, Modifiers::NONE),
				],
			),
			(
				Command::GenerateKey,
				vec![KeyEvent::new(Key::Char('g'), Modifiers::NONE)],
			),
			(
				Command::RefreshKeys,
				vec![KeyEvent::new(Key::Char('r'), Modifiers::CONTROL)],
			),
			(
				Command::ToggleDetail(true),
				vec![KeyEvent::new(Key::Char('t'), Modifiers::NONE)],
			),
			(
				Command::ToggleDetail(false),
				vec![KeyEvent::new(Key::Tab, Modifiers::NONE)],
			),
			(
				Command::Scroll(ScrollDirection::Top, false),
				vec![
					KeyEvent::new(Key::Up, Modifiers::CONTROL),
					KeyEvent::new(Key::Char('k'), Modifiers::CONTROL),
					KeyEvent::new(Key::PageUp, Modifiers::NONE),
				],
			),
			(
				Command::Scroll(ScrollDirection::Up(1), false),
				vec![
					KeyEvent::new(Key::Up, Modifiers::NONE),
					KeyEvent::new(Key::Char('k'), Modifiers::NONE),
				],
			),
			(
				Command::Scroll(ScrollDirection::Right(1), true),
				vec![
					KeyEvent::new(Key::Right, Modifiers::ALT),
					KeyEvent::new(Key::Char('l'), Modifiers::ALT),
				],
			),
			(
				Command::Scroll(ScrollDirection::Bottom, false),
				vec![
					KeyEvent::new(Key::Down, Modifiers::CONTROL),
					KeyEvent::new(Key::Char('j'), Modifiers::CONTROL),
					KeyEvent::new(Key::PageDown, Modifiers::NONE),
				],
			),
			(
				Command::Scroll(ScrollDirection::Down(1), false),
				vec![
					KeyEvent::new(Key::Down, Modifiers::NONE),
					KeyEvent::new(Key::Char('j'), Modifiers::NONE),
				],
			),
			(
				Command::Scroll(ScrollDirection::Left(1), true),
				vec![
					KeyEvent::new(Key::Left, Modifiers::ALT),
					KeyEvent::new(Key::Char('h'), Modifiers::ALT),
				],
			),
			(
				Command::Set(String::from("margin"), String::from("0")),
				vec![KeyEvent::new(Key::Char('`'), Modifiers::NONE)],
			),
			(
				Command::ChangeStyle(Style::Colored),
				vec![KeyEvent::new(Key::Char('s'), Modifiers::CONTROL)],
			),
			(
				Command::Set(String::from("armor"), String::from("true")),
				vec![KeyEvent::new(Key::Char('a'), Modifiers::NONE)],
			),
			(
				Command::Set(String::from("detail"), String::from("minimum")),
				vec![KeyEvent::new(Key::Char('1'), Modifiers::NONE)],
			),
			(
				Command::Set(String::from("detail"), String::from("standard")),
				vec![KeyEvent::new(Key::Char('2'), Modifiers::NONE)],
			),
			(
				Command::Set(String::from("detail"), String::from("full")),
				vec![KeyEvent::new(Key::Char('3'), Modifiers::NONE)],
			),
			(
				Command::Set(String::from("prompt"), String::from(":import ")),
				vec![KeyEvent::new(Key::Char('i'), Modifiers::NONE)],
			),
			(
				Command::Set(String::from("prompt"), String::from(":receive ")),
				vec![KeyEvent::new(Key::Char('f'), Modifiers::NONE)],
			),
			(
				Command::ToggleTableSize,
				vec![KeyEvent::new(Key::Char('m'), Modifiers::NONE)],
			),
			(
				Command::SwitchMode(Mode::Normal),
				vec![KeyEvent::new(Key::Char('n'), Modifiers::NONE)],
			),
			(
				Command::SwitchMode(Mode::Visual),
				vec![KeyEvent::new(Key::Char('v'), Modifiers::NONE)],
			),
			(
				Command::SwitchMode(Mode::Copy),
				vec![KeyEvent::new(Key::Char('c'), Modifiers::NONE)],
			),
			(
				Command::Paste,
				vec![KeyEvent::new(Key::Char('v'), Modifiers::CONTROL)],
			),
			(
				Command::Paste,
				vec![KeyEvent::new(Key::Char('p'), Modifiers::NONE)],
			),
			(
				Command::EnableInput,
				vec![KeyEvent::new(Key::Char(':'), Modifiers::CONTROL)],
			),
			(
				Command::Search(None),
				vec![KeyEvent::new(Key::Char('/'), Modifiers::CONTROL)],
			),
			(
				Command::NextTab,
				vec![
					KeyEvent::new(Key::Right, Modifiers::CONTROL),
					KeyEvent::new(Key::Char('l'), Modifiers::NONE),
				],
			),
			(
				Command::PreviousTab,
				vec![
					KeyEvent::new(Key::Left, Modifiers::CONTROL),
					KeyEvent::new(Key::Char('h'), Modifiers::NONE),
				],
			),
			(
				Command::Refresh,
				vec![
					KeyEvent::new(Key::Char('r'), Modifiers::NONE),
					KeyEvent::new(Key::F(5), Modifiers::NONE),
				],
			),
			(
				Command::Quit,
				vec![
					KeyEvent::new(Key::Char('q'), Modifiers::NONE),
					KeyEvent::new(Key::Esc, Modifiers::NONE),
					KeyEvent::new(Key::Char('d'), Modifiers::CONTROL),
					KeyEvent::new(Key::Char('c'), Modifiers::CONTROL),
				],
			),
			(
				Command::None,
				vec![KeyEvent::new(Key::Char('y'), Modifiers::NONE)],
			),
			(
				Command::None,
				vec![KeyEvent::new(Key::Char('ö'), Modifiers::NONE)],
			),
		];
		for (command, key_events) in test_cases {
			for key_event in key_events {
				assert_eq!(command, handle_key_event(key_event, &[], &mut app));
			}
		}
		app.prompt.enable_command_input();
		handle_key_event(
			KeyEvent::new(Key::Esc, Modifiers::NONE),
			&[],
			&mut app,
		);
		assert!(!app.prompt.is_enabled());
		app.prompt.enable_search();
		handle_key_event(
			KeyEvent::new(Key::Tab, Modifiers::NONE),
			&[],
			&mut app,
		);
		for c in String::from("normal-").chars() {
			handle_key_event(
				KeyEvent::new(Key::Char(c), Modifiers::NONE),
				&[],
				&mut app,
			);
		}
		handle_key_event(
			KeyEvent::new(Key::Backspace, Modifiers::NONE),
			&[],
			&mut app,
		);
		assert_eq!(":normal", app.prompt.text);
		assert_eq!(
			Command::SwitchMode(Mode::Normal),
			handle_key_event(
				KeyEvent::new(Key::Enter, Modifiers::NONE),
				&[],
				&mut app,
			)
		);
		app.prompt.enable_command_input();
		handle_key_event(
			KeyEvent::new(Key::Down, Modifiers::NONE),
			&[],
			&mut app,
		);
		handle_key_event(
			KeyEvent::new(Key::Up, Modifiers::NONE),
			&[],
			&mut app,
		);
		assert_eq!(":normal", app.prompt.text);
		handle_key_event(
			KeyEvent::new(Key::Enter, Modifiers::NONE),
			&[],
			&mut app,
		);
		assert_eq!(
			Command::SwitchMode(Mode::Visual),
			handle_key_event(
				KeyEvent::new(Key::Char('M'), Modifiers::NONE),
				&[CustomKeyBinding {
					keys: vec![KeyEvent::new(Key::Char('M'), Modifiers::NONE)],
					command: Command::SwitchMode(Mode::Visual),
				}],
				&mut app,
			)
		);
		Ok(())
	}
}
