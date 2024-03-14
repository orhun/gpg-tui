use crate::app::banner::Banner;
use crate::app::launcher::App;
use crate::app::prompt::OutputType;
use crate::app::style;
use crate::app::tab::Tab;
use crate::widget::row::RowItem;
use crate::widget::table::TableSize;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::terminal::Frame;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::canvas::{Canvas, Points};
use ratatui::widgets::{
	Block, Borders, Clear, List, ListItem, Paragraph, Row, Table, Wrap,
};
use std::cmp;
use std::convert::{TryFrom, TryInto};
use unicode_width::UnicodeWidthStr;

/// Lengths of keys row in minimized/normal mode.
const KEYS_ROW_LENGTH: (u16, u16) = (31, 55);

/// Renders all the widgets thus the user interface.
pub fn render(app: &mut App, frame: &mut Frame) {
	let rect = frame.size();
	if app.keys_table.state.minimize_threshold != 0 {
		app.keys_table.state.size.set_minimized(
			rect.width < app.keys_table.state.minimize_threshold,
		);
	}
	if app.state.show_splash {
		render_splash_screen(app, frame, rect);
	} else {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[Constraint::Min(rect.height - 1), Constraint::Min(1)].as_ref(),
			)
			.split(rect);
		render_command_prompt(app, frame, chunks[1]);
		match app.tab {
			Tab::Keys(_) => render_keys_table(app, frame, chunks[0]),
			Tab::Help => render_help_tab(app, frame, chunks[0]),
		}
		if app.state.show_options {
			render_options_menu(app, frame, rect);
		}
	}
}

/// Renders the splash screen.
fn render_splash_screen(app: &mut App, frame: &mut Frame, rect: Rect) {
	app.state.show_splash = app.splash_screen.step != 0;
	let data = app.splash_screen.get(app.state.style.is_colored());
	frame.render_widget(
		Canvas::default()
			.x_bounds([
				0.0,
				(app.splash_screen.image.to_rgb8().width() - 1) as f64,
			])
			.y_bounds([
				0.0,
				(app.splash_screen.image.to_rgb8().height() - 1) as f64,
			])
			.paint(|p| {
				for rgb in data.keys() {
					if let Some(coords) = data.get(rgb) {
						p.draw(&Points {
							coords,
							color: Color::Rgb(rgb.0, rgb.1, rgb.2),
						})
					}
				}
			}),
		rect,
	);
}

/// Renders the command prompt.
fn render_command_prompt(app: &mut App, frame: &mut Frame, rect: Rect) {
	frame.render_widget(
		Paragraph::new(Line::from(if !app.prompt.text.is_empty() {
			vec![Span::raw(format!(
				"{}{}",
				app.prompt.output_type, app.prompt.text
			))]
		} else {
			let arrow_color = if app.state.style.is_colored() {
				Color::LightBlue
			} else {
				Color::DarkGray
			};
			vec![
				Span::styled("< ", Style::default().fg(arrow_color)),
				match app.tab {
					Tab::Keys(key_type) => Span::raw(format!(
						"list {}{}",
						key_type,
						if !app.keys_table.items.is_empty() {
							format!(
								" ({}/{})",
								app.keys_table
									.state
									.tui
									.selected()
									.unwrap_or_default() + 1,
								app.keys_table.items.len()
							)
						} else {
							String::new()
						}
					)),
					Tab::Help => Span::raw("help"),
				},
				Span::styled(" >", Style::default().fg(arrow_color)),
			]
		}))
		.style(if app.state.style.is_colored() {
			match app.prompt.output_type {
				OutputType::Success => Style::default()
					.fg(Color::LightGreen)
					.add_modifier(Modifier::BOLD),
				OutputType::Warning => Style::default()
					.fg(Color::LightYellow)
					.add_modifier(Modifier::BOLD),
				OutputType::Failure => Style::default()
					.fg(Color::LightRed)
					.add_modifier(Modifier::BOLD),
				OutputType::Action => {
					if app.state.style.is_colored() {
						Style::default()
							.fg(Color::LightBlue)
							.add_modifier(Modifier::BOLD)
					} else {
						Style::default().add_modifier(Modifier::BOLD)
					}
				}
				OutputType::None => Style::default(),
			}
		} else if app.prompt.output_type != OutputType::None {
			Style::default().add_modifier(Modifier::BOLD)
		} else {
			Style::default()
		})
		.alignment(if !app.prompt.text.is_empty() {
			Alignment::Left
		} else {
			Alignment::Right
		})
		.wrap(Wrap { trim: false }),
		rect,
	);
	if app.prompt.is_enabled() {
		frame.set_cursor(rect.x + app.prompt.text.width() as u16, rect.y + 1);
	}
}

