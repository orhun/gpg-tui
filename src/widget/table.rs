use crate::widget::row::{ScrollAmount, ScrollDirection};
use tui::widgets::TableState as TuiState;

/// State of the table widget.
#[derive(Clone, Debug, Default)]
pub struct TableState {
	/// State that can be modified by TUI.
	pub tui: TuiState,
	/// Scroll amount of the table.
	pub scroll: ScrollAmount,
}

/// Table widget with TUI controlled states.
#[derive(Clone, Debug)]
pub struct StatefulTable<T: Clone> {
	/// Default table items (for search functionality).
	pub default_items: Vec<T>,
	/// Table items.
	pub items: Vec<T>,
	/// Table state.
	pub state: TableState,
}

impl<T: Clone> StatefulTable<T> {
	/// Constructs a new instance of `StatefulTable`.
	pub fn new(items: Vec<T>, mut state: TableState) -> StatefulTable<T> {
		state.tui.select(Some(0));
		Self {
			default_items: items.clone(),
			items,
			state,
		}
	}

	/// Construct a new `StatefulTable` with given items.
	pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
		Self::new(items, TableState::default())
	}

	/// Returns the selected item.
	pub fn selected(&self) -> Option<&T> {
		self.items.get(self.state.tui.selected()?)
	}

	/// Selects the next item.
	pub fn next(&mut self) {
		let i = match self.state.tui.selected() {
			Some(i) => {
				if i >= self.items.len().checked_sub(1).unwrap_or(i) {
					0
				} else {
					i + 1
				}
			}
			None => 0,
		};
		self.state.tui.select(Some(i));
		self.reset_scroll();
	}

	/// Selects the previous item.
	pub fn previous(&mut self) {
		let i = match self.state.tui.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len().checked_sub(1).unwrap_or(i)
				} else {
					i - 1
				}
			}
			None => 0,
		};
		self.state.tui.select(Some(i));
		self.reset_scroll();
	}

	/// Sets the scrolling state of the table row
	/// depending on the given direction and offset.
	pub fn scroll_row(&mut self, direction: ScrollDirection) {
		match direction {
			ScrollDirection::Up(value) => {
				self.state.scroll.vertical = self
					.state
					.scroll
					.vertical
					.checked_sub(value)
					.unwrap_or_default();
			}
			ScrollDirection::Right(value) => {
				self.state.scroll.horizontal = self
					.state
					.scroll
					.horizontal
					.checked_add(value)
					.unwrap_or(self.state.scroll.horizontal)
			}
			ScrollDirection::Down(value) => {
				self.state.scroll.vertical = self
					.state
					.scroll
					.vertical
					.checked_add(value)
					.unwrap_or(self.state.scroll.vertical)
			}
			ScrollDirection::Left(value) => {
				self.state.scroll.horizontal = self
					.state
					.scroll
					.horizontal
					.checked_sub(value)
					.unwrap_or_default();
			}
			_ => {}
		}
	}

	/// Resets the items state.
	pub fn reset_state(&mut self) {
		self.items = self.default_items.clone();
		self.state.tui.select(Some(0));
	}

	/// Resets the scroll state.
	pub fn reset_scroll(&mut self) {
		self.state.scroll = ScrollAmount::default();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_widget_table() {
		let mut table =
			StatefulTable::with_items(vec!["data1", "data2", "data3"]);
		table.state.tui.select(Some(1));
		assert_eq!(Some(&"data2"), table.selected());
		table.next();
		assert_eq!(Some(2), table.state.tui.selected());
		table.previous();
		assert_eq!(Some(1), table.state.tui.selected());
		table.reset_scroll();
		assert_eq!(
			"ScrollAmount { vertical: 0, horizontal: 0 }",
			&format!("{:?}", table.state.scroll)
		);
		table.scroll_row(ScrollDirection::Down(3));
		table.scroll_row(ScrollDirection::Right(2));
		assert_eq!(
			"ScrollAmount { vertical: 3, horizontal: 2 }",
			&format!("{:?}", table.state.scroll)
		);
		table.scroll_row(ScrollDirection::Up(1));
		table.scroll_row(ScrollDirection::Left(1));
		assert_eq!(
			"ScrollAmount { vertical: 2, horizontal: 1 }",
			&format!("{:?}", table.state.scroll)
		);
		table.reset_state();
		assert_eq!(Some(0), table.state.tui.selected());
		assert_eq!(table.default_items, table.items);
	}
}
