/// Application states (flags) for handling interface and events.
#[derive(Clone, Copy, Debug)]
pub struct AppState {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
		}
	}
}
