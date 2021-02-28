use crate::app::clipboard::CopyType;
use crate::app::command::Command;
use crate::app::mode::Mode;
use crate::app::prompt::{OutputType, Prompt};
use crate::app::state::State;
use crate::gpg::context::GpgContext;
use crate::gpg::key::{GpgKey, KeyDetail, KeyType};
use crate::widget::row::RowItem;
use crate::widget::table::StatefulTable;
use anyhow::Result;
use copypasta_ext::prelude::ClipboardProvider;
use copypasta_ext::x11_fork::ClipboardContext;
use std::cmp;
use std::convert::TryInto;
use std::str;
use std::str::FromStr;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Text};
use tui::widgets::{Block, Borders, Paragraph, Row, Table, Wrap};
use unicode_width::UnicodeWidthStr;

/// Lengths of keys row in minimized/maximized mode.
const KEYS_ROW_LENGTH: (u16, u16) = (31, 55);
/// Max duration of prompt messages (in seconds).
const MESSAGE_DURATION: u64 = 1;

/// Main application.
///
/// It operates the TUI via rendering the widgets
/// and updating the application state.
pub struct App<'a> {
	/// Application state.
	pub state: State,
	/// Application mode.
	pub mode: Mode,
	/// Prompt manager.
	pub prompt: Prompt,
	/// Current command.
	pub command: Command,
	/// Table of public/secret keys.
	pub keys_table: StatefulTable<GpgKey>,
	/// Level of detail to show for keys table.
	pub keys_table_detail: KeyDetail,
	/// Bottom margin value of the keys table.
	pub keys_table_margin: u16,
	/// Clipboard context.
	pub clipboard: ClipboardContext,
	/// GPGME context.
	pub gpgme: &'a mut GpgContext,
}

impl<'a> App<'a> {
	/// Constructs a new instance of `App`.
	pub fn new(gpgme: &'a mut GpgContext) -> Result<Self> {
		Ok(Self {
			state: State::default(),
			mode: Mode::Normal,
			prompt: Prompt::default(),
			command: Command::ListKeys(KeyType::Public),
			keys_table: StatefulTable::with_items(
				gpgme.get_keys(KeyType::Public, None)?,
			),
			keys_table_detail: KeyDetail::Minimum,
			keys_table_margin: 1,
			clipboard: ClipboardContext::new()
				.expect("failed to initialize clipboard"),
			gpgme,
		})
	}

	/// Resets the application state.
	pub fn refresh(&mut self) -> Result<()> {
		self.state = State::default();
		self.mode = Mode::Normal;
		self.prompt = Prompt::default();
		self.keys_table_detail = KeyDetail::Minimum;
		self.run_command(Command::ListKeys(KeyType::Public))
	}

	/// Handles the tick event of the application.
	///
	/// It is currently used to flush the prompt messages.
	pub fn tick(&mut self) {
		if let Some(clock) = self.prompt.clock {
			if clock.elapsed().as_secs() > MESSAGE_DURATION {
				self.prompt.clear()
			}
		}
	}

