use crate::gpg::key::KeyDetailLevel;

/// Application states for handling interface and events.
#[derive(Clone, Debug)]
pub struct AppState {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
	/// Is visual mode enabled?
	pub visual_mode: bool,
	/// Level of detail to show for keys table.
	pub table_detail: KeyDetailLevel,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
			visual_mode: false,
			table_detail: KeyDetailLevel::Minimum,
		}
	}
}
