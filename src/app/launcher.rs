use crate::app::clipboard::CopyType;
use crate::app::command::Command;
use crate::app::mode::Mode;
use crate::app::prompt::{OutputType, Prompt};
use crate::app::state::State;
use crate::app::style;
use crate::app::tab::Tab;
use crate::args::Args;
use crate::gpg::context::GpgContext;
use crate::gpg::key::{GpgKey, KeyDetail, KeyType};
use crate::widget::list::StatefulList;
use crate::widget::row::{RowItem, ScrollDirection};
use crate::widget::table::{StatefulTable, TableState};
use anyhow::Result;
use copypasta_ext::prelude::ClipboardProvider;
use copypasta_ext::x11_fork::ClipboardContext;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryInto;
use std::str;
use std::str::FromStr;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans, Text};
use tui::widgets::{
	Block, Borders, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};
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
	/// Current tab.
	pub tab: Tab,
	/// Content of the options menu.
	pub options: StatefulList<Command>,
	/// Public/secret keys.
	pub keys: HashMap<KeyType, Vec<GpgKey>>,
	/// Table of public/secret keys.
	pub keys_table: StatefulTable<GpgKey>,
	/// States of the keys table.
	pub keys_table_states: HashMap<KeyType, TableState>,
	/// Level of detail to show for keys table.
	pub keys_table_detail: KeyDetail,
	/// Bottom margin value of the keys table.
	pub keys_table_margin: u16,
	/// Clipboard context.
	pub clipboard: ClipboardContext,
	/// GPGME context.
	pub gpgme: &'a mut GpgContext,
	/// Parsed command-line arguments.
	args: &'a Args,
}

impl<'a> App<'a> {
	/// Constructs a new instance of `App`.
	pub fn new(gpgme: &'a mut GpgContext, args: &'a Args) -> Result<Self> {
		let keys = gpgme.get_all_keys()?;
		Ok(Self {
			state: State::default(),
			mode: Mode::Normal,
			prompt: Prompt::default(),
			tab: Tab::Keys(KeyType::Public),
			options: StatefulList::with_items(Vec::new()),
			keys_table: StatefulTable::with_items(
				keys.get(&KeyType::Public)
					.expect("failed to get public keys")
					.to_vec(),
			),
			keys,
			keys_table_states: HashMap::new(),
			keys_table_detail: KeyDetail::Minimum,
			keys_table_margin: 1,
			clipboard: ClipboardContext::new()
				.expect("failed to initialize clipboard"),
			gpgme,
			args,
		})
	}

