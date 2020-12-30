use crate::gpg::context::{GpgContext, GpgKey};
use crate::widget::list::StatefulList;
use anyhow::Result;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};

/// Main application.
///
/// It operates the TUI via rendering the widgets
/// and updating the application state.
pub struct App {
	/// Is app running?
	pub running: bool,
	/// List of public keys.
	pub key_list: StatefulList<GpgKey>,
}

impl App {
	/// Constructs a new instance of `App`.
	pub fn new() -> Result<Self> {
		Ok(Self {
			running: true,
			key_list: StatefulList::with_items(GpgContext::new()?.get_keys()?),
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
			List::new(
				self.key_list
					.items
					.iter()
					.map(|i| {
						ListItem::new(vec![Spans::from(Span::raw(
							(*i).id().unwrap_or("?"),
						))])
					})
					.collect::<Vec<ListItem>>(),
			)
			.block(Block::default().title("List").borders(Borders::ALL))
			.style(Style::default())
			.highlight_style(Style::default())
			.highlight_symbol(">>"),
			rect,
			&mut self.key_list.state,
		);
	}
}
