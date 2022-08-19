use crate::app::mode::Mode;
use crate::app::prompt::OutputType;
use crate::app::selection::Selection;
use crate::app::style::Style;
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
	/// Confirm the execution of a command.
	Confirm(Box<Command>),
	/// Show help.
	ShowHelp,
	/// Change application style.
	ChangeStyle(Style),
	/// Show application output.
	ShowOutput(OutputType, String),
	/// Show popup for options menu.
	ShowOptions,
	/// List the public/secret keys.
	ListKeys(KeyType),
	/// Import public/secret keys from files or a keyserver.
	ImportKeys(Vec<String>, bool),
	/// Import public/secret keys from clipboard.
	ImportClipboard,
	/// Export the public/secret keys.
	ExportKeys(KeyType, Vec<String>, bool),
	/// Delete the public/secret key.
	DeleteKey(KeyType, String),
	/// Send the key to the default keyserver.
	SendKey(String),
	/// Edit a key.
	EditKey(String),
	/// Sign a key.
	SignKey(String),
	/// Generate a new key pair.
	GenerateKey,
	/// Refresh the keyring.
	RefreshKeys,
	/// Copy a property to clipboard.
	Copy(Selection),
	/// Toggle the detail level.
	ToggleDetail(bool),
	/// Toggle the table size.
	ToggleTableSize,
	/// Scroll the current widget.
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
				Command::None => String::from("close menu"),
				Command::Refresh => String::from("refresh application"),
				Command::RefreshKeys => String::from("refresh the keyring"),
				Command::ShowHelp => String::from("show help"),
				Command::ChangeStyle(style) => {
					match style {
						Style::Plain => String::from("disable colors"),
						Style::Colored => String::from("enable colors"),
					}
				}
				Command::ListKeys(key_type) => {
					format!(
						"list {} keys",
						format!("{:?}", key_type).to_lowercase()
					)
				}
				Command::ImportClipboard => {
					String::from("import key(s) from clipboard")
				}
				Command::ExportKeys(key_type, patterns, ref export_subkeys) => {
					if patterns.is_empty() {
						format!("export all the keys ({})", key_type)
					} else if *export_subkeys {
						format!("export the selected subkeys ({})", key_type)
					} else {
						format!("export the selected key ({})", key_type)
					}
				}
				Command::DeleteKey(key_type, _) =>
					format!("delete the selected key ({})", key_type),
				Command::SendKey(_) =>
					String::from("send key to the keyserver"),
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
				Command::ToggleTableSize => String::from("toggle table size"),
				Command::Set(option, ref value) => {
					let action =
						if value == "true" { "enable" } else { "disable" };
					match option.as_ref() {
						"armor" => format!("{} armored output", action),
						"signer" => String::from("set as the signing key"),
						"margin" => String::from("toggle table margin"),
						"prompt" => {
							if value == ":import " {
								String::from("import key(s) from a file")
							} else if value == ":receive " {
								String::from("receive key(s) from keyserver")
							} else {
								format!("set prompt text to {}", value)
							}
						}
						_ => format!("set {} to {}", option, value),
					}
				}
				Command::SwitchMode(mode) => format!(
					"switch to {} mode",
					format!("{:?}", mode).to_lowercase()
				),
				Command::Quit => String::from("quit application"),
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
			"confirm" => Ok(Command::Confirm(Box::new(if args.is_empty() {
				Command::None
			} else {
				Command::from_str(&args.join(" "))?
			}))),
			"help" | "h" => Ok(Command::ShowHelp),
			"style" => Ok(Command::ChangeStyle(
				Style::from_str(&args.first().cloned().unwrap_or_default())
					.unwrap_or_default(),
			)),
			"output" | "out" => {
				if !args.is_empty() {
					Ok(Command::ShowOutput(
						OutputType::from(
							args.first().cloned().unwrap_or_default(),
						),
						args[1..].join(" "),
					))
				} else {
					Err(())
				}
			}
			"options" | "opt" => Ok(Command::ShowOptions),
			"list" | "ls" => Ok(Command::ListKeys(KeyType::from_str(
				&args.first().cloned().unwrap_or_else(|| String::from("pub")),
			)?)),
			"import" | "receive" => Ok(Command::ImportKeys(
				s.replacen(':', "", 1)
					.split_whitespace()
					.map(String::from)
					.skip(1)
					.collect(),
				command.as_str() == "receive",
			)),
			"import-clipboard" => Ok(Command::ImportClipboard),
			"export" | "exp" => {
				let mut patterns = if !args.is_empty() {
					args[1..].to_vec()
				} else {
					Vec::new()
				};
				let export_subkeys =
					patterns.last() == Some(&String::from("subkey"));
				if export_subkeys {
					patterns.truncate(patterns.len() - 1)
				}
				Ok(Command::ExportKeys(
					KeyType::from_str(
						&args
							.first()
							.cloned()
							.unwrap_or_else(|| String::from("pub")),
					)?,
					patterns,
					export_subkeys,
				))
			}
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
			"send" => Ok(Command::SendKey(args.first().cloned().ok_or(())?)),
			"edit" => Ok(Command::EditKey(args.first().cloned().ok_or(())?)),
			"sign" => Ok(Command::SignKey(args.first().cloned().ok_or(())?)),
			"generate" | "gen" => Ok(Command::GenerateKey),
			"copy" | "c" => {
				if let Some(arg) = args.first().cloned() {
					Ok(Command::Copy(
						Selection::from_str(&arg).map_err(|_| ())?,
					))
				} else {
					Ok(Command::SwitchMode(Mode::Copy))
				}
			}
			"toggle" | "t" => {
				if args.first() == Some(&String::from("detail")) {
					Ok(Command::ToggleDetail(
						args.get(1) == Some(&String::from("all")),
					))
				} else {
					Ok(Command::ToggleTableSize)
				}
			}
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
			"mode" | "m" => Ok(Command::SwitchMode(Mode::from_str(
				&args.first().cloned().ok_or(())?,
			)?)),
			"normal" | "n" => Ok(Command::SwitchMode(Mode::Normal)),
			"visual" | "v" => Ok(Command::SwitchMode(Mode::Visual)),
			"paste" | "p" => Ok(Command::Paste),
			"input" => Ok(Command::EnableInput),
			"search" => Ok(Command::Search(args.first().cloned())),
			"next" => Ok(Command::NextTab),
			"previous" | "prev" => Ok(Command::PreviousTab),
			"refresh" | "r" => {
				if args.first() == Some(&String::from("keys")) {
					Ok(Command::RefreshKeys)
				} else {
					Ok(Command::Refresh)
				}
			}
			"quit" | "q" | "q!" => Ok(Command::Quit),
			"none" => Ok(Command::None),
			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_command() -> Result<(), ()> {
		assert_eq!(
			Command::Confirm(Box::new(Command::None)),
			Command::from_str(":confirm none")?
		);
		assert_eq!(Command::ShowHelp, Command::from_str(":help")?);
		assert_eq!(
			Command::ShowOutput(
				OutputType::Success,
				String::from("operation successful"),
			),
			Command::from_str(":out success operation successful")?
		);
		assert_eq!(
			Command::ChangeStyle(Style::Colored),
			Command::from_str(":style colored")?
		);
		assert_eq!(
			Command::ChangeStyle(Style::Plain),
			Command::from_str(":style plain")?
		);
		assert_eq!(Command::ShowOptions, Command::from_str(":options")?);
		for cmd in &[":list", ":list pub", ":ls", ":ls pub"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::ListKeys(KeyType::Public), command);
		}
		for cmd in &[":list sec", ":ls sec"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::ListKeys(KeyType::Secret), command);
		}
		assert_eq!(
			Command::ImportKeys(
				vec![
					String::from("Test1"),
					String::from("Test2"),
					String::from("tesT3")
				],
				false
			),
			Command::from_str(":import Test1 Test2 tesT3")?
		);
		assert_eq!(
			Command::ImportKeys(vec![String::from("Test"),], true),
			Command::from_str(":receive Test")?
		);
		assert_eq!(
			Command::ImportClipboard,
			Command::from_str(":import-clipboard")?
		);
		for cmd in &[":export", ":export pub", ":exp", ":exp pub"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(
				Command::ExportKeys(KeyType::Public, Vec::new(), false),
				command
			);
		}
		assert_eq!(
			Command::ExportKeys(
				KeyType::Public,
				vec![String::from("test1"), String::from("test2")],
				false
			),
			Command::from_str(":export pub test1 test2")?
		);
		assert_eq!(
			Command::ExportKeys(
				KeyType::Secret,
				vec![String::from("test3"), String::from("test4")],
				true
			),
			Command::from_str(":export sec test3 test4 subkey")?
		);
		for cmd in &[":export sec", ":exp sec"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(
				Command::ExportKeys(KeyType::Secret, Vec::new(), false),
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
				],
				false
			),
			Command::from_str(":export sec test1 test2 test3")?
		);
		for cmd in &[":delete pub xyz", ":del pub xyz"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(
				Command::DeleteKey(KeyType::Public, String::from("xyz")),
				command
			);
		}
		assert_eq!(
			Command::SendKey(String::from("test")),
			Command::from_str(":send test")?
		);
		assert_eq!(
			Command::EditKey(String::from("test")),
			Command::from_str(":edit test")?
		);
		assert_eq!(
			Command::SignKey(String::from("test")),
			Command::from_str(":sign test")?
		);
		assert_eq!(Command::GenerateKey, Command::from_str(":generate")?);
		assert_eq!(Command::RefreshKeys, Command::from_str(":refresh keys")?);
		for cmd in &[":toggle detail all", ":t detail all"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::ToggleDetail(true), command);
		}
		assert_eq!(Command::ToggleTableSize, Command::from_str(":toggle")?);
		for cmd in &[":scroll up 1", ":scroll u 1"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::Scroll(ScrollDirection::Up(1), false), command);
		}
		for cmd in &[":set armor true", ":s armor true"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(
				Command::Set(String::from("armor"), String::from("true")),
				command
			);
		}
		for cmd in &[":get armor", ":g armor"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::Get(String::from("armor")), command);
		}
		assert_eq!(
			Command::Set(String::from("test"), String::from("_")),
			Command::from_str(":set test _")?
		);
		for cmd in &[":normal", ":n"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::SwitchMode(Mode::Normal), command);
		}
		for cmd in &[":visual", ":v"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::SwitchMode(Mode::Visual), command);
		}
		for cmd in &[":copy", ":c"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::SwitchMode(Mode::Copy), command);
		}
		for cmd in &[":paste", ":p"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::Paste, command);
		}
		assert_eq!(
			Command::Search(Some(String::from("q"))),
			Command::from_str(":search q")?
		);
		assert_eq!(Command::EnableInput, Command::from_str(":input")?);
		assert_eq!(Command::NextTab, Command::from_str(":next")?);
		assert_eq!(Command::PreviousTab, Command::from_str(":prev")?);
		assert_eq!(Command::Refresh, Command::from_str(":refresh")?);
		for cmd in &[":quit", ":q", ":q!"] {
			let command = Command::from_str(cmd)?;
			assert_eq!(Command::Quit, command);
		}
		assert_eq!(Command::None, Command::from_str(":none")?);
		assert!(Command::from_str("test").is_err());

		assert_eq!("close menu", Command::None.to_string());
		assert_eq!("show help", Command::ShowHelp.to_string());
		assert_eq!(
			"disable colors",
			Command::ChangeStyle(Style::Plain).to_string()
		);
		assert_eq!(
			"enable colors",
			Command::ChangeStyle(Style::Colored).to_string()
		);
		assert_eq!("refresh application", Command::Refresh.to_string());
		assert_eq!("refresh the keyring", Command::RefreshKeys.to_string());
		assert_eq!(
			"list public keys",
			Command::ListKeys(KeyType::Public).to_string()
		);
		assert_eq!(
			"export all the keys (sec)",
			Command::ExportKeys(KeyType::Secret, Vec::new(), false).to_string()
		);
		assert_eq!(
			"export the selected subkeys (sec)",
			Command::ExportKeys(KeyType::Secret, vec![String::new()], true)
				.to_string()
		);
		assert_eq!(
			"export the selected key (pub)",
			Command::ExportKeys(KeyType::Public, vec![String::new()], false)
				.to_string()
		);
		assert_eq!(
			"delete the selected key (pub)",
			Command::DeleteKey(KeyType::Public, String::new()).to_string()
		);
		assert_eq!(
			"send key to the keyserver",
			Command::SendKey(String::new()).to_string()
		);
		assert_eq!(
			"edit the selected key",
			Command::EditKey(String::new()).to_string()
		);
		assert_eq!(
			"sign the selected key",
			Command::SignKey(String::new()).to_string()
		);
		assert_eq!("generate a new key pair", Command::GenerateKey.to_string());
		assert_eq!(
			"copy exported key",
			Command::Copy(Selection::Key).to_string()
		);
		assert_eq!("paste from clipboard", Command::Paste.to_string());
		assert_eq!(
			"toggle detail (all)",
			Command::ToggleDetail(true).to_string()
		);
		assert_eq!(
			"toggle detail (selected)",
			Command::ToggleDetail(false).to_string()
		);
		assert_eq!("toggle table size", Command::ToggleTableSize.to_string());
		assert_eq!(
			"disable armored output",
			Command::Set(String::from("armor"), String::from("false"))
				.to_string()
		);
		assert_eq!(
			"set style to colored",
			Command::Set(String::from("style"), String::from("colored"))
				.to_string()
		);
		assert_eq!(
			"toggle table margin",
			Command::Set(String::from("margin"), String::new()).to_string()
		);
		assert_eq!(
			"import key(s) from a file",
			Command::Set(String::from("prompt"), String::from(":import "))
				.to_string()
		);
		assert_eq!(
			"import key(s) from clipboard",
			Command::ImportClipboard.to_string()
		);
		assert_eq!(
			"receive key(s) from keyserver",
			Command::Set(String::from("prompt"), String::from(":receive "))
				.to_string()
		);
		assert_eq!(
			"set prompt text to xyz",
			Command::Set(String::from("prompt"), String::from("xyz"))
				.to_string()
		);
		assert_eq!(
			"set x to y",
			Command::Set(String::from("x"), String::from("y")).to_string()
		);
		assert_eq!(
			"switch to visual mode",
			Command::SwitchMode(Mode::Visual).to_string()
		);
		assert_eq!(
			"refresh application",
			Command::Confirm(Box::new(Command::Refresh)).to_string()
		);
		assert_eq!("quit application", Command::Quit.to_string());
		assert_eq!("NextTab", Command::NextTab.to_string());
		Ok(())
	}
}
