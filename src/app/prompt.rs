use std::time::Instant;

/// Application prompt which is responsible for
/// handling user input ([`text`]) and showing the
/// output of [`commands`] (for a while).
///
/// [`text`]: Prompt::text
/// [`commands`]: crate::app::command::Command
#[derive(Clone, Debug, Default)]
pub struct Prompt {
	/// Input/output text.
	pub text: String,
	/// Clock for tracking the duration of output messages.
	pub clock: Option<Instant>,
}

impl Prompt {
	/// Enables the user input.
	pub fn enable_input(&mut self) {
		self.text = String::from(":");
		self.clock = None;
	}

	/// Enables the search.
	pub fn enable_search(&mut self) {
		self.text = String::from("/");
		self.clock = None;
	}

	/// Checks if the input is enabled.
	pub fn is_input_enabled(&self) -> bool {
		!self.text.is_empty() && self.clock.is_none()
	}

	/// Checks if the search is enabled.
	pub fn is_search_enabled(&self) -> bool {
		self.text.starts_with('/')
	}

	/// Sets the output message.
	pub fn set_output<S: AsRef<str>>(&mut self, message: S) {
		self.text = message.as_ref().to_string();
		self.clock = Some(Instant::now());
	}

	/// Clears the prompt.
	pub fn clear(&mut self) {
		self.text.clear();
		self.clock = None;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::{assert_eq, assert_ne};
	#[test]
	fn test_app_prompt() {
		let mut prompt = Prompt::default();
		prompt.enable_input();
		assert!(prompt.is_input_enabled());
		prompt.set_output("test");
		assert_eq!(String::from("test"), prompt.text);
		assert_ne!(0, prompt.clock.unwrap().elapsed().as_nanos());
		prompt.clear();
		assert_eq!(String::new(), prompt.text);
		assert_eq!(None, prompt.clock);
	}
}
