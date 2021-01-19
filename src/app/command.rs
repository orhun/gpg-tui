use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Command to run on rendering process.
///
/// It specifies the main operation to perform on [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Copy, Debug)]
pub enum Command {
	/// List the public keys.
	ListPublicKeys,
	/// List the secret keys.
	ListSecretKeys,
	/// Quit the application.
	Quit,
}

impl Default for Command {
	fn default() -> Self {
		Self::ListPublicKeys
	}
}

impl Display for Command {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::ListPublicKeys => "list pub",
				Self::ListSecretKeys => "list sec",
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
				return Ok(command);
			}
		}
		Err(())
	}
}

impl Command {
	/// Returns an iterator for `Command` variants.
	pub fn iterator() -> impl Iterator<Item = Self> {
		[Self::ListPublicKeys, Self::ListSecretKeys, Self::Quit]
			.iter()
			.copied()
	}
}
