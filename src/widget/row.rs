use std::convert::TryInto;
use std::str::FromStr;

/// Scrolling direction and offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollDirection {
	/// Scroll up.
	Up(u16),
	/// Scroll right.
	Right(u16),
	/// Scroll down.
	Down(u16),
	/// Scroll left.
	Left(u16),
	/// Scroll to top.
	Top,
	/// Scroll to bottom.
	Bottom,
}

impl FromStr for ScrollDirection {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.split_whitespace().collect::<Vec<&str>>();
		let value = s.get(1).cloned().unwrap_or_default().parse().unwrap_or(1);
		match s.first().cloned() {
			Some("up") | Some("u") => Ok(Self::Up(value)),
			Some("right") | Some("r") => Ok(Self::Right(value)),
			Some("down") | Some("d") => Ok(Self::Down(value)),
			Some("left") | Some("l") => Ok(Self::Left(value)),
			Some("top") | Some("t") => Ok(Self::Top),
			Some("bottom") | Some("b") => Ok(Self::Bottom),
			_ => Err(()),
		}
	}
}

/// Vertical/horizontal scroll values.
#[derive(Clone, Copy, Debug, Default)]
pub struct ScrollAmount {
	/// Vertical scroll amount.
	pub vertical: u16,
	/// Horizontal scroll amount.
	pub horizontal: u16,
}

/// Row item.
#[derive(Clone, Debug)]
pub struct RowItem {
	/// Row data.
	pub data: Vec<String>,
	/// Maximum width of the row.
	max_width: Option<u16>,
	/// Maximum height of the row.
	max_height: u16,
	/// Overflow value of row height.
	height_overflow: u16,
	/// Scroll amount.
	scroll: ScrollAmount,
}

impl RowItem {
	/// Constructs a new instance of `RowItem`.
	pub fn new(
		data: Vec<String>,
		max_width: Option<u16>,
		max_height: u16,
		scroll: ScrollAmount,
	) -> Self {
		let mut item = Self {
			max_width,
			max_height,
			height_overflow: (data
				.len()
				.checked_sub(max_height.into())
				.unwrap_or_default()
				+ 1)
			.try_into()
			.unwrap_or_default(),
			scroll,
			data,
		};
		item.process();
		item
	}

	/// Processes the row data.
	///
	/// It involves scrolling vertically/horizontally
	/// and limiting the row width/height.
	fn process(&mut self) {
		if self.height_overflow != 1 {
			if self.scroll.vertical != 0 {
				self.scroll_vertical();
			}
			if self.scroll.vertical < self.height_overflow {
				self.limit_height(self.max_height);
			}
		}
		if let Some(width) = self.max_width {
			if self.scroll.horizontal != 0
				&& match self.data.iter().max_by(|x, y| x.len().cmp(&y.len())) {
					Some(line) => line.len() >= width.into(),
					None => false,
				} {
				self.scroll_horizontal();
			}
			self.limit_width(width);
		}
	}

	/// Scrolls the row vertically.
	fn scroll_vertical(&mut self) {
		self.data = self
			.data
			.iter()
			.skip(if self.scroll.vertical <= self.height_overflow {
				self.scroll.vertical.into()
			} else {
				self.height_overflow.into()
			})
			.enumerate()
			.map(|(i, line)| {
				if i == 0 {
					String::from("...")
				} else {
					line.to_string()
				}
			})
			.collect::<Vec<String>>()
	}

	/// Scrolls the row horizontally.
	fn scroll_horizontal(&mut self) {
		self.data = self
			.data
			.iter()
			.map(|line| {
				match line
					.char_indices()
					.nth((self.scroll.horizontal + 1).into())
				{
					Some((pos, _)) => {
						format!(".{}", &line[pos..])
					}
					None => String::new(),
				}
			})
			.collect::<Vec<String>>();
	}

	/// Limits the row width to match the maximum width.
	fn limit_width(&mut self, width: u16) {
		self.data = self
			.data
			.iter()
			.map(|line| match line.char_indices().nth(width.into()) {
				Some((pos, _)) => format!("{}..", &line[0..pos]),
				None => line.to_string(),
			})
			.collect::<Vec<String>>()
	}

	/// Limits the row height to match the maximum height.
	fn limit_height(&mut self, height: u16) {
		self.data = self
			.data
			.drain(0..(height).into())
			.enumerate()
			.map(|(i, line)| {
				if i == (height - 1) as usize {
					String::from("...")
				} else {
					line
				}
			})
			.collect::<Vec<String>>()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_widget_row() {
		assert_eq!(
			vec!["..", ".ne3", ".ne4", ".."],
			RowItem::new(
				vec![
					String::from("line1"),
					String::from("line2"),
					String::from("line3"),
					String::from("line4"),
					String::from("line5"),
				],
				Some(4),
				4,
				ScrollAmount {
					vertical: 1,
					horizontal: 1,
				},
			)
			.data
		);
		assert_eq!(
			ScrollDirection::Right(5),
			ScrollDirection::from_str("right 5").unwrap()
		);
		assert_eq!(
			ScrollDirection::Left(9),
			ScrollDirection::from_str("left 9").unwrap()
		);
		assert_eq!(
			ScrollDirection::Down(1),
			ScrollDirection::from_str("d").unwrap()
		);
		assert_eq!(
			ScrollDirection::Top,
			ScrollDirection::from_str("top").unwrap()
		);
		assert_eq!(
			ScrollDirection::Bottom,
			ScrollDirection::from_str("bottom").unwrap()
		);
		assert!(ScrollDirection::from_str("xyz").is_err());
	}
}
