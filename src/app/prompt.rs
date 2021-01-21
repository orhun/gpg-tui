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

	/// Checks if the input is enabled.
	pub fn is_input_enabled(&self) -> bool {
		!self.text.is_empty() && self.clock.is_none()
	}

	/// Sets the output message.
	pub fn set_output(&mut self, message: String) {
		self.text = message;
		self.clock = Some(Instant::now());
	}

	/// Clears the prompt.
	pub fn clear(&mut self) {
		self.text.clear();
		self.clock = None;
	}
}