/// Renders the help tab.
fn render_help_tab(app: &mut App, frame: &mut Frame, rect: Rect) {
	frame.render_widget(
		Block::default()
			.borders(Borders::ALL)
			.border_style(Style::default().fg(Color::DarkGray)),
		rect,
	);
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.margin(1)
		.constraints(
			[Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
		)
		.split(rect);
	{
		let description = app
			.key_bindings
			.selected()
			.map(|v| {
				v.get_description_text(
					Style::default()
						.fg(Color::DarkGray)
						.add_modifier(Modifier::ITALIC),
				)
			})
			.unwrap_or_default();
		let description_height = u16::try_from(
			app.key_bindings
				.selected()
				.map(|v| v.description.lines().count())
				.unwrap_or_default(),
		)
		.unwrap_or(1)
			+ 2;
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints(
				[
					Constraint::Min(
						chunks[0]
							.height
							.checked_sub(description_height)
							.unwrap_or_default(),
					),
					Constraint::Min(description_height),
				]
				.as_ref(),
			)
			.split(chunks[0]);
		frame.render_stateful_widget(
			List::new(
				app.key_bindings
					.items
					.iter()
					.enumerate()
					.map(|(i, v)| {
						v.as_list_item(
							app.state.style.is_colored(),
							app.key_bindings.state.selected() == Some(i),
						)
					})
					.collect::<Vec<ListItem>>(),
			)
			.block(
				Block::default()
					.borders(Borders::RIGHT)
					.border_style(Style::default().fg(Color::DarkGray)),
			)
			.style(Style::default().fg(app.state.color))
			.highlight_style(if app.state.style.is_colored() {
				Style::default().add_modifier(Modifier::BOLD)
			} else {
				Style::default()
					.fg(Color::Reset)
					.add_modifier(Modifier::BOLD)
			})
			.highlight_symbol("> "),
			chunks[0],
			&mut app.key_bindings.state,
		);
		frame.render_widget(
			Paragraph::new(description)
				.block(
					Block::default()
						.borders(Borders::RIGHT)
						.border_style(Style::default().fg(Color::DarkGray)),
				)
				.style(Style::default().fg(app.state.color))
				.alignment(Alignment::Left)
				.wrap(Wrap { trim: true }),
			chunks[1],
		);
	}
	{
		let information = match app.gpgme.config.get_info() {
			Ok(text) => text,
			Err(e) => e.to_string(),
		};
		let information_height =
			u16::try_from(information.lines().count()).unwrap_or(1) + 1;
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.margin(1)
			.constraints(
				[
					Constraint::Min(
						chunks[1]
							.height
							.checked_sub(information_height)
							.unwrap_or_default(),
					),
					Constraint::Min(information_height),
				]
				.as_ref(),
			)
			.split(chunks[1]);
		let banner = Banner::get(chunks[0]);
		frame.render_widget(
			Paragraph::new(if app.state.style.is_colored() {
				style::get_colored_info(&banner, Color::Magenta)
			} else {
				Text::raw(banner)
			})
			.block(
				Block::default()
					.borders(Borders::BOTTOM)
					.border_style(Style::default().fg(Color::DarkGray)),
			)
			.style(Style::default().fg(app.state.color))
			.alignment(Alignment::Left)
			.wrap(Wrap { trim: false }),
			chunks[0],
		);
		frame.render_widget(
			Paragraph::new(if app.state.style.is_colored() {
				style::get_colored_info(&information, Color::Cyan)
			} else {
				Text::raw(information)
			})
			.block(
				Block::default()
					.borders(Borders::NONE)
					.border_style(Style::default().fg(Color::DarkGray)),
			)
			.style(Style::default().fg(app.state.color))
			.alignment(Alignment::Left)
			.wrap(Wrap { trim: true }),
			chunks[1],
		);
	}
}

/// Renders the options menu.
fn render_options_menu(app: &mut App, frame: &mut Frame, rect: Rect) {
	let items = app
		.options
		.items
		.iter()
		.map(|v| ListItem::new(Span::raw(v.to_string())))
		.collect::<Vec<ListItem>>();
	let (length_x, mut percent_y) = (38, 60);
	let text_height = items.iter().map(|v| v.height() as f32).sum::<f32>() + 3.;
	if rect.height.checked_sub(5).unwrap_or(rect.height) as f32 > text_height {
		percent_y = ((text_height / rect.height as f32) * 100.) as u16;
	}
	let popup_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints(
			[
				Constraint::Percentage((100 - percent_y) / 2),
				Constraint::Percentage(percent_y),
				Constraint::Percentage((100 - percent_y) / 2),
			]
			.as_ref(),
		)
		.split(rect);
	let area = Layout::default()
		.direction(Direction::Horizontal)
		.constraints(
			[
				Constraint::Length(
					(popup_layout[1].width.checked_sub(length_x))
						.unwrap_or_default() / 2,
				),
				Constraint::Min(length_x),
				Constraint::Length(
					(popup_layout[1].width.checked_sub(length_x))
						.unwrap_or_default() / 2,
				),
			]
			.as_ref(),
		)
		.split(popup_layout[1])[1];
	frame.render_widget(Clear, area);
	frame.render_stateful_widget(
		List::new(items)
			.block(
				Block::default()
					.title("Options")
					.title_alignment(Alignment::Center)
					.style(if app.state.style.is_colored() {
						Style::default().fg(Color::LightBlue)
					} else {
						Style::default()
					})
					.borders(Borders::ALL),
			)
			.style(Style::default().fg(app.state.color))
			.highlight_style(
				Style::default()
					.fg(Color::Reset)
					.add_modifier(Modifier::BOLD),
			)
			.highlight_symbol("> "),
		area,
		&mut app.options.state,
	);
}

/// Renders the table of keys.
fn render_keys_table(app: &mut App, frame: &mut Frame, rect: Rect) {
	let keys_row_length = if app.keys_table.state.size != TableSize::Normal {
		KEYS_ROW_LENGTH.0
	} else {
		KEYS_ROW_LENGTH.1
	};
	frame.render_stateful_widget(
		Table::new(
			get_keys_table_rows(
				app,
				rect.width
					.checked_sub(keys_row_length + 7)
					.unwrap_or(rect.width),
				rect.height.checked_sub(2).unwrap_or(rect.height),
			),
			&[
				Constraint::Min(keys_row_length),
				Constraint::Percentage(100),
			],
		)
		.style(Style::default().fg(app.state.color))
		.highlight_style(if app.state.style.is_colored() {
			Style::default().add_modifier(Modifier::BOLD)
		} else {
			Style::default()
				.fg(Color::Reset)
				.add_modifier(Modifier::BOLD)
		})
		.highlight_symbol("> ")
		.block(
			Block::default()
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::DarkGray)),
		)
		.column_spacing(1),
		rect,
		&mut app.keys_table.state.tui,
	);
}

