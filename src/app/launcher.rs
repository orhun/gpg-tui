use crate::gpg::context::GpgContext;
use crate::gpg::key::GpgKey;
use crate::widget::table::StatefulTable;
use anyhow::Result;
use chrono::{DateTime, Utc};
use gpgme::{UserId, UserIdSignature};
use std::cmp;
use std::convert::TryInto;
use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Row, Table};

/// Threshold value (width) for minimizing the table.
const TABLE_MIN_THRESHOLD: u16 = 100;
/// Length of keys row in maximized mode.
const KEYS_ROW_MAX_LENGTH: u16 = 55;
/// Length of keys row in minimized mode.
const KEYS_ROW_MIN_LENGTH: u16 = 31;

/// Main application.
///
/// It operates the TUI via rendering the widgets
/// and updating the application state.
pub struct App {
	/// Is app running?
	pub running: bool,
	/// Is table minimized?
	pub table_minimized: bool,
	/// List of public keys.
	pub key_list: StatefulTable<GpgKey>,
}

impl App {
	/// Constructs a new instance of `App`.
	pub fn new() -> Result<Self> {
		Ok(Self {
			running: true,
			table_minimized: false,
			key_list: StatefulTable::with_items(GpgContext::new()?.get_keys()?),
		})
	}

	/// Reset the application state.
	pub fn refresh(&mut self) {
		self.key_list.state.select(Some(0))
	}

	/// Renders the user interface.
	pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
		self.render_key_list(frame, frame.size())
	}

	/// Renders the list of public keys.
	fn render_key_list<B: Backend>(
		&mut self,
		frame: &mut Frame<'_, B>,
		rect: Rect,
	) {
		self.table_minimized = rect.width < TABLE_MIN_THRESHOLD;
		frame.render_stateful_widget(
			Table::new(self.key_list.items.iter().map(|key| {
				let max_row_height =
					rect.height.checked_sub(4).unwrap_or(rect.height);
				let keys_row = self
					.adjust_row_height(self.get_keys_row(key), max_row_height);
				let users_row = self
					.adjust_row_height(self.get_users_row(key), max_row_height);
				let row_height = cmp::max(
					keys_row.lines().count().try_into().unwrap_or(1),
					users_row.lines().count().try_into().unwrap_or(1),
				);
				Row::new(vec![Text::from(keys_row), Text::from(users_row)])
					.height(row_height)
					.bottom_margin(1)
					.style(Style::default())
			}))
			.header(
				Row::new(vec!["Key", "User"])
					.style(Style::default())
					.bottom_margin(1),
			)
			.block(Block::default().title("Table").borders(Borders::ALL))
			.style(Style::default())
			.highlight_style(Style::default().add_modifier(Modifier::BOLD))
			.widths(&[
				Constraint::Min(if self.table_minimized {
					KEYS_ROW_MIN_LENGTH
				} else {
					KEYS_ROW_MAX_LENGTH
				}),
				Constraint::Percentage(100),
			])
			.column_spacing(1),
			rect,
			&mut self.key_list.state,
		);
	}

	/// Limits the row height to the maximum height.
	fn adjust_row_height(&self, row: String, max_height: u16) -> String {
		if row.lines().count() > max_height.into() {
			row.lines()
				.collect::<Vec<&str>>()
				.drain(
					0..(max_height.checked_sub(1).unwrap_or(max_height)).into(),
				)
				.collect::<Vec<&str>>()
				.join("\n") + "\n ..."
		} else {
			row
		}
	}

	/// Returns information about keys for the first row of the table.
	fn get_keys_row(&self, key: &GpgKey) -> String {
		let subkeys = key.get_subkeys();
		subkeys
			.iter()
			.enumerate()
			.fold(String::new(), |acc, (i, key)| {
				format!(
					"{}[{}] {}/{}\n{}      └─{}\n",
					acc,
					GpgKey::get_flags(*key),
					key.algorithm_name()
						.unwrap_or_else(|_| { String::from("[?]") }),
					if self.table_minimized {
						key.id()
					} else {
						key.fingerprint()
					}
					.unwrap_or("[?]"),
					if i != subkeys.len() - 1 { "|" } else { " " },
					GpgKey::get_time(
						*key,
						if self.table_minimized { "%Y" } else { "%F" }
					),
				)
			})
	}

	/// Returns information about users for the second row of the table.
	fn get_users_row(&self, key: &GpgKey) -> String {
		let user_ids = key.get_user_ids();
		user_ids
			.iter()
			.enumerate()
			.fold(String::new(), |acc, (i, user)| {
				format!(
					"{}{}[{}] {}\n{}",
					acc,
					if i == 0 {
						""
					} else if i == user_ids.len() - 1 {
						" └─"
					} else {
						" ├─"
					},
					user.validity(),
					if self.table_minimized {
						user.email()
					} else {
						user.id()
					}
					.unwrap_or("[?]"),
					self.get_user_signatures(key, user, user_ids.len(), i)
				)
			})
	}

	/// Returns the signature information of an user.
	fn get_user_signatures(
		&self,
		key: &GpgKey,
		user: &UserId,
		user_count: usize,
		user_index: usize,
	) -> String {
		let signatures = user.signatures().collect::<Vec<UserIdSignature>>();
		signatures
			.iter()
			.enumerate()
			.fold(String::new(), |acc, (i, sig)| {
				format!(
					"{} {}  {}[{:x}] {} ({})\n",
					acc,
					if user_count == 1 {
						" "
					} else if user_index == user_count - 1 {
						"    "
					} else if user_index == 0 {
						"│"
					} else {
						"│   "
					},
					if i == signatures.len() - 1 {
						"└─"
					} else {
						"├─"
					},
					sig.cert_class(),
					if sig.signer_key_id() == key.get_id() {
						String::from("selfsig")
					} else if self.table_minimized {
						sig.signer_key_id().unwrap_or("[?]").to_string()
					} else {
						format!(
							"{} {}",
							sig.signer_key_id().unwrap_or("[?]"),
							sig.signer_user_id().unwrap_or("[?]")
						)
					},
					if let Some(date) = sig.creation_time() {
						DateTime::<Utc>::from(date)
							.format(if self.table_minimized {
								"%Y"
							} else {
								"%F"
							})
							.to_string()
					} else {
						String::from("[?]")
					}
				)
			})
	}
}
