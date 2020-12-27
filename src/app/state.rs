/// States (flags) about the [`App`].
///
/// [`App`]: crate::app::launcher::App
#[derive(Clone, Copy, Debug)]
pub struct State {
	/// Is app running?
	pub running: bool,
}

impl Default for State {
	fn default() -> Self {
		Self { running: true }
	}
}
