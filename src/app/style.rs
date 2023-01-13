use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use tui::style::{Color, Style as TuiStyle};
use tui::text::{Span, Spans, Text};

/// Application style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
	/// Plain style with basic colors.
	Plain,
	/// More rich style with highlighted widgets and more colors.
	Colored,
}

impl Default for Style {
	fn default() -> Self {
		Self::Plain
	}
}

impl Display for Style {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", format!("{self:?}").to_lowercase())
	}
}

impl FromStr for Style {
	type Err = String;
	fn from_str(style: &str) -> Result<Self, Self::Err> {
		match style {
			"plain" => Ok(Self::Plain),
			"colored" => Ok(Self::Colored),
			_ => Err(String::from("could not parse the style")),
		}
	}
}

impl Style {
	/// Returns `true` if the style is [`Colored`].
	///
	/// [`Colored`]: Self::Colored
	pub fn is_colored(&self) -> bool {
		self == &Self::Colored
	}

	/// Returns the next style.
	pub fn next(&self) -> Self {
		match self {
			Self::Plain => Self::Colored,
			_ => Self::Plain,
		}
	}
}

/// Converts the given multi-line row value to colored [`Text`] widget.
///
/// It adds colors to:
/// * flags in bracket characters. (e.g. `[?]`)
/// * parts separated by slash character. (e.g. `rsa2048/abc123`)
/// * values in arrow characters (e.g. `<test@example.com>`)
pub fn get_colored_table_row<'a>(
	row_data: &[String],
	highlighted: bool,
) -> Text<'a> {
	let highlight_style = if highlighted {
		TuiStyle::default().fg(Color::Reset)
	} else {
		TuiStyle::default()
	};
	let mut row = Vec::new();
	for line in row_data.iter() {
		let (first_bracket, second_bracket) = (
			line.find('[').unwrap_or_default(),
			line.find(']').unwrap_or_default(),
		);
		row.push(
			// Colorize inside the brackets to start.
			if second_bracket > first_bracket + 1 {
				let data = line[first_bracket + 1..second_bracket].to_string();
				let mut colored_line = vec![Span::styled(
					line[..first_bracket + 1].to_string(),
					highlight_style,
				)];
				if vec![
					// expired
					String::from("exp"),
					// revoked
					String::from("rev"),
					// disabled
					String::from("d"),
					// invalid
					String::from("i"),
				]
				.contains(&data)
				{
					colored_line.push(Span::styled(
						data,
						TuiStyle::default().fg(Color::Red),
					))
				} else if data.len() == 2 {
					let style = match data.as_ref() {
						// 0x10: no indication
						"10" => TuiStyle::default().fg(Color::Yellow),
						// 0x11: personal belief but no verification
						"11" => TuiStyle::default().fg(Color::Magenta),
						// 0x12: casual verification
						"12" => TuiStyle::default().fg(Color::Blue),
						// 0x13: extensive verification
						"13" => TuiStyle::default().fg(Color::Green),
						_ => TuiStyle::default().fg(Color::Red),
					};
					colored_line.push(Span::styled(data, style))
				} else {
					for c in data.chars().map(String::from) {
						let style = match c.as_ref() {
							// GPGME_VALIDITY_UNKNOWN | GPGME_VALIDITY_UNDEFINED | 0
							"?" | "q" | "-" => {
								TuiStyle::default().fg(Color::DarkGray)
							}
							// GPGME_VALIDITY_NEVER
							"n" => TuiStyle::default().fg(Color::Red),
							// GPGME_VALIDITY_MARGINAL
							"m" => TuiStyle::default().fg(Color::Blue),
							// GPGME_VALIDITY_FULL
							"f" => TuiStyle::default().fg(Color::Magenta),
							// GPGME_VALIDITY_ULTIMATE | GPGME_SIG_NOTATION_HUMAN_READABLE
							"u" | "h" => TuiStyle::default().fg(Color::Green),
							// can_sign
							"s" => TuiStyle::default().fg(Color::LightGreen),
							// can_certify
							"c" => TuiStyle::default().fg(Color::LightBlue),
							// can_encrypt
							"e" => TuiStyle::default().fg(Color::Yellow),
							// can_authenticate | GPGME_SIG_NOTATION_CRITICAL
							"a" | "!" => {
								TuiStyle::default().fg(Color::LightRed)
							}
							_ => TuiStyle::default(),
						};
						colored_line.push(Span::styled(c, style))
					}
				}
				let data = line[second_bracket..].to_string();
				// Colorize the separate parts using slash character.
				if data.find('/') == Some(9) {
					colored_line.push(Span::styled(
						data.chars().next().unwrap_or_default().to_string(),
						highlight_style,
					));
					colored_line.push(Span::styled(
						data[1..9].to_string(),
						TuiStyle::default().fg(Color::Cyan),
					));
					colored_line.push(Span::styled(
						"/",
						TuiStyle::default().fg(Color::DarkGray),
					));
					colored_line.push(Span::styled(
						data[10..].to_string(),
						highlight_style,
					));
				// Colorize inside the arrows.
				} else if let (Some(first_arrow), Some(second_arrow)) =
					(data.rfind('<'), data.rfind('>'))
				{
					colored_line.push(Span::styled(
						data[..first_arrow].to_string(),
						highlight_style,
					));
					colored_line.push(Span::styled(
						"<",
						TuiStyle::default().fg(Color::DarkGray),
					));
					colored_line.push(Span::styled(
						data[first_arrow + 1..second_arrow].to_string(),
						TuiStyle::default().fg(Color::Cyan),
					));
					colored_line.push(Span::styled(
						">",
						TuiStyle::default().fg(Color::DarkGray),
					));
					colored_line.push(Span::styled(
						data[second_arrow + 1..].to_string(),
						highlight_style,
					));
				// Use the rest of the data as raw.
				} else {
					colored_line.push(Span::styled(data, highlight_style));
				}
				Spans::from(colored_line)
			// Use the unfit data as is.
			} else {
				Spans::from(vec![Span::styled(
					line.to_string(),
					highlight_style,
				)])
			},
		)
	}
	Text::from(row)
}

