use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};

/// Runs [`xplr`] command and returns the selected files.
///
/// [`xplr`]: https://github.com/sayanarijit/xplr
pub fn run_xplr() -> Result<Vec<String>> {
	match Command::new("xplr").stdout(Stdio::piped()).spawn() {
		Ok(child) => {
			let output = child.wait_with_output()?;
			if output.status.success() {
				match String::from_utf8(output.stdout) {
					Ok(s) => Ok(s.lines().map(String::from).collect()),
					Err(e) => Err(anyhow!("UTF-8 error: {:?}", e)),
				}
			} else {
				Err(anyhow!("xplr process exited with {:?}", output.status))
			}
		}
		Err(e) => Err(anyhow!("cannot run xplr: {:?}", e)),
	}
}
