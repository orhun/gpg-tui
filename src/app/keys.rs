use std::fmt::{Display, Formatter, Result as FmtResult};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::ListItem;

/// Key bindings of the application.
pub const KEY_BINDINGS: &[KeyBinding] = &[
	KeyBinding {
		key: "?",
		action: "show help",
		description: r#"
        Use arrow keys / hjkl to navigate through the key bindings.
        Corresponding commands and additional information will be shown here.
        :help
        "#,
	},
	KeyBinding {
		key: "o,space,enter",
		action: "show options",
		description: r#"
        Shows the options menu for the current tab.
        :options
        "#,
	},
	KeyBinding {
		key: "hjkl,arrows,pgkeys",
		action: "navigate",
		description: r#"
        Scrolls the current widget or selects the next/previous tab.
        M-<key>: scroll the table rows
        C-<key>,pgup,pgdown: scroll to top/bottom
        :scroll (row) up/down/left/right <amount>
        "#,
	},
	KeyBinding {
		key: "n",
		action: "switch to normal mode",
		description: r#"
        Resets the application mode.
        :normal
        "#,
	},
	KeyBinding {
		key: "v",
		action: "switch to visual mode",
		description: r#"
        Disables the mouse capture.
        :visual
        "#,
	},
	KeyBinding {
		key: "c",
		action: "switch to copy mode",
		description: r#"
        x: Copy the exported key
        i: Copy the key id
        f: Copy the key fingerprint
        u: Copy the user id
        1,2: Copy the content of the row
        :copy
        "#,
	},
	KeyBinding {
		key: "p,C-v",
		action: "paste from clipboard",
		description: ":paste",
	},
	KeyBinding {
		key: "x",
		action: "export key",
		description: r#"
        Exports the key to "$GNUPGHOME/out" or specified path via `--outdir`
        :export <pub/sec> <keyids>
        "#,
	},
	KeyBinding {
		key: "s",
		action: "sign key",
		description: r#"
        Signs the key with the default secret key.
        Same as `gpg --sign-key`
        :sign <keyid>
        "#,
	},
	KeyBinding {
		key: "e",
		action: "edit key",
		description: r#"
        Presents a menu for key management.
        Same as `gpg --edit-key`
        :edit <keyid>
        "#,
	},
	KeyBinding {
		key: "i",
		action: "import key(s)",
		description: r#"
        Imports the keys from given files.
        :import <file1> <file2>
        "#,
	},
	KeyBinding {
		key: "f",
		action: "receive key",
		description: r#"
        Imports the keys with the given key IDs from default keyserver.
        Same as `gpg --receive-keys`
        :receive <keyids>
        "#,
	},
	KeyBinding {
		key: "u",
		action: "send key",
		description: r#"
        Sends the key to the default keyserver.
        :send <keyid>
        "#,
	},
	KeyBinding {
		key: "g",
		action: "generate key",
		description: r#"
        Generates a new key pair with dialogs for all options.
        Same as `gpg --full-generate-key`
        :generate
        "#,
	},
	KeyBinding {
		key: "d,backspace",
		action: "delete key",
		description: r#"
        Removes the public/secret key from the keyring.
        :delete <pub/sec> <keyid>
        "#,
	},
	KeyBinding {
		key: "C-r",
		action: "refresh keys",
		description: r#"
        Requests updates for keys on the local keyring.
        Same as `gpg --refresh-keys`
        :refresh keys
        "#,
	},
	KeyBinding {
		key: "a",
		action: "toggle armored output",
		description: r#"
        Toggles ASCII armored output.
        The default is to create the binary OpenPGP format.
        :set armor <true/false>
        "#,
	},
	KeyBinding {
		key: "1,2,3",
		action: "set detail level",
		description: r#"
        1: Minimum
        2: Standard
        3: Full
        :set detail <level>
        "#,
	},
	KeyBinding {
		key: "t,tab",
		action: "toggle detail (all/selected)",
		description: ":toggle detail (all)",
	},
	KeyBinding {
		key: "`",
		action: "toggle table margin",
		description: ":set margin <0/1>",
	},
	KeyBinding {
		key: "m",
		action: "toggle table size",
		description: ":toggle",
	},
	KeyBinding {
		key: "C-s",
		action: "toggle style",
		description: ":set colored <true/false>",
	},
	KeyBinding {
		key: "/",
		action: "search",
		description: ":search <query>",
	},
	KeyBinding {
		key: ":",
		action: "run command",
		description: "Switches to command mode for running commands.",
	},
	KeyBinding {
		key: "r,f5",
		action: "refresh application",
		description: ":refresh",
	},
	KeyBinding {
		key: "q,C-c/d,escape",
		action: "quit application",
		description: ":quit",
	},
];