/// Converts the given information text to colored [`Text`] widget.
///
/// It adds colors to:
/// * parts separated by ':' character. (e.g. `version: 2`)
/// Skips the lines that starts with ' '.
pub fn get_colored_info(info: &str, color: Color) -> Text<'_> {
	Text::from(
		info.lines()
			.map(|v| {
				let mut values = v.split(':').collect::<Vec<&str>>();
				Spans::from(if values.len() >= 2 && !v.starts_with(' ') {
					vec![
						Span::styled(
							values[0],
							TuiStyle::default().fg(Color::Reset),
						),
						Span::styled(
							":",
							TuiStyle::default().fg(Color::DarkGray),
						),
						Span::styled(
							values.drain(1..).collect::<Vec<&str>>().join(":"),
							TuiStyle::default().fg(color),
						),
					]
				} else {
					vec![Span::styled(v, TuiStyle::default().fg(Color::Reset))]
				})
			})
			.collect::<Vec<Spans>>(),
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::borrow::Cow::Borrowed;
	#[test]
	fn test_app_style() {
		let row_data = r#"
[sc--] rsa2048/C4B2D24CF87CD188C79D00BB485B7C52E9EC0DC6
       └─(2020-07-29)
		"#
		.to_string()
		.lines()
		.map(String::from)
		.collect::<Vec<String>>();
		assert_eq!(
			Text {
				lines: vec![
					Spans(vec![Span {
						content: Borrowed(""),
						style: TuiStyle::default(),
					}]),
					Spans(vec![
						Span {
							content: Borrowed("["),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed("s"),
							style: TuiStyle {
								fg: Some(Color::LightGreen),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("c"),
							style: TuiStyle {
								fg: Some(Color::LightBlue),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("-"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("-"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("]"),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed(" rsa2048"),
							style: TuiStyle {
								fg: Some(Color::Cyan),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("/"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(
								"C4B2D24CF87CD188C79D00BB485B7C52E9EC0DC6"
							),
							style: TuiStyle::default(),
						},
					],),
					Spans(vec![Span {
						content: Borrowed("       └─(2020-07-29)"),
						style: TuiStyle::default(),
					}]),
					Spans(vec![Span {
						content: Borrowed("\t\t"),
						style: TuiStyle::default(),
					}]),
				],
			},
			get_colored_table_row(&row_data, false)
		);
		let row_data = r#"
[u] kmon releases <kmonlinux@protonmail.com>
	├─[13] selfsig (2020-07-29)
	├─][ test
	└─[10] B928720AEC532117 orhun <orhunparmaksiz@gmail.com> (2020-07-29)
				"#
		.to_string()
		.lines()
		.map(String::from)
		.collect::<Vec<String>>();
		assert_eq!(
			Text {
				lines: vec![
					Spans(vec![Span {
						content: Borrowed(""),
						style: TuiStyle::default(),
					}]),
					Spans(vec![
						Span {
							content: Borrowed("["),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed("u"),
							style: TuiStyle {
								fg: Some(Color::Green),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("] kmon releases "),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed("<"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("kmonlinux@protonmail.com"),
							style: TuiStyle {
								fg: Some(Color::Cyan),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(">"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(""),
							style: TuiStyle::default(),
						},
					]),
					Spans(vec![
						Span {
							content: Borrowed("\t├─["),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed("13"),
							style: TuiStyle {
								fg: Some(Color::Green),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("] selfsig (2020-07-29)"),
							style: TuiStyle::default(),
						},
					]),
					Spans(vec![Span {
						content: Borrowed("\t├─][ test"),
						style: TuiStyle::default(),
					}]),
					Spans(vec![
						Span {
							content: Borrowed("\t└─["),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed("10"),
							style: TuiStyle {
								fg: Some(Color::Yellow),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("] B928720AEC532117 orhun "),
							style: TuiStyle::default(),
						},
						Span {
							content: Borrowed("<"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed("orhunparmaksiz@gmail.com"),
							style: TuiStyle {
								fg: Some(Color::Cyan),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(">"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(" (2020-07-29)"),
							style: TuiStyle::default(),
						},
					]),
					Spans(vec![Span {
						content: Borrowed("\t\t\t\t"),
						style: TuiStyle::default(),
					}]),
				],
			},
			get_colored_table_row(&row_data, false)
		);
		assert_eq!(
			Text {
				lines: vec![
					Spans(vec![
						Span {
							content: Borrowed("test"),
							style: TuiStyle {
								fg: Some(Color::Reset),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(":"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(" xyz "),
							style: TuiStyle {
								fg: Some(Color::LightRed),
								..TuiStyle::default()
							},
						},
					]),
					Spans(vec![
						Span {
							content: Borrowed("test2"),
							style: TuiStyle {
								fg: Some(Color::Reset),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(":"),
							style: TuiStyle {
								fg: Some(Color::DarkGray),
								..TuiStyle::default()
							},
						},
						Span {
							content: Borrowed(" abc"),
							style: TuiStyle {
								fg: Some(Color::LightRed),
								..TuiStyle::default()
							},
						},
					]),
					Spans(vec![Span {
						content: Borrowed(" skip this line"),
						style: TuiStyle {
							fg: Some(Color::Reset),
							..TuiStyle::default()
						},
					}]),
					Spans(vec![Span {
						content: Borrowed("reset"),
						style: TuiStyle {
							fg: Some(Color::Reset),
							..TuiStyle::default()
						},
					}]),
				],
			},
			get_colored_info(
				"test: xyz \n\
				test2: abc\n \
				skip this line\n\
				reset",
				Color::LightRed
			)
		)
	}
}
