use crate::app::command::Command;
use crate::app::prompt::Prompt;
use crate::app::state::AppState;
use crate::gpg::context::GpgContext;
use crate::gpg::key::{GpgKey, KeyType};
use crate::widget::row::RowItem;
use crate::widget::table::StatefulTable;
use anyhow::Result;
use std::cmp;
use std::convert::TryInto;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Text};
use tui::widgets::{Paragraph, Row, Table, Wrap};
use unicode_width::UnicodeWidthStr;

/// Threshold value (width) for minimizing.
const MINIMIZE_THRESHOLD: u16 = 90;
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
	pub state: AppState,
	/// Application prompt.
	pub prompt: Prompt,
	/// Application command.
	pub command: Command,
	/// List of public keys.
	pub key_list: StatefulTable<GpgKey>,
	/// GPGME context.
	gpgme: &'a mut GpgContext,
}

impl<'a> App<'a> {
	/// Constructs a new instance of `App`.
	pub fn new(gpgme: &'a mut GpgContext) -> Result<Self> {
		Ok(Self {
			state: AppState::default(),
			prompt: Prompt::default(),
			command: Command::default(),
			key_list: StatefulTable::with_items(
				gpgme.get_keys(KeyType::Public, None)?,
			),
			gpgme,
		})
	}

	/// Resets the application state.
	pub fn refresh(&mut self) -> Result<()> {
		self.state = AppState::default();
		self.prompt = Prompt::default();
		self.run_command(Command::default())
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
			Command::ListKeys(key_type) => {
				self.key_list = StatefulTable::with_items(
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
						Ok(path) => format!("Export: {}", path),
						Err(e) => format!("Error: {}", e),
					},
				);
			}
			Command::Quit => self.state.running = false,
		}
		Ok(())
	}

	/// Renders all the widgets thus the user interface.
	pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
		let rect = frame.size();
		self.state.minimized = rect.width < MINIMIZE_THRESHOLD;
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
						if !self.key_list.items.is_empty() {
							format!(
								"{} ({}/{})",
								self.command.to_string(),
								self.key_list
									.state
									.selected()
									.unwrap_or_default() + 1,
								self.key_list.items.len()
							)
						} else {
							self.command.to_string()
						}
					}
					_ => self.command.to_string(),
				}
			}))
			.style(Style::default())
			.alignment(if !self.prompt.text.is_empty() {
				Alignment::Left
			} else {
				Alignment::Right
			})
			.wrap(Wrap { trim: false }),
			rect,
		);
		if self.prompt.is_input_enabled() {
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
							} + 3,
						)
						.unwrap_or(rect.width),
					rect.height,
				),
			)
			.style(Style::default())
			.highlight_style(Style::default().add_modifier(Modifier::BOLD))
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
			&mut self.key_list.state,
		);
	}

	/// Returns the rows for keys table.
	fn get_keys_table_rows(
		&mut self,
		max_width: u16,
		max_height: u16,
	) -> Vec<Row<'a>> {
		let mut rows = Vec::new();
		self.key_list.items = self
			.key_list
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
					self.key_list.scroll,
				);
				let users_row = RowItem::new(
					user_info,
					Some(max_width),
					max_height,
					self.key_list.scroll,
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
					.bottom_margin(1)
					.style(Style::default()),
				);
				true
			})
			.collect();
		rows
	}
}
