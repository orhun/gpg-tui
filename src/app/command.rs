use crate::gpg::key::KeyType;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::slice::Iter;
use std::str::FromStr;

/// Command to run on rendering process.
///
/// It specifies the main operation to perform on [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Copy, Debug)]
pub enum Command {
	/// List the public/secret keys.
	ListKeys(KeyType),
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
				Self::ListKeys(KeyType::Public) => "list pub",
				Self::ListKeys(KeyType::Secret) => "list sec",
				Self::Quit => "quit",
			}
		)
	}
}

impl FromStr for Command {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.replacen(':', "", 1);
		for command in Command::iterator() {
			if command.to_string().matches(&s).count() >= 1 {
				return Ok(*command);
			}
		}
		Err(())
	}
}

impl Command {
	/// Returns an slice iterator for `Command` variants.
	pub fn iterator() -> Iter<'static, Self> {
		[
			Command::ListKeys(KeyType::Public),
			Command::ListKeys(KeyType::Secret),
			Command::Quit,
		]
		.iter()
	}
}