/// Returns the rows for keys table.
fn get_keys_table_rows<'a>(
	app: &mut App,
	max_width: u16,
	max_height: u16,
) -> Vec<Row<'a>> {
	let mut rows = Vec::new();
	app.keys_table.items = app
		.keys_table
		.items
		.clone()
		.into_iter()
		.enumerate()
		.filter(|(i, key)| {
			let subkey_info = key.get_subkey_info(
				app.gpgme.config.default_key.as_deref(),
				app.keys_table.state.size != TableSize::Normal,
			);
			let user_info = key.get_user_info(
				app.keys_table.state.size == TableSize::Minimized,
			);
			if app.prompt.is_search_enabled() {
				let search_term =
					app.prompt.text.replacen('/', "", 1).to_lowercase();
				if !subkey_info.join("\n").to_lowercase().contains(&search_term)
					&& !user_info
						.join("\n")
						.to_lowercase()
						.contains(&search_term)
				{
					return false;
				}
			}
			let keys_row = RowItem::new(
				subkey_info,
				None,
				max_height,
				app.keys_table.state.scroll,
			);
			let users_row = RowItem::new(
				user_info,
				Some(max_width),
				max_height,
				app.keys_table.state.scroll,
			);
			rows.push(
				Row::new(if app.state.style.is_colored() {
					let highlighted =
						app.keys_table.state.tui.selected() == Some(*i);
					vec![
						style::get_colored_table_row(
							&keys_row.data,
							highlighted,
						),
						style::get_colored_table_row(
							&users_row.data,
							highlighted,
						),
					]
				} else {
					vec![
						Text::from(keys_row.data.join("\n")),
						Text::from(users_row.data.join("\n")),
					]
				})
				.height(
					cmp::max(keys_row.data.len(), users_row.data.len())
						.try_into()
						.unwrap_or(1),
				)
				.bottom_margin(app.keys_table_margin)
				.style(Style::default()),
			);
			true
		})
		.map(|(_, v)| v)
		.collect();
	rows
}

