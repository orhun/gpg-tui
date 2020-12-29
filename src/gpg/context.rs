use anyhow::Result;
use gpgme::{Context as GpgContext, Key as GpgKey, KeyListMode, Protocol};

/// Representation of a key.
type Key = GpgKey;

/// A context for cryptographic operations.
pub struct Context {
	/// GPGME context type.
	inner: GpgContext,
}

impl Context {
	/// Constructs a new instance of `Context`.
	pub fn new() -> Result<Self> {
		let mut context = GpgContext::from_protocol(Protocol::OpenPgp)?;
		context.set_key_list_mode(KeyListMode::LOCAL)?;
		context.set_offline(true);
		Ok(Self { inner: context })
	}

	/// Get the list of all public keys.
	pub fn get_keys(&mut self) -> Result<Vec<Key>> {
		Ok(self
			.inner
			.find_keys(Vec::<String>::new())?
			.filter_map(|key| key.ok())
			.collect())
	}
}
