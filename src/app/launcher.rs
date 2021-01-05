use crate::gpg::context::GpgContext;
use crate::gpg::key::GpgKey;
use crate::gpg::subkey;
use crate::widget::list::StatefulTable;
use anyhow::Result;
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
				let first_row = self.get_first_row(key);
				let first_row_height = first_row.lines().count();
				let second_row = self.get_second_row(key);
				let second_row_height = second_row.lines().count();
				Row::new(vec![Text::from(first_row), Text::from(second_row)])
					.height(cmp::max(
						first_row_height.try_into().unwrap_or(1),
						second_row_height.try_into().unwrap_or(1),
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

	/// Returns the first row of the table.
	fn get_first_row(&self, key: &GpgKey) -> String {
		let subkeys = key.get_subkeys();
		format!(
			"[{}] {}/{}\n{}      └─{}\n{}",
			key.get_flags(),
			key.get_algorithm(),
			key.get_fingerprint(),
			if !subkeys.is_empty() { "|" } else { " " },
			key.get_time(),
			subkeys
				.iter()
				.enumerate()
				.fold(String::new(), |acc, (i, key)| {
					let time = subkey::get_time(*key);
					format!(
						"{}[{}] {}/{}\n{}      └─{}\n",
						acc,
						subkey::get_flags(*key),
						key.algorithm_name()
							.unwrap_or_else(|_| { String::from("[?]") }),
						key.fingerprint_raw()
							.map_or(String::from("[?]"), |v| v
								.to_string_lossy()
								.into_owned()),
						if i != subkeys.len() - 1 { "|" } else { " " },
						time
					)
				})
		)
	}

	/// Returns the second row of the table.
	fn get_second_row(&self, key: &GpgKey) -> String {
		let user_ids = key.get_user_ids();
		user_ids
			.iter()
			.enumerate()
			.fold(String::new(), |acc, (i, user)| {
				let val = format!(
					"[{}] {}",
					user.validity(),
					user.id_raw().map_or(String::from("[?]"), |v| v
						.to_string_lossy()
						.into_owned())
				);
				if i == 0 {
					format!("{}\n", val)
				} else if i == user_ids.len() - 1 {
					format!("{} └─{}\n", acc, val)
				} else {
					format!("{} ├─{}\n", acc, val)
				}
			})
	}
}