	/// Runs the given command which is used to specify
	/// the widget to render or action to perform.
	pub fn run_command(&mut self, command: Command) -> Result<()> {
		match command {
			Command::ShowOutput(output_type, message) => {
				self.prompt.set_output((output_type, message))
			}
			Command::ListKeys(key_type) => {
				self.keys_table = StatefulTable::with_items(
					self.gpgme.get_keys(key_type, None)?,
				);
				self.command = command;
			}
			Command::ExportKeys(key_type, ref patterns) => {
				self.prompt.set_output(
					match self
						.gpgme
						.export_keys(key_type, Some(patterns.to_vec()))
					{
						Ok(path) => {
							(OutputType::Success, format!("export: {}", path))
						}
						Err(e) => (
							OutputType::Failure,
							format!("export error: {}", e),
						),
					},
				);
			}
			Command::ToggleDetail(true) => {
				self.keys_table_detail.increase();
				for key in self.keys_table.items.iter_mut() {
					key.detail = self.keys_table_detail;
				}
				for key in self.keys_table.default_items.iter_mut() {
					key.detail = self.keys_table_detail;
				}
			}
			Command::ToggleDetail(false) => {
				if let Some(index) = self.keys_table.state.selected() {
					if let Some(key) = self.keys_table.items.get_mut(index) {
						key.detail.increase()
					}
					if self.keys_table.items.len()
						== self.keys_table.default_items.len()
					{
						if let Some(key) =
							self.keys_table.default_items.get_mut(index)
						{
							key.detail.increase()
						}
					}
				}
			}
			Command::Scroll(direction) => {
				self.keys_table.scroll(direction);
			}
			Command::Set(option, value) => self.prompt.set_output(match option
				.as_str()
			{
				"mode" => {
					if let Ok(mode) = Mode::from_str(&value) {
						self.mode = mode;
						(OutputType::Success, format!("mode: {:?}", mode))
					} else {
						(OutputType::Failure, String::from("invalid mode"))
					}
				}
				"armor" => {
					if let Ok(value) = FromStr::from_str(&value) {
						self.gpgme.config.armor = value;
						self.gpgme.apply_config();
						(OutputType::Success, format!("armor: {}", value))
					} else {
						(
							OutputType::Failure,
							String::from("usage: set armor <true/false>"),
						)
					}
				}
				"minimize" => {
					self.state.minimize_threshold =
						value.parse().unwrap_or_default();
					(
						OutputType::Success,
						format!("minimize threshold: {}", value),
					)
				}
				"detail" => {
					if let Ok(detail_level) = KeyDetail::from_str(&value) {
						if let Some(index) = self.keys_table.state.selected() {
							if let Some(key) =
								self.keys_table.items.get_mut(index)
							{
								key.detail = detail_level;
							}
							if self.keys_table.items.len()
								== self.keys_table.default_items.len()
							{
								if let Some(key) =
									self.keys_table.default_items.get_mut(index)
								{
									key.detail = detail_level;
								}
							}
						}
						(
							OutputType::Success,
							format!("detail: {}", detail_level),
						)
					} else {
						(
							OutputType::Failure,
							String::from("usage: set detail <level>"),
						)
					}
				}
				"margin" => {
					self.keys_table_margin = value.parse().unwrap_or_default();
					(
						OutputType::Success,
						format!("table margin: {}", self.keys_table_margin),
					)
				}
				_ => (
					OutputType::Failure,
					if !option.is_empty() {
						format!("unknown option: {}", option)
					} else {
						String::from("Usage: set <option> <value>")
					},
				),
			}),
			Command::Get(option) => {
				self.prompt.set_output(match option.as_str() {
					"mode" => {
						(OutputType::Success, format!("mode: {:?}", self.mode))
					}
					"armor" => (
						OutputType::Success,
						format!("armor: {}", self.gpgme.config.armor),
					),
					"minimize" => (
						OutputType::Success,
						format!(
							"minimize threshold: {}",
							self.state.minimize_threshold
						),
					),
					"detail" => {
						if let Some(index) = self.keys_table.state.selected() {
							if let Some(key) = self.keys_table.items.get(index)
							{
								(
									OutputType::Success,
									format!("detail: {}", key.detail),
								)
							} else {
								(
									OutputType::Failure,
									String::from("invalid selection"),
								)
							}
						} else {
							(
								OutputType::Failure,
								String::from("unknown selection"),
							)
						}
					}
					"margin" => (
						OutputType::Success,
						format!("table margin: {}", self.keys_table_margin),
					),
					_ => (
						OutputType::Failure,
						if !option.is_empty() {
							format!("unknown option: {}", option)
						} else {
							String::from("usage: get <option>")
						},
					),
				})
			}
			Command::SwitchMode(mode) => {
				self.mode = mode;
				self.prompt
					.set_output((OutputType::Action, mode.to_string()))
			}
			Command::Copy(copy_type) => {
				let selected_key = &self.keys_table.items[self
					.keys_table
					.state
					.selected()
					.expect("invalid selection")];
				self.clipboard
					.set_contents(match copy_type {
						CopyType::TableRow(1) => selected_key
							.get_subkey_info(self.state.minimized)
							.join("\n"),
						CopyType::TableRow(2) => selected_key
							.get_user_info(self.state.minimized)
							.join("\n"),
						CopyType::TableRow(_) => String::new(),
						CopyType::Key => {
							str::from_utf8(&self.gpgme.get_exported_keys(
								match self.command {
									Command::ListKeys(key_type) => key_type,
									_ => KeyType::Public,
								},
								Some(vec![selected_key.get_id()]),
							)?)?
							.to_string()
						}
						CopyType::KeyId => selected_key.get_id(),
						CopyType::KeyFingerprint => {
							selected_key.get_fingerprint()
						}
						CopyType::KeyUserId => selected_key.get_user_id(),
					})
					.expect("failed to set clipboard contents");
				self.prompt.set_output((
					OutputType::Success,
					format!("{} copied to clipboard", copy_type),
				));
				self.mode = Mode::Normal;
			}
			Command::Paste => {
				self.prompt.text = format!(
					":{}",
					self.clipboard
						.get_contents()
						.expect("failed to get clipboard contents")
				);
			}
			Command::EnableInput => self.prompt.enable_command_input(),
			Command::Search(query) => {
				self.prompt.text = format!("/{}", query.unwrap_or_default());
				self.keys_table.items = self.keys_table.default_items.clone();
			}
			Command::Next => self.keys_table.next(),
			Command::Previous => self.keys_table.previous(),
			Command::Minimize | Command::Maximize => {
				self.state.minimize_threshold = 0;
				self.state.minimized = command == Command::Minimize;
			}
			Command::Refresh => self.refresh()?,
			Command::Quit => self.state.running = false,
			Command::None => {}
		}
		Ok(())
	}