	/// Resets the application state.
	pub fn refresh(&mut self) -> Result<()> {
		self.state = State::default();
		self.mode = Mode::Normal;
		self.prompt = Prompt::default();
		self.options.state.select(Some(0));
		self.keys = self.gpgme.get_all_keys()?;
		self.keys_table_states.clear();
		self.keys_table_detail = KeyDetail::Minimum;
		self.keys_table_margin = 1;
		match self.tab {
			Tab::Keys(key_type) => {
				self.keys_table = StatefulTable::with_items(
					self.keys
						.get(&key_type)
						.unwrap_or_else(|| {
							panic!("failed to get {} keys", key_type)
						})
						.to_vec(),
				)
			}
		};
		Ok(())
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
		let mut show_options = false;
		match command {
			Command::ShowOutput(output_type, message) => {
				self.prompt.set_output((output_type, message))
			}
			Command::ShowOptions => {
				let prev_selection = self.options.state.selected();
				let prev_item_count = self.options.items.len();
				self.options = StatefulList::with_items(match self.tab {
					Tab::Keys(key_type) => vec![
						Command::None,
						Command::Refresh,
						Command::ExportKeys(
							key_type,
							vec![self.keys_table.items[self
								.keys_table
								.state
								.tui
								.selected()
								.expect("invalid selection")]
							.get_id()],
						),
						Command::ExportKeys(key_type, Vec::new()),
						Command::Copy(CopyType::Key),
						Command::Copy(CopyType::KeyId),
						Command::Copy(CopyType::KeyFingerprint),
						Command::Copy(CopyType::KeyUserId),
						Command::Copy(CopyType::TableRow(1)),
						Command::Copy(CopyType::TableRow(2)),
						Command::Paste,
						Command::ToggleDetail(false),
						Command::ToggleDetail(true),
						Command::Set(
							String::from("detail"),
							String::from("minimum"),
						),
						Command::Set(
							String::from("detail"),
							String::from("standard"),
						),
						Command::Set(
							String::from("detail"),
							String::from("full"),
						),
						Command::Set(
							String::from("armor"),
							(!self.gpgme.config.armor).to_string(),
						),
						Command::Set(
							String::from("margin"),
							String::from(if self.keys_table_margin == 1 {
								"0"
							} else {
								"1"
							}),
						),
						if self.state.minimized {
							Command::Maximize
						} else {
							Command::Minimize
						},
						if self.mode == Mode::Visual {
							Command::SwitchMode(Mode::Normal)
						} else {
							Command::SwitchMode(Mode::Visual)
						},
					],
				});
				if prev_item_count == 0
					|| self.options.items.len() == prev_item_count
				{
					self.options.state.select(prev_selection.or(Some(0)));
				}
				show_options = true;
			}
			Command::ListKeys(key_type) => {
				let previous_key_type = match key_type {
					KeyType::Public => KeyType::Secret,
					KeyType::Secret => KeyType::Public,
				};
				self.keys_table_states
					.insert(previous_key_type, self.keys_table.state.clone());
				self.keys.insert(
					previous_key_type,
					self.keys_table.default_items.clone(),
				);
				self.keys_table = StatefulTable::with_items(
					self.keys
						.get(&key_type)
						.unwrap_or_else(|| {
							panic!("failed to get {} keys", key_type)
						})
						.to_vec(),
				);
				if let Some(state) = self.keys_table_states.get(&key_type) {
					self.keys_table.state = state.clone();
				}
				self.tab = Tab::Keys(key_type);
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
				if let Some(index) = self.keys_table.state.tui.selected() {
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
			Command::Scroll(direction, false) => match direction {
				ScrollDirection::Down(_) => {
					if self.state.show_options {
						self.options.next();
						show_options = true;
					} else {
						self.keys_table.next();
					}
				}
				ScrollDirection::Up(_) => {
					if self.state.show_options {
						self.options.previous();
						show_options = true;
					} else {
						self.keys_table.previous();
					}
				}
				ScrollDirection::Top => {
					self.keys_table.state.tui.select(Some(0));
				}
				ScrollDirection::Bottom => {
					self.keys_table.state.tui.select(Some(
						self.keys_table
							.items
							.len()
							.checked_sub(1)
							.unwrap_or_default(),
					));
				}
				_ => {}
			},
			Command::Scroll(direction, true) => {
				self.keys_table.scroll_row(direction);
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
						if let Some(index) =
							self.keys_table.state.tui.selected()
						{
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
						if let Some(index) =
							self.keys_table.state.tui.selected()
						{
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
				if !(mode == Mode::Copy && self.keys_table.items.is_empty()) {
					self.mode = mode;
					self.prompt
						.set_output((OutputType::Action, mode.to_string()))
				}
			}
			Command::Copy(copy_type) => {
				let selected_key = &self.keys_table.items[self
					.keys_table
					.state
					.tui
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
								match self.tab {
									Tab::Keys(key_type) => key_type,
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
				self.prompt.enable_search();
				self.keys_table.items = self.keys_table.default_items.clone();
			}
			Command::NextTab => {
				self.tab.next();
				self.run_command(self.tab.get_command())?
			}
			Command::PreviousTab => {
				self.tab.previous();
				self.run_command(self.tab.get_command())?
			}
			Command::Minimize | Command::Maximize => {
				self.state.minimize_threshold = 0;
				self.state.minimized = command == Command::Minimize;
			}
			Command::Refresh => self.refresh()?,
			Command::Quit => self.state.running = false,
			Command::None => {}
		}
		self.state.show_options = show_options;
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
		match self.tab {
			Tab::Keys(_) => self.render_keys_table(frame, chunks[0]),
		}
		if self.state.show_options {
			self.render_options_menu(frame, rect);
		}
	}

	/// Renders the command prompt. (widget)
	fn render_command_prompt<B: Backend>(
		&mut self,
		frame: &mut Frame<'_, B>,
		rect: Rect,
	) {
		frame.render_widget(
			Paragraph::new(Spans::from(if !self.prompt.text.is_empty() {
				vec![Span::raw(format!(
					"{}{}",
					self.prompt.output_type, self.prompt.text
				))]
			} else {
				match self.tab {
					Tab::Keys(key_type) => {
						let arrow_color = if self.args.style == *"colored" {
							Color::LightBlue
						} else {
							Color::DarkGray
						};
						vec![
							Span::styled(
								"< ",
								Style::default().fg(arrow_color),
							),
							Span::raw(format!(
								"list {}{}",
								key_type,
								if !self.keys_table.items.is_empty() {
									format!(
										" ({}/{})",
										self.keys_table
											.state
											.tui
											.selected()
											.unwrap_or_default() + 1,
										self.keys_table.items.len()
									)
								} else {
									String::new()
								}
							)),
							Span::styled(
								" >",
								Style::default().fg(arrow_color),
							),
						]
					}
				}
			}))
			.style(if self.args.style == *"colored" {
				match self.prompt.output_type {
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
						if self.args.style == *"colored" {
							Style::default()
								.fg(Color::LightBlue)
								.add_modifier(Modifier::BOLD)
						} else {
							Style::default().add_modifier(Modifier::BOLD)
						}
					}
					OutputType::None => Style::default(),
				}
			} else if self.prompt.output_type != OutputType::None {
				Style::default().add_modifier(Modifier::BOLD)
			} else {
				Style::default()
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

	/// Renders the options menu. (widget)
	fn render_options_menu<B: Backend>(
		&mut self,
		frame: &mut Frame<'_, B>,
		rect: Rect,
	) {
		let (length_x, percent_y) = (38, 50);
		let popup_layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[
					Constraint::Percentage((100 - percent_y) / 2),
					Constraint::Percentage(percent_y),
					Constraint::Percentage((100 - percent_y) / 2),
				]
				.as_ref(),
			)
			.split(rect);
		let area = Layout::default()
			.direction(Direction::Horizontal)
			.constraints(
				[
					Constraint::Length(
						(popup_layout[1].width.checked_sub(length_x))
							.unwrap_or_default() / 2,
					),
					Constraint::Min(length_x),
					Constraint::Length(
						(popup_layout[1].width.checked_sub(length_x))
							.unwrap_or_default() / 2,
					),
				]
				.as_ref(),
			)
			.split(popup_layout[1])[1];
		frame.render_widget(Clear, area);
		frame.render_stateful_widget(
			List::new(
				self.options
					.items
					.iter()
					.map(|v| ListItem::new(Span::raw(v.to_string())))
					.collect::<Vec<ListItem>>(),
			)
			.block(
				Block::default()
					.title("Options")
					.style(if self.args.style == *"colored" {
						Style::default().fg(Color::LightBlue)
					} else {
						Style::default()
					})
					.borders(Borders::ALL),
			)
			.style(Style::default().fg(Color::Gray))
			.highlight_style(
				Style::default()
					.fg(Color::Reset)
					.add_modifier(Modifier::BOLD),
			)
			.highlight_symbol("> "),
			area,
			&mut self.options.state,
		);
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
			.highlight_style(if self.args.style == *"colored" {
				Style::default().add_modifier(Modifier::BOLD)
			} else {
				Style::default()
					.fg(Color::Reset)
					.add_modifier(Modifier::BOLD)
			})
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
			&mut self.keys_table.state.tui,
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
			.enumerate()
			.filter(|(i, key)| {
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
					self.keys_table.state.scroll,
				);
				let users_row = RowItem::new(
					user_info,
					Some(max_width),
					max_height,
					self.keys_table.state.scroll,
				);
				rows.push(
					Row::new(if self.args.style == *"colored" {
						let highlighted =
							self.keys_table.state.tui.selected() == Some(*i);
						vec![
							style::get_colored_table_row(
								&keys_row.data,
								highlighted,
							),
							style::get_colored_table_row(
								&users_row.data,
								highlighted,
							),
						]
					} else {
						vec![
							Text::from(keys_row.data.join("\n")),
							Text::from(users_row.data.join("\n")),
						]
					})
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
			.map(|(_, v)| v)
			.collect();
		rows
	}
}
