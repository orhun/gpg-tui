use std::cmp::Ordering;
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
	/// Command history.
	pub history: Vec<String>,
	/// Index of the selected command from history.
	history_index: usize,
}

impl Prompt {
	/// Checks if the prompt is enabled.
	pub fn is_enabled(&self) -> bool {
		!self.text.is_empty() && self.clock.is_none()
	}

	/// Enables the command input.
	pub fn enable_command_input(&mut self) {
		self.text = if self.text.is_empty() || self.clock.is_some() {
			String::from(":")
		} else {
			format!(":{}", &self.text[1..self.text.len()])
		};
		self.clock = None;
		self.history_index = 0;
	}

	/// Checks if the command input is enabled.
	pub fn is_command_input_enabled(&self) -> bool {
		self.text.starts_with(':')
	}

	/// Enables the search.
	pub fn enable_search(&mut self) {
		self.text = if self.text.is_empty() || self.clock.is_some() {
			String::from("/")
		} else {
			format!("/{}", &self.text[1..self.text.len()])
		};
		self.clock = None;
		self.history_index = 0;
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

	/// Select the next command.
	pub fn next(&mut self) {
		match self.history_index.cmp(&1) {
			Ordering::Greater => {
				self.history_index -= 1;
				self.text = self.history
					[self.history.len() - self.history_index]
					.to_string();
			}
			Ordering::Equal => {
				self.text = String::from(":");
				self.history_index = 0;
			}
			Ordering::Less => {}
		}
	}

	/// Select the previous command.
	pub fn previous(&mut self) {
		if self.history.len() > self.history_index {
			self.text = self.history
				[self.history.len() - (self.history_index + 1)]
				.to_string();
			self.history_index += 1;
		}
	}

	/// Clears the prompt.
	pub fn clear(&mut self) {
		self.text.clear();
		self.clock = None;
		self.history_index = 0;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::{assert_eq, assert_ne};
	#[test]
	fn test_app_prompt() {
		let mut prompt = Prompt::default();
		prompt.enable_command_input();
		assert!(prompt.is_command_input_enabled());
		prompt.enable_search();
		assert!(prompt.is_search_enabled());
		assert!(prompt.is_enabled());
		prompt.set_output("test");
		assert_eq!(String::from("test"), prompt.text);
		assert_ne!(0, prompt.clock.unwrap().elapsed().as_nanos());
		assert!(!prompt.is_enabled());
		prompt.clear();
		assert_eq!(String::new(), prompt.text);
		assert_eq!(None, prompt.clock);
	}
}
