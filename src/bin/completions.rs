use clap::{ArgEnum, CommandFactory};
use clap_complete::Shell;
use gpg_tui::args::Args;
use std::env;

/// Shell completions can be created with `cargo run --bin gpg-tui-completions`
/// in a directory specified by the environment variable [OUT_DIR].
///
/// [OUT_DIR]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
fn main() -> Result<(), std::io::Error> {
	let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
	let mut app = Args::command();
	for &shell in Shell::value_variants() {
		clap_complete::generate_to(
			shell,
			&mut app,
			env!("CARGO_PKG_NAME"),
			&out_dir,
		)?;
	}
	println!("Completion scripts are generated in {out_dir:?}");
	Ok(())
}
