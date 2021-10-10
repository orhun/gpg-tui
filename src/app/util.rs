use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};

/// Runs the given OS command and returns the output lines.
pub fn run_os_command(cmd: &str) -> Result<Vec<String>> {
	let child = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", cmd])
			.stdout(Stdio::piped())
			.spawn()
	} else {
		Command::new("sh")
			.args(["-c", cmd])
			.stdout(Stdio::piped())
			.spawn()
	};
	match child {
		Ok(child) => {
			let output = child.wait_with_output()?;
			if output.status.success() {
				match String::from_utf8(output.stdout) {
					Ok(s) => Ok(s.lines().map(String::from).collect()),
					Err(e) => Err(anyhow!("UTF-8 error: {:?}", e)),
				}
			} else {
				Err(anyhow!("command exited with {:?}", output.status))
			}
		}
		Err(e) => Err(anyhow!("cannot run command: {:?}", e)),
	}
}
