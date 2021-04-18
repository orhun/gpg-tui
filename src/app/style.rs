use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

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
		Style::default().fg(Color::Reset)
	} else {
		Style::default()
	};
	let mut row = Vec::new();
	for line in row_data.iter() {
		row.push(
			// Colorize inside the brackets to start.
			if let (Some(first_bracket), Some(second_bracket)) =
				(line.find('['), line.find(']'))
			{
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
						Style::default().fg(Color::Red),
					))
				} else if data.len() == 2 {
					let style = match data.as_ref() {
						// 0x10: no indication
						"10" => Style::default().fg(Color::Yellow),
						// 0x11: personal belief but no verification
						"11" => Style::default().fg(Color::Magenta),
						// 0x12: casual verification
						"12" => Style::default().fg(Color::Blue),
						// 0x13: extensive verification
						"13" => Style::default().fg(Color::Green),
						_ => Style::default().fg(Color::Red),
					};
					colored_line.push(Span::styled(data, style))
				} else {
					for c in data.chars().map(String::from) {
						let style = match c.as_ref() {
							// GPGME_VALIDITY_UNKNOWN | GPGME_VALIDITY_UNDEFINED | 0
							"?" | "q" | "-" => {
								Style::default().fg(Color::DarkGray)
							}
							// GPGME_VALIDITY_NEVER
							"n" => Style::default().fg(Color::Red),
							// GPGME_VALIDITY_MARGINAL
							"m" => Style::default().fg(Color::Blue),
							// GPGME_VALIDITY_FULL
							"f" => Style::default().fg(Color::Magenta),
							// GPGME_VALIDITY_ULTIMATE
							"u" => Style::default().fg(Color::Green),
							// can_sign
							"s" => Style::default().fg(Color::LightGreen),
							// can_certify
							"c" => Style::default().fg(Color::LightBlue),
							// can_encrypt
							"e" => Style::default().fg(Color::Yellow),
							// can_authenticate
							"a" => Style::default().fg(Color::LightRed),
							_ => Style::default(),
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
						Style::default().fg(Color::Cyan),
					));
					colored_line.push(Span::styled(
						"/",
						Style::default().fg(Color::DarkGray),
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
						Style::default().fg(Color::DarkGray),
					));
					colored_line.push(Span::styled(
						data[first_arrow + 1..second_arrow].to_string(),
						Style::default().fg(Color::Cyan),
					));
					colored_line.push(Span::styled(
						">",
						Style::default().fg(Color::DarkGray),
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
pub fn get_colored_info(info: &str) -> Text<'_> {
	Text::from(
		info.lines()
			.map(|v| {
				let mut values = v.split(':').collect::<Vec<&str>>();
				Spans::from(if values.len() >= 2 {
					vec![
						Span::styled(
							values[0],
							Style::default().fg(Color::Reset),
						),
						Span::styled(":", Style::default().fg(Color::DarkGray)),
						Span::styled(
							values.drain(1..).collect::<Vec<&str>>().join(":"),
							Style::default().fg(Color::Cyan),
						),
					]
				} else {
					vec![Span::raw(v)]
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
						style: Style::default(),
					}]),
					Spans(vec![
						Span {
							content: Borrowed("["),
							style: Style::default(),
						},
						Span {
							content: Borrowed("s"),
							style: Style {
								fg: Some(Color::LightGreen),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("c"),
							style: Style {
								fg: Some(Color::LightBlue),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("-"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("-"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("]"),
							style: Style::default(),
						},
						Span {
							content: Borrowed(" rsa2048"),
							style: Style {
								fg: Some(Color::Cyan),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("/"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(
								"C4B2D24CF87CD188C79D00BB485B7C52E9EC0DC6"
							),
							style: Style::default(),
						},
					],),
					Spans(vec![Span {
						content: Borrowed("       └─(2020-07-29)"),
						style: Style::default(),
					}]),
					Spans(vec![Span {
						content: Borrowed("\t\t"),
						style: Style::default(),
					}]),
				],
			},
			get_colored_table_row(&row_data, false)
		);
		let row_data = r#"
[u] kmon releases <kmonlinux@protonmail.com>
	├─[13] selfsig (2020-07-29)
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
						style: Style::default(),
					}]),
					Spans(vec![
						Span {
							content: Borrowed("["),
							style: Style::default(),
						},
						Span {
							content: Borrowed("u"),
							style: Style {
								fg: Some(Color::Green),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("] kmon releases "),
							style: Style::default(),
						},
						Span {
							content: Borrowed("<"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("kmonlinux@protonmail.com"),
							style: Style {
								fg: Some(Color::Cyan),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(">"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(""),
							style: Style::default(),
						},
					]),
					Spans(vec![
						Span {
							content: Borrowed("\t├─["),
							style: Style::default(),
						},
						Span {
							content: Borrowed("13"),
							style: Style {
								fg: Some(Color::Green),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("] selfsig (2020-07-29)"),
							style: Style::default(),
						},
					]),
					Spans(vec![
						Span {
							content: Borrowed("\t└─["),
							style: Style::default(),
						},
						Span {
							content: Borrowed("10"),
							style: Style {
								fg: Some(Color::Yellow),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("] B928720AEC532117 orhun "),
							style: Style::default(),
						},
						Span {
							content: Borrowed("<"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("orhunparmaksiz@gmail.com"),
							style: Style {
								fg: Some(Color::Cyan),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(">"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(" (2020-07-29)"),
							style: Style::default(),
						},
					]),
					Spans(vec![Span {
						content: Borrowed("\t\t\t\t"),
						style: Style::default(),
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
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(":"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(" xyz "),
							style: Style {
								fg: Some(Color::Cyan),
								..Style::default()
							},
						},
					]),
					Spans(vec![
						Span {
							content: Borrowed(" test2"),
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(":"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed(" abc"),
							style: Style {
								fg: Some(Color::Cyan),
								..Style::default()
							},
						},
					]),
				],
			},
			get_colored_info("test: xyz \n test2: abc")
		)
	}
}
