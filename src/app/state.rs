use crate::gpg::key::KeyDetailLevel;

/// Application states (flags) for handling interface and events.
#[derive(Clone, Copy, Debug)]
pub struct AppState {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
	/// Level of detail to show for keys table.
	pub table_detail: KeyDetailLevel,
	/// Level of detail to show for the selected row of the keys table.
	pub selected_row_detail: KeyDetailLevel,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
			table_detail: KeyDetailLevel::Minimum,
			selected_row_detail: KeyDetailLevel::Minimum,
		}
	}
}
