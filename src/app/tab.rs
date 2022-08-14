use crate::app::command::Command;
use crate::gpg::key::KeyType;

/// Application tabs.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Tab {
	/// Show help.
	Help,
	/// Show keys in the GPG keyring.
	Keys(KeyType),
}

impl Tab {
	/// Returns the corresponding application command.
	pub fn get_command(&self) -> Command {
		match self {
			Self::Keys(key_type) => Command::ListKeys(*key_type),
			Self::Help => Command::ShowHelp,
		}
	}

	/// Returns the next tab.
	pub fn next(&self) -> Self {
		match self {
			Self::Keys(KeyType::Public) => Self::Keys(KeyType::Secret),
			_ => Self::Keys(KeyType::Public),
		}
	}

	/// Returns the previous tab.
	pub fn previous(&self) -> Self {
		match self {
			Self::Keys(KeyType::Secret) => Self::Keys(KeyType::Public),
			_ => Self::Keys(KeyType::Secret),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::{assert_eq, assert_ne};
	#[test]
	fn test_app_tab() {
		let tab = Tab::Keys(KeyType::Public);
		assert_eq!(Command::ListKeys(KeyType::Public), tab.get_command());
		let tab = tab.next();
		assert_eq!(Tab::Keys(KeyType::Secret), tab);
		assert_ne!(Tab::Keys(KeyType::Public), tab);
		assert_eq!(Command::ListKeys(KeyType::Secret), tab.get_command());
		let tab = tab.previous();
		assert_eq!(Tab::Keys(KeyType::Public), tab);
		assert_ne!(Tab::Keys(KeyType::Secret), tab);
	}
}
