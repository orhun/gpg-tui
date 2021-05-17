use crate::args::Args;
use anyhow::{anyhow, Result};
use gpgme::{Gpgme, Protocol};
use std::path::PathBuf;

/// Configuration manager for GPGME.
#[derive(Clone, Debug)]
pub struct GpgConfig {
	/// GPGME Gpgme type.
	inner: Gpgme,
	/// Flag for using ASCII armored output.
	pub armor: bool,
	/// Default key for signing operations.
	pub default_key: Option<String>,
	/// Home directory.
	pub home_dir: PathBuf,
	/// Output directory.
	pub output_dir: PathBuf,
}

impl GpgConfig {
	/// Constructs a new instance of `GpgConfig`.
	pub fn new(args: &Args) -> Result<Self> {
		let gpgme = gpgme::init();
		let home_dir = PathBuf::from(if let Some(home_dir) = &args.homedir {
			gpgme.set_engine_home_dir(Protocol::OpenPgp, home_dir)?;
			home_dir
		} else {
			gpgme
				.get_dir_info(Gpgme::HOME_DIR)
				.expect("failed to get homedir")
		});
		let mut output_dir = home_dir.join("out");
		if let Some(output) = &args.outdir {
			output_dir = output.to_path_buf();
		}
		Ok(Self {
			inner: gpgme,
			armor: args.armor,
			default_key: args.default_key.as_ref().cloned(),
			home_dir,
			output_dir,
		})
	}

	/// Returns general information about the library configuration.
	pub fn get_info(&mut self) -> Result<String> {
		let engine_info = self.inner.engine_info()?;
		match engine_info.get(gpgme::Protocol::OpenPgp) {
			Some(engine) => Ok(format!(
				r#"
				GPGME version: {}
				GPGME protocol: {}
				GPGME engine: "{}"
				GPGME engine version: {} (>{})
				GnuPG home directory: {:?}
				GnuPG data directory: "{}"
				Output directory: {:?}
				Default signing key: {}
				"#,
				self.inner.version(),
				engine.protocol(),
				engine.path().unwrap_or("?"),
				engine.version().unwrap_or("?"),
				engine.required_version().unwrap_or("?"),
				self.home_dir,
				self.get_dir_info("datadir").unwrap_or("?"),
				self.output_dir.as_os_str(),
				self.default_key
					.as_ref()
					.cloned()
					.unwrap_or_else(|| String::from("not specified")),
			)),
			None => Err(anyhow!("failed to get engine information")),
		}
	}

	/// Returns the directory information for the given value.
	pub fn get_dir_info(&self, dir: &str) -> Result<&str> {
		self.inner.get_dir_info(dir).map_err(|e| anyhow!("{:?}", e))
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
	use crate::GPGME_REQUIRED_VERSION;
	#[test]
	fn test_gpg_config() -> Result<()> {
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		config.check_gpgme_version(GPGME_REQUIRED_VERSION);
		Ok(())
	}
}
