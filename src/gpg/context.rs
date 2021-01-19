use crate::gpg::key::GpgKey;
use anyhow::Result;
use gpgme::{Context, KeyListMode, Protocol};

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
		context.set_key_list_mode(KeyListMode::SIGS)?;
		context.set_offline(true);
		Ok(Self { inner: context })
	}

	/// Returns the list of all public keys.
	pub fn get_public_keys(&mut self) -> Result<Vec<GpgKey>> {
		Ok(self
			.inner
			.find_keys(Vec::<String>::new())?
			.filter_map(|key| key.ok())
			.map(GpgKey::from)
			.collect())
	}

	/// Returns the list of all secret keys.
	pub fn get_secret_keys(&mut self) -> Result<Vec<GpgKey>> {
		Ok(self
			.inner
			.find_secret_keys(Vec::<String>::new())?
			.filter_map(|key| key.ok())
			.map(GpgKey::from)
			.collect())
	}
}
