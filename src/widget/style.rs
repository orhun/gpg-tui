use colorsys::Rgb;
use std::str::FromStr;
use tui::style::Color as TuiColor;

/// Wrapper for widget colors.
#[derive(Clone, Copy, Debug)]
pub struct Color {
	/// Inner widget color type.
	inner: TuiColor,
}

impl Color {
	/// Returns the underlying [`Color`] type.
	///
	/// [`Color`]: tui::style::Color
	pub fn get(self) -> TuiColor {
		self.inner
	}
}

impl<'a> From<&'a str> for Color {
	fn from(s: &'a str) -> Self {
		Self {
			inner: match s.to_lowercase().as_ref() {
				"black" => TuiColor::Black,
				"red" => TuiColor::Red,
				"green" => TuiColor::Green,
				"yellow" => TuiColor::Yellow,
				"blue" => TuiColor::Blue,
				"magenta" => TuiColor::Magenta,
				"cyan" => TuiColor::Cyan,
				"gray" => TuiColor::Gray,
				"darkgray" => TuiColor::DarkGray,
				"lightred" => TuiColor::LightRed,
				"lightgreen" => TuiColor::LightGreen,
				"lightyellow" => TuiColor::LightYellow,
				"lightblue" => TuiColor::LightBlue,
				"lightmagenta" => TuiColor::LightMagenta,
				"lightcyan" => TuiColor::LightCyan,
				"white" => TuiColor::White,
				_ => match Rgb::from_hex_str(&format!("#{s}")) {
					Ok(rgb) => TuiColor::Rgb(
						rgb.red() as u8,
						rgb.green() as u8,
						rgb.blue() as u8,
					),
					Err(_) => Self::default().get(),
				},
			},
		}
	}
}

impl FromStr for Color {
	type Err = std::convert::Infallible;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self::from(s))
	}
}

impl Default for Color {
	fn default() -> Self {
		Self {
			inner: TuiColor::Gray,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_widget_style() {
		assert_eq!(TuiColor::Gray, Color::from("gray").get());
		assert_eq!(TuiColor::Black, Color::from("black").get());
		assert_eq!(TuiColor::Green, Color::from("green").get());
		assert_eq!(TuiColor::Gray, Color::from("xyz").get());
		assert_eq!(TuiColor::Rgb(152, 157, 69), Color::from("989D45").get());
		assert_eq!(TuiColor::Rgb(18, 49, 47), Color::from("12312F").get());
		assert_eq!(TuiColor::Rgb(255, 242, 255), Color::from("FFF2FF").get());
		assert_eq!(TuiColor::Gray, Color::from("FF00FX").get());
	}
}
