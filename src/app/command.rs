use crate::app::clipboard::CopyType;
use crate::app::mode::Mode;
use crate::app::prompt::OutputType;
use crate::gpg::key::KeyType;
use crate::widget::row::ScrollDirection;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Command to run on rendering process.
///
/// It specifies the main operation to perform on [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
	/// Show application output.
	ShowOutput(OutputType, String),
	/// List the public/secret keys.
	ListKeys(KeyType),
	/// Export the public/secret keys.
	ExportKeys(KeyType, Vec<String>),
	/// Toggle the detail level.
	ToggleDetail(bool),
	/// Scroll the widget.
	Scroll(ScrollDirection),
	/// Set the value of an option.
	Set(String, String),
	/// Get the value of an option.
	Get(String),
	/// Switch the application mode.
	SwitchMode(Mode),
	/// Copy a property to clipboard.
	Copy(CopyType),
	/// Paste the clipboard contents.
	Paste,
	/// Enable command input.
	EnableInput,
	/// Search for a value.
	Search(Option<String>),
	/// Select the next value.
	Next,
	/// Select the previous value.
	Previous,
	/// Minimize the application.
	Minimize,
	/// Maximize the application.
	Maximize,
	/// Refresh the application.
	Refresh,
	/// Quit the application.
	Quit,
	/// Do nothing.
	None,
}

impl Display for Command {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::ShowOutput(output_type, message) =>
					format!("{:?}: {}", output_type, message),
				Self::ListKeys(key_type) => format!("list {}", key_type),
				Self::ExportKeys(key_type, _) => format!("export {}", key_type),
				Self::ToggleDetail(all) =>
					format!("toggle{}", if *all { " all" } else { "" }),
				Self::Scroll(_) => String::from("scroll"),
				Self::Set(option, value) => format!("set {} {}", option, value),
				Self::Get(option) => format!("get {}", option),
				Self::SwitchMode(mode) => mode.to_string(),
				Self::Copy(copy_type) => format!("copy: {}", copy_type),
				Self::Paste => String::from("paste"),
				Self::EnableInput => String::from("input"),
				Self::Search(_) => String::from("search"),
				Self::Next => String::from("next"),
				Self::Previous => String::from("previous"),
				Self::Minimize => String::from("minimize"),
				Self::Maximize => String::from("maximize"),
				Self::Refresh => String::from("refresh"),
				Self::Quit => String::from("quit"),
				Self::None => String::new(),
			}
		)
	}
}

