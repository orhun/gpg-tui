use crate::app::selection::Selection;
use crate::app::style::Style;
use crate::args::Args;
use crate::gpg::key::KeyDetail;
use crate::widget::style::Color;
use ratatui::style::Color as TuiColor;

/// Application states (flags) for managing the launcher.
#[derive(Clone, Debug)]
pub struct State {
	/// Is app running?
	pub running: bool,
	/// Style of the app.
	pub style: Style,
	/// Accent color of the app.
	pub color: TuiColor,
	/// Is the options menu (popup) showing?
	pub show_options: bool,
	/// Is the splash screen showing?
	pub show_splash: bool,
	/// Is the selection mode enabled?
	pub select: Option<Selection>,
	/// File explorer to run.
	pub file_explorer: String,
	/// Detail level for the keys table.
	pub detail_level: KeyDetail,
	/// Exit message of the app.
	pub exit_message: Option<String>,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: true,
			style: Style::default(),
			color: Color::default().get(),
			show_options: false,
			show_splash: false,
			select: None,
			file_explorer: String::from("xplr"),
			detail_level: KeyDetail::default(),
			exit_message: None,
		}
	}
}

impl<'a> From<&'a Args> for State {
	fn from(args: &'a Args) -> Self {
		State {
			style: args.style,
			color: args.color.get(),
			show_splash: args.splash,
			select: args.select,
			file_explorer: args.file_explorer.to_string(),
			detail_level: args.detail_level,
			..Self::default()
		}
	}
}

impl State {
	/// Reverts back the values to default.
	pub fn refresh(&mut self) {
		let style = self.style;
		let detail_level = self.detail_level;
		let color = self.color;
		*self = Self::default();
		self.style = style;
		self.detail_level = detail_level;
		self.color = color;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_state() {
		let mut state = State::default();
		state.refresh();
		assert_eq!(true, state.running);
		assert_eq!(Style::Plain, state.style);
		assert_eq!(TuiColor::Gray, state.color);
		assert_eq!(false, state.show_options);
		assert_eq!(false, state.show_splash);
		assert_eq!(None, state.select);
		assert_eq!(KeyDetail::default(), state.detail_level);
		assert_eq!("xplr", state.file_explorer);
		assert_eq!(None, state.exit_message);
	}
}
