use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::Instant;

/// Output type of the prompt.
#[derive(Clone, Debug, PartialEq)]
pub enum OutputType {
	/// No output.
	None,
	/// Successful execution.
	Success,
	/// Warning about execution.
	Warning,
	/// Failed execution.
	Failure,
	/// Performed an action (such as changing the mode).
	Action,
}

impl Default for OutputType {
	fn default() -> Self {
		Self::None
	}
}

impl Display for OutputType {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::Success => "(i) ",
				Self::Warning => "(w) ",
				Self::Failure => "(e) ",
				_ => "",
			}
		)
	}
}

impl From<String> for OutputType {
	fn from(s: String) -> Self {
		match s.to_lowercase().as_str() {
			"success" => Self::Success,
			"warning" => Self::Warning,
			"failure" => Self::Failure,
			"action" => Self::Action,
			_ => Self::None,
		}
	}
}

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
	/// Output type.
	pub output_type: OutputType,
	/// Clock for tracking the duration of output messages.
	pub clock: Option<Instant>,
	/// Command history.
	pub history: Vec<String>,
	/// Index of the selected command from history.
	history_index: usize,
}

impl Prompt {
	/// Enables the prompt.
	///
	/// Available prefixes:
	/// * `:`: command input
	/// * `/`: search
	fn enable(&mut self, prefix: String) {
		self.text = if self.text.is_empty() || self.clock.is_some() {
			prefix
		} else {
			format!("{}{}", prefix, &self.text[1..self.text.len()])
		};
		self.output_type = OutputType::None;
		self.clock = None;
		self.history_index = 0;
	}

	/// Checks if the prompt is enabled.
	pub fn is_enabled(&self) -> bool {
		!self.text.is_empty() && self.clock.is_none()
	}

	/// Enables the command input.
	pub fn enable_command_input(&mut self) {
		self.enable(String::from(":"));
	}

	/// Checks if the command input is enabled.
	pub fn is_command_input_enabled(&self) -> bool {
		self.text.starts_with(':')
	}

	/// Enables the search.
	pub fn enable_search(&mut self) {
		self.enable(String::from("/"));
	}

	/// Checks if the search is enabled.
	pub fn is_search_enabled(&self) -> bool {
		self.text.starts_with('/')
	}

	/// Sets the output message.
	pub fn set_output<S: AsRef<str>>(&mut self, output: (OutputType, S)) {
		let (output_type, message) = output;
		self.output_type = output_type;
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
		self.output_type = OutputType::None;
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
		prompt.set_output((OutputType::from(String::from("success")), "Test"));
		assert_eq!(String::from("Test"), prompt.text);
		assert_eq!(OutputType::Success, prompt.output_type);
		assert_ne!(0, prompt.clock.unwrap().elapsed().as_nanos());
		assert!(!prompt.is_enabled());
		prompt.clear();
		assert_eq!(String::new(), prompt.text);
		assert_eq!(None, prompt.clock);
		prompt.history =
			vec![String::from("0"), String::from("1"), String::from("2")];
		for i in 0..prompt.history.len() {
			prompt.previous();
			assert_eq!((prompt.history.len() - i - 1).to_string(), prompt.text);
		}
		for i in 1..prompt.history.len() {
			prompt.next();
			assert_eq!(i.to_string(), prompt.text);
		}
	}
}
