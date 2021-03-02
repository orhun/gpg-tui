use crate::gpg::key::KeyType;

/// Application tabs.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tab {
	/// Show keys in the GPG keyring.
	Keys(KeyType),
}
