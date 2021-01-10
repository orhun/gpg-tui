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
	pub fn new(items: Vec<T>, state: State) -> StatefulTable<T> {
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
