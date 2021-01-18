use crate::gpg::key::KeyDetailLevel;

/// Application states for handling interface and events.
#[derive(Clone, Debug)]
pub struct AppState {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
	/// Command input.
	pub input: String,
	/// Level of detail to show for keys table.
	pub table_detail: KeyDetailLevel,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
			input: String::new(),
			table_detail: KeyDetailLevel::Minimum,
		}
	}
}
