use crate::app::command::Command;
use crate::gpg::key::KeyType;

/// Application tabs.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tab {
	/// Show keys in the GPG keyring.
	Keys(KeyType),
}

impl Tab {
	/// Returns the corresponding application command.
	pub fn get_command(&self) -> Command {
		match self {
			Self::Keys(key_type) => Command::ListKeys(*key_type),
		}
	}

	/// Sets the next tab.
	pub fn next(&mut self) {
		*self = match self {
			Self::Keys(KeyType::Public) => Self::Keys(KeyType::Secret),
			_ => Self::Keys(KeyType::Public),
		}
	}

	/// Sets the previous tab.
	pub fn previous(&mut self) {
		*self = match self {
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
		let mut tab = Tab::Keys(KeyType::Public);
		assert_eq!(Command::ListKeys(KeyType::Public), tab.get_command());
		tab.next();
		assert_eq!(Tab::Keys(KeyType::Secret), tab);
		assert_ne!(Tab::Keys(KeyType::Public), tab);
		assert_eq!(Command::ListKeys(KeyType::Secret), tab.get_command());
		tab.previous();
		assert_eq!(Tab::Keys(KeyType::Public), tab);
		assert_ne!(Tab::Keys(KeyType::Secret), tab);
	}
}
