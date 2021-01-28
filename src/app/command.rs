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
			.split_whitespace()
			.map(String::from)
			.collect::<Vec<String>>();
		let command = values.first().cloned().unwrap_or_default();
		let args = values.drain(1..).collect::<Vec<String>>();
		if "list".matches(&command).count() >= 1 {
			Ok(Self::ListKeys(KeyType::from_str(
				&args.first().cloned().unwrap_or_else(|| String::from("pub")),
			)?))
		} else if "export".matches(&command).count() >= 1 {
			Ok(Command::ExportKeys(
				KeyType::from_str(
					&args
						.first()
						.cloned()
						.unwrap_or_else(|| String::from("pub")),
				)?,
				args[1..].to_vec(),
			))
		} else if "quit".matches(&command).count() >= 1 || command == "q!" {
			Ok(Self::Quit)
		} else {
			Err(())
		}
	}
}
