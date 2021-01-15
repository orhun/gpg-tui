use crate::widget::row::{ScrollAmount, ScrollDirection};
use tui::widgets::TableState as State;

/// Table widget with TUI controlled states.
#[derive(Clone, Debug)]
pub struct StatefulTable<T> {
	/// Table items (states).
	pub items: Vec<T>,
	/// State that can be modified by TUI.
	pub state: State,
	/// Scroll amount of the table.
	pub scroll: ScrollAmount,
}

impl<T> StatefulTable<T> {
	/// Constructs a new instance of `StatefulTable`.
	pub fn new(items: Vec<T>, mut state: State) -> StatefulTable<T> {
		state.select(Some(0));
		Self {
			items,
			state,
			scroll: ScrollAmount::default(),
		}
	}

	/// Construct a new `StatefulTable` with given items.
	pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
		Self::new(items, State::default())
	}

	/// Select the next item.
	pub fn next(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i >= self.items.len() - 1 {
					0
				} else {
					i + 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
		self.reset_scroll();
	}

	/// Select the previous item.
	pub fn previous(&mut self) {
		let i = match self.state.selected() {
			Some(i) => {
				if i == 0 {
					self.items.len() - 1
				} else {
					i - 1
				}
			}
			None => 0,
		};
		self.state.select(Some(i));
		self.reset_scroll();
	}

	/// Sets the scrolling state of the table
	/// depending on the given direction and offset.
	pub fn scroll(&mut self, direction: ScrollDirection) {
		match direction {
			ScrollDirection::Up(value) => {
				self.scroll.vertical =
					self.scroll.vertical.checked_sub(value).unwrap_or_default();
			}
			ScrollDirection::Right(value) => {
				self.scroll.horizontal = self
					.scroll
					.horizontal
					.checked_add(value)
					.unwrap_or(self.scroll.horizontal)
			}
			ScrollDirection::Down(value) => {
				self.scroll.vertical = self
					.scroll
					.vertical
					.checked_add(value)
					.unwrap_or(self.scroll.vertical)
			}
			ScrollDirection::Left(value) => {
				self.scroll.horizontal = self
					.scroll
					.horizontal
					.checked_sub(value)
					.unwrap_or_default();
			}
		}
	}

	/// Resets the scroll state.
	pub fn reset_scroll(&mut self) {
		self.scroll = ScrollAmount::default();
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
		table.state.select(Some(1));
		assert_eq!(Some(1), table.state.selected());
		table.next();
		assert_eq!(Some(2), table.state.selected());
		table.previous();
		assert_eq!(Some(1), table.state.selected());
		table.reset_scroll();
		assert_eq!(
			"ScrollAmount { vertical: 0, horizontal: 0 }",
			&format!("{:?}", table.scroll)
		);
		table.scroll(ScrollDirection::Down(3));
		table.scroll(ScrollDirection::Right(2));
		assert_eq!(
			"ScrollAmount { vertical: 3, horizontal: 2 }",
			&format!("{:?}", table.scroll)
		);
		table.scroll(ScrollDirection::Up(1));
		table.scroll(ScrollDirection::Left(1));
		assert_eq!(
			"ScrollAmount { vertical: 2, horizontal: 1 }",
			&format!("{:?}", table.scroll)
		);
	}
}
