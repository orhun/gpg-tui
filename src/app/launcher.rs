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

/// Main application.
///
/// It operates the TUI via rendering the widgets
/// and updating the application state.
pub struct App {
	/// Is app running?
	pub running: bool,
	/// List of public keys.
	pub key_list: StatefulTable<GpgKey>,
}

impl App {
	/// Constructs a new instance of `App`.
	pub fn new() -> Result<Self> {
		Ok(Self {
			running: true,
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
		frame.render_stateful_widget(
			Table::new(self.key_list.items.iter().map(|key| {
				let keys_row = self.get_keys_row(key);
				let keys_row_height = keys_row.lines().count();
				let users_row = self.get_users_row(key);
				let users_row_height = users_row.lines().count();
				Row::new(vec![Text::from(keys_row), Text::from(users_row)])
					.height(cmp::max(
						keys_row_height.try_into().unwrap_or(1),
						users_row_height.try_into().unwrap_or(1),
					))
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
			.widths(&[Constraint::Min(55), Constraint::Percentage(100)])
			.column_spacing(1),
			rect,
			&mut self.key_list.state,
		);
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
					key.fingerprint().unwrap_or("[?]"),
					if i != subkeys.len() - 1 { "|" } else { " " },
					GpgKey::get_time(*key),
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
					user.id().unwrap_or("[?]"),
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
					} else {
						format!(
							"{} {}",
							sig.signer_key_id().unwrap_or("[?]"),
							sig.signer_user_id().unwrap_or("[?]"),
						)
					},
					if let Some(date) = sig.creation_time() {
						DateTime::<Utc>::from(date).format("%F").to_string()
					} else {
						String::from("[?]")
					}
				)
			})
	}
}
