use crate::app::clipboard::CopyType;
use crate::app::mode::Mode;
use crate::app::prompt::OutputType;
use crate::gpg::key::KeyType;
use crate::widget::row::ScrollDirection;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;
use std::str::FromStr;

/// Command to run on rendering process.
///
/// It specifies the main operation to perform on [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
	/// Confirm the execution of a command.
	Confirm(Box<Command>),
	/// Show application output.
	ShowOutput(OutputType, String),
	/// Show popup for options menu.
	ShowOptions,
	/// List the public/secret keys.
	ListKeys(KeyType),
	/// Import public/secret keys.
	ImportKeys(Vec<PathBuf>),
	/// Export the public/secret keys.
	ExportKeys(KeyType, Vec<String>),
	/// Delete the public/secret key.
	DeleteKey(KeyType, String),
	/// Edit a key.
	EditKey(String),
	/// Sign a key.
	SignKey(String),
	/// Generate a new key pair.
	GenerateKey,
	/// Copy a property to clipboard.
	Copy(CopyType),
	/// Toggle the detail level.
	ToggleDetail(bool),
	/// Scroll the currrent widget.
	Scroll(ScrollDirection, bool),
	/// Set the value of an option.
	Set(String, String),
	/// Get the value of an option.
	Get(String),
	/// Switch the application mode.
	SwitchMode(Mode),
	/// Paste the clipboard contents.
	Paste,
	/// Enable command input.
	EnableInput,
	/// Search for a value.
	Search(Option<String>),
	/// Select the next tab.
	NextTab,
	/// Select the previous tab.
	PreviousTab,
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
				Command::None => String::from("close"),
				Command::Refresh => String::from("refresh"),
				Command::ExportKeys(key_type, patterns) => {
					if patterns.is_empty() {
						format!("export all the keys ({})", key_type)
					} else {
						format!("export the selected key ({})", key_type)
					}
				}
				Command::DeleteKey(key_type, _) =>
					format!("delete the selected key ({})", key_type),
				Command::EditKey(_) => String::from("edit the selected key"),
				Command::SignKey(_) => String::from("sign the selected key"),
				Command::GenerateKey => String::from("generate a new key pair"),
				Command::Copy(copy_type) =>
					format!("copy {}", copy_type.to_string().to_lowercase()),
				Command::Paste => String::from("paste from clipboard"),
				Command::ToggleDetail(all) => format!(
					"toggle detail ({})",
					if *all { "all" } else { "selected" }
				),
				Command::Set(option, ref value) => {
					let action =
						if value == "true" { "enable" } else { "disable" };
					match option.as_ref() {
						"armor" => format!("{} armored output", action),
						"color" => format!("{} colors", action),
						"margin" => String::from("toggle table margin"),
						"prompt" => {
							if value == ":import " {
								String::from("import key(s)")
							} else {
								format!("set prompt text to {}", value)
							}
						}
						_ => format!("set {} to {}", option, value),
					}
				}
				Command::Minimize => String::from("minimize the table"),
				Command::Maximize => String::from("maximize the table"),
				Command::SwitchMode(mode) => format!(
					"switch to {} mode",
					format!("{:?}", mode).to_lowercase()
				),
				Command::Confirm(command) => (*command).to_string(),
				_ => format!("{:?}", self),
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
			"confirm" => Ok(Self::Confirm(Box::new(if args.is_empty() {
				Command::None
			} else {
				Command::from_str(&args.join(" "))?
			}))),
			"output" | "out" => Ok(Self::ShowOutput(
				OutputType::from(args.first().cloned().unwrap_or_default()),
				args[1..].join(" "),
			)),
			"options" | "opt" => Ok(Command::ShowOptions),
			"list" | "ls" => Ok(Self::ListKeys(KeyType::from_str(
				&args.first().cloned().unwrap_or_else(|| String::from("pub")),
			)?)),
			"import" => Ok(Command::ImportKeys(
				s.replacen(':', "", 1)
					.split_whitespace()
					.map(PathBuf::from)
					.skip(1)
					.collect(),
			)),
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
			"delete" | "del" => {
				let key_id = args.get(1).cloned().unwrap_or_default();
				Ok(Command::DeleteKey(
					KeyType::from_str(
						&args
							.get(0)
							.cloned()
							.unwrap_or_else(|| String::from("pub")),
					)?,
					if let Some(key) = key_id.strip_prefix("0x") {
						format!("0x{}", key.to_string().to_uppercase())
					} else {
						key_id
					},
				))
			}
			"edit" => Ok(Command::EditKey(args.first().cloned().ok_or(())?)),
			"sign" => Ok(Command::SignKey(args.first().cloned().ok_or(())?)),
			"generate" | "gen" => Ok(Command::GenerateKey),
			"copy" | "c" => {
				if let Some(arg) = args.first().cloned() {
					Ok(Self::Copy(CopyType::from_str(&arg)?))
				} else {
					Ok(Self::SwitchMode(Mode::Copy))
				}
			}
			"toggle" | "t" => Ok(Command::ToggleDetail(
				args.first() == Some(&String::from("all")),
			)),
			"scroll" => {
				let scroll_row = args.first() == Some(&String::from("row"));
				Ok(Command::Scroll(
					ScrollDirection::from_str(&if scroll_row {
						args[1..].join(" ")
					} else {
						args.join(" ")
					})
					.unwrap_or(ScrollDirection::Down(1)),
					scroll_row,
				))
			}
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
			"paste" | "p" => Ok(Self::Paste),
			"input" => Ok(Self::EnableInput),
			"search" => Ok(Self::Search(args.first().cloned())),
			"next" => Ok(Self::NextTab),
			"previous" | "prev" => Ok(Self::PreviousTab),
			"minimize" | "min" => Ok(Self::Minimize),
			"maximize" | "max" => Ok(Self::Maximize),
			"refresh" | "r" => Ok(Self::Refresh),
			"quit" | "q" | "q!" => Ok(Self::Quit),
			"none" => Ok(Self::None),
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
			Command::Confirm(Box::new(Command::None)),
			Command::from_str(":confirm none").unwrap()
		);
		assert_eq!(
			Command::ShowOutput(
				OutputType::Success,
				String::from("operation successful"),
			),
			Command::from_str(":out success operation successful").unwrap()
		);
		assert_eq!(
			Command::ShowOptions,
			Command::from_str(":options").unwrap()
		);
		for cmd in &[":list", ":list pub", ":ls", ":ls pub"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::ListKeys(KeyType::Public), command);
		}
		for cmd in &[":list sec", ":ls sec"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::ListKeys(KeyType::Secret), command);
		}
		assert_eq!(
			Command::ImportKeys(vec![
				PathBuf::from("Test1"),
				PathBuf::from("Test2"),
				PathBuf::from("tesT3")
			]),
			Command::from_str(":import Test1 Test2 tesT3").unwrap()
		);
		for cmd in &[":export", ":export pub", ":exp", ":exp pub"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::ExportKeys(KeyType::Public, Vec::new()),
				command
			);
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
		for cmd in &[":delete pub xyz", ":del pub xyz"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::DeleteKey(KeyType::Public, String::from("xyz")),
				command
			);
		}
		assert_eq!(
			Command::GenerateKey,
			Command::from_str(":generate").unwrap()
		);
		for cmd in &[":toggle all", ":t all"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::ToggleDetail(true), command);
		}
		for cmd in &[":scroll up 1", ":scroll u 1"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Scroll(ScrollDirection::Up(1), false), command);
		}
		for cmd in &[":set armor true", ":s armor true"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::Set(String::from("armor"), String::from("true")),
				command
			);
		}
		for cmd in &[":get armor", ":g armor"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Get(String::from("armor")), command);
		}
		assert_eq!(
			Command::Set(String::from("test"), String::from("_")),
			Command::from_str(":set test _").unwrap()
		);
		for cmd in &[":normal", ":n"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::SwitchMode(Mode::Normal), command);
		}
		for cmd in &[":visual", ":v"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::SwitchMode(Mode::Visual), command);
		}
		for cmd in &[":copy", ":c"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::SwitchMode(Mode::Copy), command);
		}
		for cmd in &[":paste", ":p"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Paste, command);
		}
		let command = Command::from_str(":search q").unwrap();
		assert_eq!(Command::Search(Some(String::from("q"))), command);
		let command = Command::from_str(":input").unwrap();
		assert_eq!(Command::EnableInput, command);
		let command = Command::from_str(":next").unwrap();
		assert_eq!(Command::NextTab, command);
		let command = Command::from_str(":prev").unwrap();
		assert_eq!(Command::PreviousTab, command);
		for cmd in &[":minimize", ":min"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Minimize, command);
		}
		for cmd in &[":maximize", ":max"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Maximize, command);
		}
		for cmd in &[":refresh", ":r"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Refresh, command);
		}
		for cmd in &[":quit", ":q", ":q!"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Quit, command);
		}
		assert_eq!(Command::None, Command::from_str(":none").unwrap());
		assert!(Command::from_str("test").is_err());
	}
}
