use crate::gpg::key::KeyType;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::vec::IntoIter;

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
		let s = s.replacen(':', "", 1);
		for command in Command::iter() {
			if command.to_string().matches(&s).count() >= 1 {
				return Ok(command);
			} else if s.contains(&command.to_string()) {
				if let Command::ExportKeys(key_type, _) = command {
					return Ok(Command::ExportKeys(
						key_type,
						s.split_whitespace()
							.collect::<Vec<&str>>()
							.drain(2..)
							.map(String::from)
							.collect(),
					));
				}
			}
		}
		Err(())
	}
}

impl Command {
	/// Returns an iterator for `Command` variants.
	pub fn iter() -> IntoIter<Self> {
		vec![
			Command::ListKeys(KeyType::Public),
			Command::ListKeys(KeyType::Secret),
			Command::ExportKeys(KeyType::Public, Vec::new()),
			Command::ExportKeys(KeyType::Secret, Vec::new()),
			Command::Quit,
		]
		.into_iter()
	}
}
