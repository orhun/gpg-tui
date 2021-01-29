use crate::gpg::key::KeyType;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Command to run on rendering process.
///
/// It specifies the main operation to perform on [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Debug)]
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
				args[1..].to_vec(),
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