	/// Renders all the widgets thus the user interface.
	pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
		let rect = frame.size();
		if self.state.minimize_threshold != 0 {
			self.state.minimized = rect.width < self.state.minimize_threshold;
		}
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[Constraint::Min(rect.height - 1), Constraint::Min(1)].as_ref(),
			)
			.split(rect);
		self.render_command_prompt(frame, chunks[1]);
		if let Command::ListKeys(_) = self.command {
			self.render_keys_table(frame, chunks[0])
		}
	}

	/// Renders the command prompt. (widget)
	fn render_command_prompt<B: Backend>(
		&mut self,
		frame: &mut Frame<'_, B>,
		rect: Rect,
	) {
		frame.render_widget(
			Paragraph::new(Span::raw(if !self.prompt.text.is_empty() {
				self.prompt.text.clone()
			} else {
				match self.command {
					Command::ListKeys(_) => {
						if !self.keys_table.items.is_empty() {
							format!(
								"{} ({}/{})",
								self.command.to_string(),
								self.keys_table
									.state
									.selected()
									.unwrap_or_default() + 1,
								self.keys_table.items.len()
							)
						} else {
							self.command.to_string()
						}
					}
					_ => self.command.to_string(),
				}
			}))
			.style(match self.prompt.output_type {
				OutputType::Success => Style::default()
					.fg(Color::LightGreen)
					.add_modifier(Modifier::BOLD),
				OutputType::Warning => Style::default()
					.fg(Color::LightYellow)
					.add_modifier(Modifier::BOLD),
				OutputType::Failure => Style::default()
					.fg(Color::LightRed)
					.add_modifier(Modifier::BOLD),
				OutputType::Action => {
					Style::default().add_modifier(Modifier::BOLD)
				}
				_ => Style::default(),
			})
			.alignment(if !self.prompt.text.is_empty() {
				Alignment::Left
			} else {
				Alignment::Right
			})
			.wrap(Wrap { trim: false }),
			rect,
		);
		if self.prompt.is_enabled() {
			frame.set_cursor(
				rect.x + self.prompt.text.width() as u16,
				rect.y + 1,
			);
		}
	}

	/// Renders the table of keys. (widget)
	fn render_keys_table<B: Backend>(
		&mut self,
		frame: &mut Frame<'_, B>,
		rect: Rect,
	) {
		frame.render_stateful_widget(
			Table::new(
				self.get_keys_table_rows(
					rect.width
						.checked_sub(
							if self.state.minimized {
								KEYS_ROW_LENGTH.0
							} else {
								KEYS_ROW_LENGTH.1
							} + 7,
						)
						.unwrap_or(rect.width),
					rect.height.checked_sub(2).unwrap_or(rect.height),
				),
			)
			.style(Style::default().fg(Color::Gray))
			.highlight_style(
				Style::default()
					.fg(Color::Reset)
					.add_modifier(Modifier::BOLD),
			)
			.highlight_symbol("> ")
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_style(Style::default().fg(Color::DarkGray)),
			)
			.widths(&[
				Constraint::Min(if self.state.minimized {
					KEYS_ROW_LENGTH.0
				} else {
					KEYS_ROW_LENGTH.1
				}),
				Constraint::Percentage(100),
			])
			.column_spacing(1),
			rect,
			&mut self.keys_table.state,
		);
	}

	/// Returns the rows for keys table.
	fn get_keys_table_rows(
		&mut self,
		max_width: u16,
		max_height: u16,
	) -> Vec<Row<'a>> {
		let mut rows = Vec::new();
		self.keys_table.items = self
			.keys_table
			.items
			.clone()
			.into_iter()
			.filter(|key| {
				let subkey_info = key.get_subkey_info(self.state.minimized);
				let user_info = key.get_user_info(self.state.minimized);
				if self.prompt.is_search_enabled() {
					let search_term =
						self.prompt.text.replacen("/", "", 1).to_lowercase();
					if !subkey_info
						.join("\n")
						.to_lowercase()
						.contains(&search_term) && !user_info
						.join("\n")
						.to_lowercase()
						.contains(&search_term)
					{
						return false;
					}
				}
				let keys_row = RowItem::new(
					subkey_info,
					None,
					max_height,
					self.keys_table.scroll,
				);
				let users_row = RowItem::new(
					user_info,
					Some(max_width),
					max_height,
					self.keys_table.scroll,
				);
				rows.push(
					Row::new(vec![
						Text::from(keys_row.data.join("\n")),
						Text::from(users_row.data.join("\n")),
					])
					.height(
						cmp::max(keys_row.data.len(), users_row.data.len())
							.try_into()
							.unwrap_or(1),
					)
					.bottom_margin(self.keys_table_margin)
					.style(Style::default()),
				);
				true
			})
			.collect();
		rows
	}
}