#[cfg(feature = "gpg-tests")]
#[cfg(test)]
mod tests {
	use super::*;
	use crate::app::command::Command;
	use crate::args::Args;
	use crate::gpg::config::GpgConfig;
	use crate::gpg::context::GpgContext;
	use crate::gpg::key::KeyType;
	use anyhow::Result;
	use pretty_assertions::assert_eq;
	use ratatui::backend::TestBackend;
	use ratatui::buffer::Buffer;
	use ratatui::Terminal;
	use std::env;
	fn assert_buffer(mut buffer: Buffer, terminal: &Terminal<TestBackend>) {
		assert_eq!(buffer.area, terminal.backend().size().unwrap());
		for x in 0..buffer.area().width {
			for y in 0..buffer.area().height {
				buffer
					.get_mut(x, y)
					.set_style(terminal.backend().buffer().get(x, y).style());
			}
		}
		terminal.backend().assert_buffer(&buffer);
	}
	#[test]
	fn test_app_renderer() -> Result<()> {
		env::set_var(
			"GNUPGHOME",
			dirs_next::cache_dir()
				.unwrap()
				.join(env!("CARGO_PKG_NAME"))
				.to_str()
				.unwrap(),
		);
		let args = Args::default();
		let config = GpgConfig::new(&args)?;
		let mut context = GpgContext::new(config)?;
		let mut app = App::new(&mut context, &args)?;
		let backend = TestBackend::new(70, 10);
		let mut terminal = Terminal::new(backend)?;
		let test_key = format!(
			"│> [sc--] rsa3072/{} [u] test@example.org              │",
			app.gpgme.get_all_keys(None)?.get(&KeyType::Public).unwrap()[0]
				.get_id()
		)
		.replace("0x", "");
		app.run_command(Command::ListKeys(KeyType::Public))?;
		terminal.draw(|frame| render(&mut app, frame))?;
		assert_buffer(
			Buffer::with_lines(vec![
			"┌────────────────────────────────────────────────────────────────────┐",
			&test_key,
			"│                                                                    │",
			"│  [sc--] rsa4096/1BC755D9FBD24068 [?] gpg-tui@protonmail.com        │",
			"│                                                                    │",
			"│                                                                    │",
			"│                                                                    │",
			"│                                                                    │",
			"└────────────────────────────────────────────────────────────────────┘",
			"                                                    < list pub (1/2) >",
		]),
			&terminal,
		);
		app.run_command(Command::ShowOptions)?;
		terminal.draw(|frame| render(&mut app, frame))?;
		assert_buffer(
			Buffer::with_lines(vec![
			"┌────────────────────────────────────────────────────────────────────┐",
			&test_key,
			"│               ┌──────────────Options───────────────┐               │",
			"│  [sc--] rsa409│> close menu                        │ail.com        │",
			"│               │  show help                         │               │",
			"│               │  refresh application               │               │",
			"│               │  refresh the keyring               │               │",
			"│               └────────────────────────────────────┘               │",
			"└────────────────────────────────────────────────────────────────────┘",
			"                                                    < list pub (1/2) >",
			]),
			&terminal,
		);
		app.run_command(Command::ShowHelp)?;
		terminal.draw(|frame| render(&mut app, frame))?;
		let gpg_info = app
			.gpgme
			.config
			.get_info()?
			.lines()
			.map(String::from)
			.collect::<Vec<String>>();
		assert_buffer(
			Buffer::with_lines(vec![
			"┌────────────────────────────────────────────────────────────────────┐",
			"│                                                                    │",
			"│                                │                                   │",
			&format!("│ Use arrow keys / hjkl to       │  {}            │", gpg_info[1].trim()),
			&format!("│ navigate through the key       │  {}          │", gpg_info[2].trim()),
			&format!("│ bindings.                      │  {}     │", gpg_info[3].trim()),
			&format!("│ Corresponding commands and     │  {}     │", &gpg_info[4][0..32].trim()),
			"│                                                                    │",
			"└────────────────────────────────────────────────────────────────────┘",
			"                                                              < help >",
			].iter().map(|line| {
				match line.char_indices().nth(69).map(|(pos, _)| pos) {
					Some(pos) => format!("{}{}", &line[..pos], line.chars().last().unwrap_or_default()),
					None => line.to_string(),
				}
			}).collect()),
			&terminal,
		);
		Ok(())
	}
}
