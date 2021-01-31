use crate::gpg::key::KeyType;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Command to run on rendering process.
///
/// It specifies the main operation to perform on [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
	/// List the public/secret keys.
	ListKeys(KeyType),
	/// Export the public/secret keys.
	ExportKeys(KeyType, Vec<String>),
	/// Set the value of an option.
	Set(String, String),
	/// Quit the application.
	Quit,
}

impl Default for Command {
	fn default() -> Self {
		Self::ListKeys(KeyType::Public)
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::ListKeys(key_type) => format!("list {}", key_type),
				Self::ExportKeys(key_type, _) => format!("export {}", key_type),
				Self::Set(option, value) => format!("set {} {}", option, value),
				Self::Quit => String::from("quit"),
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
			"set" | "s" => Ok(Command::Set(
				args.get(0).cloned().unwrap_or_default(),
				args.get(1).cloned().unwrap_or_default(),
			)),
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
		assert_eq!(Command::ListKeys(KeyType::Public), Command::default());
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
		for cmd in &[":set armor true", ":s armor true"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(
				Command::Set(String::from("armor"), String::from("true")),
				command
			);
			assert_eq!("set armor true", &command.to_string())
		}
		assert_eq!(
			Command::Set(String::from("test"), String::from("_")),
			Command::from_str(":set test _").unwrap()
		);
		for cmd in &[":quit", ":q", ":q!"] {
			let command = Command::from_str(cmd).unwrap();
			assert_eq!(Command::Quit, command);
			assert_eq!("quit", &command.to_string())
		}
		assert!(Command::from_str("test").is_err());
	}
}
