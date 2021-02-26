/// Application states (flags) for managing the launcher.
#[derive(Clone, Debug)]
pub struct State {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
	/// Threshold value (width) for minimizing.
	pub minimize_threshold: u16,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
			minimize_threshold: 90,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_state() {
		let state = State::default();
		assert_eq!(true, state.running);
		assert_eq!(false, state.minimized);
		assert_eq!(90, state.minimize_threshold);
	}
}
