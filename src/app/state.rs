/// Application states (flags) for managing the launcher.
#[derive(Clone, Debug)]
pub struct State {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
		}
	}
}
