use crate::args::Args;
use anyhow::{self, Result};
use gpgme::{Gpgme, Protocol};
use std::path::PathBuf;

/// Configuration manager for GPGME.
#[derive(Clone, Debug)]
pub struct GpgConfig {
	/// GPGME Gpgme type.
	inner: Gpgme,
	/// Flag for using ASCII armored output.
	pub armor: bool,
	/// Output directory.
	pub output_dir: PathBuf,
}

impl GpgConfig {
	/// Constructs a new instance of `GpgConfig`.
	pub fn new(args: &Args) -> Result<Self> {
		let gpgme = gpgme::init();
		let mut output_dir =
			PathBuf::from(if let Some(home_dir) = &args.homedir {
				gpgme.set_engine_home_dir(Protocol::OpenPgp, home_dir)?;
				home_dir
			} else {
				gpgme
					.get_dir_info(Gpgme::HOME_DIR)
					.expect("failed to get homedir")
			})
			.join("out");
		if let Some(output) = &args.output {
			output_dir = output.to_path_buf();
		}
		Ok(Self {
			inner: gpgme,
			armor: args.armor,
			output_dir,
		})
	}

	/// Checks if the linked version of the library is
	/// at least the specified version.
	pub fn check_gpgme_version(&self, version: &str) {
		assert!(self.inner.check_version(version));
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_gpg_config() -> Result<()> {
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		config.check_gpgme_version("1.7.0");
		Ok(())
	}
}
