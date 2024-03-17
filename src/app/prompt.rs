use log::Level;

use crate::app::command::Command;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::time::Instant;

/// Prefix character for indicating command input.
pub const COMMAND_PREFIX: char = ':';
/// Prefix character for indicating search input.
pub const SEARCH_PREFIX: char = '/';

/// Output type of the prompt.
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl OutputType {
	/// Converts output type to log level.
	pub fn as_log_level(&self) -> Level {
		match self {
			OutputType::None => Level::Trace,
			OutputType::Success => Level::Info,
			OutputType::Warning => Level::Warn,
			OutputType::Failure => Level::Error,
			OutputType::Action => Level::Info,
		}
	}
}

/// Application prompt which is responsible for
/// handling user input ([`text`]), showing the
/// output of [`commands`] and ask for confirmation.
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
	/// Command that will be confirmed for execution.
	pub command: Option<Command>,
	/// Command history.
	pub history: Vec<String>,
	/// Index of the selected command from history.
	pub history_index: usize,
}

impl Prompt {
	/// Enables the prompt.
	///
	/// Available prefixes:
	/// * `:`: command input
	/// * `/`: search
	fn enable(&mut self, prefix: char) {
		self.text = if self.text.is_empty() || self.clock.is_some() {
			prefix.to_string()
		} else {
			format!("{}{}", prefix, &self.text[1..self.text.len()])
		};
		self.output_type = OutputType::None;
		self.clock = None;
		self.command = None;
		self.history_index = 0;
	}

	/// Checks if the prompt is enabled.
	pub fn is_enabled(&self) -> bool {
		!self.text.is_empty() && self.clock.is_none() && self.command.is_none()
	}

	/// Enables the command input.
	pub fn enable_command_input(&mut self) {
		self.enable(COMMAND_PREFIX);
	}

	/// Checks if the command input is enabled.
	pub fn is_command_input_enabled(&self) -> bool {
		self.text.starts_with(COMMAND_PREFIX)
	}

	/// Enables the search.
	pub fn enable_search(&mut self) {
		self.enable(SEARCH_PREFIX);
	}

	/// Checks if the search is enabled.
	pub fn is_search_enabled(&self) -> bool {
		self.text.starts_with(SEARCH_PREFIX)
	}

	/// Sets the output message.
	pub fn set_output<S: AsRef<str>>(&mut self, output: (OutputType, S)) {
		let (output_type, message) = output;
		log::log!(target: "tui", self.output_type.as_log_level(), "{}", message.as_ref().to_string());
		self.output_type = output_type;
		self.text = message.as_ref().to_string();
		self.clock = Some(Instant::now());
	}

	/// Sets the command that will be asked to confirm.
	pub fn set_command(&mut self, command: Command) {
		self.text = format!("press 'y' to {command}");
		self.output_type = OutputType::Action;
		self.command = Some(command);
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
		self.command = None;
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
		assert_ne!(
			0,
			prompt
				.clock
				.expect("could not get clock")
				.elapsed()
				.as_nanos()
		);
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
		for output_type in [
			OutputType::from(String::from("warning")),
			OutputType::from(String::from("failure")),
			OutputType::from(String::from("action")),
			OutputType::from(String::from("test")),
		] {
			assert_eq!(
				match output_type {
					OutputType::Warning => "(w) ",
					OutputType::Failure => "(e) ",
					_ => "",
				},
				&output_type.to_string()
			);
		}
	}
}
