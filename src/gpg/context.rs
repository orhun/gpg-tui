use anyhow::Result;
use gpgme::{Context, Key, KeyListMode, Protocol};

/// Representation of a key.
pub type GpgKey = Key;

/// A context for cryptographic operations.
pub struct GpgContext {
	/// GPGME context type.
	inner: Context,
}

impl GpgContext {
	/// Constructs a new instance of `Context`.
	pub fn new() -> Result<Self> {
		let mut context = Context::from_protocol(Protocol::OpenPgp)?;
		context.set_key_list_mode(KeyListMode::LOCAL)?;
		context.set_offline(true);
		Ok(Self { inner: context })
	}

	/// Get the list of all public keys.
	pub fn get_keys(&mut self) -> Result<Vec<GpgKey>> {
		Ok(self
			.inner
			.find_keys(Vec::<String>::new())?
			.filter_map(|key| key.ok())
			.collect())
	}
}
