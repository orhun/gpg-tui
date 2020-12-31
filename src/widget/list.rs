use tui::widgets::TableState as State;

/// List widget with TUI controlled states.
pub struct StatefulTable<T> {
	/// List items (states).
	pub items: Vec<T>,
	/// State that can be modified by TUI.
	pub state: State,
}

impl<T> StatefulTable<T> {
	/// Constructs a new instance of `StatefulTable`.
	pub fn new(items: Vec<T>, state: State) -> StatefulTable<T> {
		Self { items, state }
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
	}
}