/// Representation of an individual key binding.
#[derive(Clone, Copy, Debug)]
pub struct KeyBinding<'a> {
	/// Key binding.
	key: &'a str,
	/// Brief description of the key binding action.
	action: &'a str,
	/// Full description of the action along with the commands.
	pub description: &'a str,
}

impl<'a> Display for KeyBinding<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}\n └─{}\n ",
			self.key
				.split(',')
				.fold(String::new(), |acc, v| format!("{}[{}] ", acc, v)),
			self.action
		)
	}
}

impl<'a> KeyBinding<'a> {
	/// Constructs a new instance of `KeyBinding`.
	pub fn new(key: &'a str, action: &'a str, description: &'a str) -> Self {
		Self {
			key,
			action,
			description,
		}
	}

	/// Returns the description text of the key binding.
	pub fn get_description_text(&self, command_style: Style) -> Text<'a> {
		let mut lines = Vec::new();
		for line in self.description.lines().map(|v| format!("{}\n", v.trim()))
		{
			lines.push(if line.starts_with(':') {
				Spans::from(Span::styled(line, command_style))
			} else {
				Spans::from(line)
			})
		}
		Text::from(lines)
	}

	/// Returns the key binding as a list item.
	pub fn as_list_item(
		&self,
		colored: bool,
		highlighted: bool,
	) -> ListItem<'a> {
		let highlight_style = if highlighted {
			Style::default().fg(Color::Reset)
		} else {
			Style::default()
		};
		ListItem::new(if colored {
			Text::from(vec![
				Spans::from(self.key.split(',').fold(
					Vec::new(),
					|mut keys, key| {
						keys.push(Span::styled("[", highlight_style));
						keys.push(Span::styled(
							key,
							Style::default()
								.fg(Color::Green)
								.add_modifier(Modifier::BOLD),
						));
						keys.push(Span::styled("] ", highlight_style));
						keys
					},
				)),
				Spans::from(vec![
					Span::styled(" └─", Style::default().fg(Color::DarkGray)),
					Span::styled(self.action, highlight_style),
				]),
				Spans::default(),
			])
		} else {
			Text::raw(self.to_string())
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	use std::borrow::Cow::Borrowed;
	#[test]
	fn test_app_keys() {
		let key_binding =
			KeyBinding::new("q,esc", "quit", "quits the application\n:quit");
		assert_eq!("quits the application\n:quit", key_binding.description);
		assert_eq!(
			Text {
				lines: vec![
					Spans(vec![Span {
						content: Borrowed("quits the application\n"),
						style: Style::default(),
					}]),
					Spans(vec![Span {
						content: Borrowed(":quit\n"),
						style: Style::default().fg(Color::Red),
					}]),
				],
			},
			key_binding.get_description_text(Style::default().fg(Color::Red))
		);
		assert_eq!(
			ListItem::new(Text {
				lines: vec![
					Spans(vec![Span {
						content: Borrowed("[q] [esc] "),
						style: Style::default(),
					}]),
					Spans(vec![Span {
						content: Borrowed(" └─quit"),
						style: Style::default(),
					}]),
					Spans(vec![Span {
						content: Borrowed(" "),
						style: Style::default(),
					}]),
				],
			}),
			key_binding.as_list_item(false, false)
		);
		assert_eq!(
			ListItem::new(Text {
				lines: vec![
					Spans(vec![
						Span {
							content: Borrowed("["),
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("q"),
							style: Style {
								fg: Some(Color::Green),
								bg: None,
								add_modifier: Modifier::BOLD,
								sub_modifier: Modifier::empty(),
							},
						},
						Span {
							content: Borrowed("] "),
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("["),
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("esc"),
							style: Style {
								fg: Some(Color::Green),
								bg: None,
								add_modifier: Modifier::BOLD,
								sub_modifier: Modifier::empty(),
							},
						},
						Span {
							content: Borrowed("] "),
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
					]),
					Spans(vec![
						Span {
							content: Borrowed(" └─"),
							style: Style {
								fg: Some(Color::DarkGray),
								..Style::default()
							},
						},
						Span {
							content: Borrowed("quit"),
							style: Style {
								fg: Some(Color::Reset),
								..Style::default()
							},
						},
					]),
					Spans::default(),
				]
			}),
			key_binding.as_list_item(true, true)
		);
	}
}
