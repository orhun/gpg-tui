use gpg_tui::args::Args;
use std::{env, str::FromStr};
use structopt::clap::Shell;
use structopt::StructOpt;

/// Shell completions can be created with `cargo run --bin gpg-tui-completions`
/// in a directory specified by the environment variable [OUT_DIR].
///
/// [OUT_DIR]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
fn main() {
	let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
	let mut app = Args::clap();
	for variant in Shell::variants()
		.iter()
		.filter_map(|v| Shell::from_str(v).ok())
	{
		app.gen_completions(env!("CARGO_PKG_NAME"), variant, &out_dir);
	}
	println!("Completion scripts are generated in \"{}\"", out_dir);
}
