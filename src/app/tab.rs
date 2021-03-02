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
