use crate::gpg::context::GpgContext;
use crate::gpg::key::GpgKey;
use crate::widget::list::StatefulTable;
use anyhow::Result;
use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::style::Style;
use tui::terminal::Frame;
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
			Table::new(
				["Key", "User"].iter(),
				self.key_list.items.iter().map(|key| {
					Row::Data(
						vec![key.get_id(), key.get_primary_user_id()]
							.into_iter(),
					)
				}),
			)
			.block(Block::default().title("Table").borders(Borders::ALL))
			.style(Style::default())
			.header_style(Style::default())
			.highlight_style(Style::default())
			.highlight_symbol(">>")
			.widths(&[Constraint::Percentage(10), Constraint::Percentage(50)])
			.column_spacing(1),
			rect,
			&mut self.key_list.state,
		);
	}
}
