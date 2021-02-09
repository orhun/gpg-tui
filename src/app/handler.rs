use crate::app::clipboard::CopyType;
use crate::app::command::Command;
use crate::app::launcher::App;
use crate::app::mode::AppMode;
use crate::gpg::key::KeyType;
use crate::term::tui::Tui;
use crate::widget::row::ScrollDirection;
use anyhow::Result;
use copypasta_ext::prelude::ClipboardProvider;
use crossterm::event::{KeyCode as Key, KeyEvent, KeyModifiers as Modifiers};
use std::str::FromStr;
use tui::backend::Backend;

/// Handle key events.
pub fn handle_key_input<B: Backend>(
	key_event: KeyEvent,
	tui: &mut Tui<B>,
	app: &mut App,
) -> Result<()> {
	if app.prompt.is_enabled() {
		match key_event.code {
			Key::Tab => {
				if app.prompt.is_command_input_enabled() {
					app.prompt.enable_search();
				} else if app.prompt.is_search_enabled() {
					app.prompt.enable_command_input();
					app.key_list.items = app.key_list.default_items.clone();
				}
			}
			Key::Char(c) => {
				app.prompt.text.push(c);
				if app.prompt.is_search_enabled() {
					app.key_list.reset_state();
				}
			}
			Key::Backspace => {
				app.prompt.text.pop();
				if app.prompt.is_search_enabled() {
					app.key_list.reset_state();
				}
			}
			Key::Esc => {
				app.prompt.clear();
				if app.prompt.is_search_enabled() {
					app.key_list.reset_state();
				}
			}
			Key::Enter => {
				if app.prompt.is_search_enabled() || app.prompt.text.len() < 2 {
					app.prompt.clear();
				} else if let Ok(command) = Command::from_str(&app.prompt.text)
				{
					app.prompt.clear();
					if let Command::ExportKeys(_, _) = command {
						tui.toggle_pause()?;
						app.run_command(command)?;
						tui.toggle_pause()?;
					} else if let Command::SwitchMode(mode) = command {
						match mode {
							AppMode::Normal => tui.enable_mouse_capture()?,
							AppMode::Visual => tui.disable_mouse_capture()?,
							_ => {}
						}
						app.run_command(command)?;
					} else {
						app.run_command(command)?;
					}
				} else {
					app.prompt.set_output(format!(
						"Invalid command: {}",
						app.prompt.text.replacen(":", "", 1)
					));
				}
			}
			_ => {}
		}
	} else {
		match key_event.code {
			Key::Char('q') | Key::Char('Q') | Key::Esc => {
				app.state.running = false
			}
			Key::Char('d') | Key::Char('D') => {
				if key_event.modifiers == Modifiers::CONTROL {
					app.state.running = false;
				}
			}
			Key::Char('c') | Key::Char('C') => {
				if key_event.modifiers == Modifiers::CONTROL {
					app.state.running = false;
				} else if app.state.mode == AppMode::Copy {
					app.run_command(Command::Copy(CopyType::KeyFingerprint))?;
				} else {
					app.run_command(Command::SwitchMode(AppMode::Copy))?;
				}
			}
			Key::Char('v') | Key::Char('V') => {
				if key_event.modifiers == Modifiers::CONTROL {
					app.prompt.text = format!(
						":{}",
						app.clipboard
							.get_contents()
							.expect("failed to get clipboard contents")
					);
				} else {
					tui.disable_mouse_capture()?;
					app.run_command(Command::SwitchMode(AppMode::Visual))?;
				}
			}
			Key::Char('r') | Key::Char('R') | Key::F(5) => {
				app.refresh()?;
				tui.enable_mouse_capture()?;
			}
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
				for key in app.key_list.default_items.iter_mut() {
					key.detail = app.state.table_detail;
				}
			}
			Key::Tab => {
				if let Some(index) = app.key_list.state.selected() {
					if let Some(key) = app.key_list.items.get_mut(index) {
						key.detail.increase()
					}
					if app.key_list.items.len()
						== app.key_list.default_items.len()
					{
						if let Some(key) =
							app.key_list.default_items.get_mut(index)
						{
							key.detail.increase()
						}
					}
				}
			}
			Key::Char('`') => app.run_command(match app.command {
				Command::ListKeys(KeyType::Public) => {
					Command::ListKeys(KeyType::Secret)
				}
				_ => Command::ListKeys(KeyType::Public),
			})?,
			Key::Char('p') | Key::Char('P') => {
				app.run_command(Command::ListKeys(KeyType::Public))?
			}
			Key::Char('s') | Key::Char('S') => {
				app.run_command(Command::ListKeys(KeyType::Secret))?
			}
			Key::Char('e') | Key::Char('E') => {
				tui.toggle_pause()?;
				app.run_command(Command::ExportKeys(
					match app.command {
						Command::ListKeys(key_type) => key_type,
						_ => KeyType::Public,
					},
					vec![app.key_list.items[app
						.key_list
						.state
						.selected()
						.expect("invalid selection")]
					.get_id()],
				))?;
				tui.toggle_pause()?;
			}
			Key::Char('a') | Key::Char('A') => {
				app.run_command(Command::Set(
					String::from("armor"),
					(!app.gpgme.config.armor).to_string(),
				))?;
			}
			Key::Char('n') | Key::Char('N') => {
				tui.enable_mouse_capture()?;
				app.run_command(Command::SwitchMode(AppMode::Normal))?;
			}
			Key::Char('1') => {
				if app.state.mode == AppMode::Copy {
					app.run_command(Command::Copy(CopyType::TableRow(1)))?;
				}
			}
			Key::Char('2') => {
				if app.state.mode == AppMode::Copy {
					app.run_command(Command::Copy(CopyType::TableRow(2)))?;
				}
			}
			Key::Char('i') | Key::Char('I') => {
				if app.state.mode == AppMode::Copy {
					app.run_command(Command::Copy(CopyType::KeyId))?;
				}
			}
			Key::Char('f') | Key::Char('F') => {
				if app.state.mode == AppMode::Copy {
					app.run_command(Command::Copy(CopyType::KeyFingerprint))?;
				}
			}
			Key::Char('u') | Key::Char('U') => {
				if app.state.mode == AppMode::Copy {
					app.run_command(Command::Copy(CopyType::KeyUserId))?;
				}
			}
			Key::Char(':') => app.prompt.enable_command_input(),
			Key::Char('/') => {
				app.prompt.enable_search();
				app.key_list.items = app.key_list.default_items.clone();
			}
			_ => {}
		}
	}
	Ok(())
}
