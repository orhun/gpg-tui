use crate::args::Args;

/// Application states (flags) for managing the launcher.
#[derive(Clone, Debug)]
pub struct State {
	/// Is app running?
	pub running: bool,
	/// Is app minimized?
	pub minimized: bool,
	/// Is app colored?
	pub colored: bool,
	/// Threshold value (width) for minimizing.
	pub minimize_threshold: u16,
	/// Is the options menu (popup) showing?
	pub show_options: bool,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: true,
			minimized: false,
			colored: false,
			minimize_threshold: 90,
			show_options: false,
		}
	}
}

impl<'a> From<&'a Args> for State {
	fn from(args: &'a Args) -> Self {
		State {
			colored: args.style == *"colored",
			..Self::default()
		}
	}
}

impl State {
	/// Reverts back the values to default.
	pub fn refresh(&mut self) {
		let colored = self.colored;
		*self = Self::default();
		self.colored = colored;
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
		assert_eq!(false, state.show_options);
	}
}
