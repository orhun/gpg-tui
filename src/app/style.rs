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
				let mut colored_line = Vec::new();
				colored_line.push(Span::styled(
					line[..first_bracket + 1].to_string(),
					highlight_style,
				));
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