impl FromStr for Command {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut values = s
			.replacen(':', "", 1)
			.to_lowercase()
			.split_whitespace()
			.map(String::from)
			.collect::<Vec<String>>();
		let command = values.first().cloned().unwrap_or_default();
		let args = values.drain(1..).collect::<Vec<String>>();
		match command.as_str() {
			"output" | "out" => Ok(Self::ShowOutput(
				OutputType::from(args.first().cloned().unwrap_or_default()),
				args[1..].join(" "),
			)),
			"list" | "ls" => Ok(Self::ListKeys(KeyType::from_str(
				&args.first().cloned().unwrap_or_else(|| String::from("pub")),
			)?)),
			"export" | "exp" => Ok(Command::ExportKeys(
				KeyType::from_str(
					&args
						.first()
						.cloned()
						.unwrap_or_else(|| String::from("pub")),
				)?,
				if !args.is_empty() {
					args[1..].to_vec()
				} else {
					Vec::new()
				},
			)),
			"toggle" | "t" => Ok(Command::ToggleDetail(
				args.first() == Some(&String::from("all")),
			)),
			"scroll" => Ok(Command::Scroll(
				ScrollDirection::from_str(&args.join(" "))
					.unwrap_or(ScrollDirection::Down(1)),
			)),
			"set" | "s" => Ok(Command::Set(
				args.get(0).cloned().unwrap_or_default(),
				args.get(1).cloned().unwrap_or_default(),
			)),
			"get" | "g" => {
				Ok(Command::Get(args.get(0).cloned().unwrap_or_default()))
			}
			"mode" | "m" => Ok(Self::SwitchMode(Mode::from_str(
				&args.first().cloned().ok_or(())?,
			)?)),
			"normal" | "n" => Ok(Self::SwitchMode(Mode::Normal)),
			"visual" | "v" => Ok(Self::SwitchMode(Mode::Visual)),
			"copy" | "c" => {
				if let Some(arg) = args.first().cloned() {
					Ok(Self::Copy(CopyType::from_str(&arg)?))
				} else {
					Ok(Self::SwitchMode(Mode::Copy))
				}
			}
			"paste" | "p" => Ok(Self::Paste),
			"input" => Ok(Self::EnableInput),
			"search" => Ok(Self::Search(args.first().cloned())),
			"next" => Ok(Self::Next),
			"previous" | "prev" => Ok(Self::Previous),
			"minimize" | "min" => Ok(Self::Minimize),
			"maximize" | "max" => Ok(Self::Maximize),
			"refresh" | "r" => Ok(Self::Refresh),
			"quit" | "q" | "q!" => Ok(Self::Quit),
			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_command() {
		assert_eq!(
			Command::ShowOutput(
				OutputType::Success,
				String::from("operation successful"),
			),
			Command::from_str(":out success operation successful").unwrap()
		);
		for cmd in &[":list", ":list pub", ":ls", ":ls pub"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::ListKeys(KeyType::Public), command);
			assert_eq!("list pub", &command.to_string())
		}
		for cmd in &[":list sec", ":ls sec"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::ListKeys(KeyType::Secret), command);
			assert_eq!("list sec", &command.to_string())
		}
		for cmd in &[":export", ":export pub", ":exp", ":exp pub"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::ExportKeys(KeyType::Public, Vec::new()),
				command
			);
			assert_eq!("export pub", &command.to_string())
		}
		assert_eq!(
			Command::ExportKeys(
				KeyType::Public,
				vec![String::from("test1"), String::from("test2")]
			),
			Command::from_str(":export pub test1 test2").unwrap()
		);
		for cmd in &[":export sec", ":exp sec"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::ExportKeys(KeyType::Secret, Vec::new()),
				command
			);
			assert_eq!("export sec", &command.to_string())
		}
		assert_eq!(
			Command::ExportKeys(
				KeyType::Secret,
				vec![
					String::from("test1"),
					String::from("test2"),
					String::from("test3")
				]
			),
			Command::from_str(":export sec test1 test2 test3").unwrap()
		);
		for cmd in &[":toggle all", ":t all"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::ToggleDetail(true), command);
			assert_eq!("toggle all", &command.to_string())
		}
		for cmd in &[":scroll up 1", ":scroll u 1"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Scroll(ScrollDirection::Up(1)), command);
			assert_eq!("scroll", &command.to_string())
		}
		for cmd in &[":set armor true", ":s armor true"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::Set(String::from("armor"), String::from("true")),
				command
			);
			assert_eq!("set armor true", &command.to_string())
		}
		for cmd in &[":get armor", ":g armor"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Get(String::from("armor")), command);
			assert_eq!("get armor", &command.to_string())
		}
		assert_eq!(
			Command::Set(String::from("test"), String::from("_")),
			Command::from_str(":set test _").unwrap()
		);
		for cmd in &[":normal", ":n"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::SwitchMode(Mode::Normal), command);
			assert_eq!("-- NORMAL --", &command.to_string())
		}
		for cmd in &[":visual", ":v"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::SwitchMode(Mode::Visual), command);
			assert_eq!("-- VISUAL --", &command.to_string())
		}
		for cmd in &[":copy", ":c"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::SwitchMode(Mode::Copy), command);
			assert_eq!("-- COPY --", &command.to_string())
		}
		for cmd in &[":paste", ":p"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Paste, command);
			assert_eq!("paste", &command.to_string())
		}
		let command = Command::from_str(":search q").unwrap();
		assert_eq!(Command::Search(Some(String::from("q"))), command);
		assert_eq!("search", &command.to_string());
		let command = Command::from_str(":input").unwrap();
		assert_eq!(Command::EnableInput, command);
		assert_eq!("input", &command.to_string());
		let command = Command::from_str(":next").unwrap();
		assert_eq!(Command::Next, command);
		assert_eq!("next", &command.to_string());
		let command = Command::from_str(":prev").unwrap();
		assert_eq!(Command::Previous, command);
		assert_eq!("previous", &command.to_string());
		for cmd in &[":minimize", ":min"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Minimize, command);
			assert_eq!("minimize", &command.to_string())
		}
		for cmd in &[":maximize", ":max"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Maximize, command);
			assert_eq!("maximize", &command.to_string())
		}
		for cmd in &[":refresh", ":r"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Refresh, command);
			assert_eq!("refresh", &command.to_string())
		}
		for cmd in &[":quit", ":q", ":q!"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Quit, command);
			assert_eq!("quit", &command.to_string())
		}
		assert!(Command::from_str("test").is_err());
	}
}
