use crate::gpg::context::GpgContext;
use crate::gpg::key::GpgKey;
use crate::widget::list::StatefulTable;
use anyhow::Result;
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
				let user_ids = key.get_user_ids();
				Row::new(vec![
					Text::from(key.get_fingerprint()),
					Text::from(format!(
						"{}\n{}",
						user_ids
							.first()
							.cloned()
							.unwrap_or_else(|| String::from("[?]")),
						user_ids
							.iter()
							.skip(1)
							.fold(String::new(), |acc, x| acc + x + "\n")
					)),
				])
				.height(user_ids.len().try_into().unwrap_or(1))
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
			.widths(&[Constraint::Min(40), Constraint::Percentage(100)])
			.column_spacing(1),
			rect,
			&mut self.key_list.state,
		);
	}
}
