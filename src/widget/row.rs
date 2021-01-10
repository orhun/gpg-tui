use std::convert::TryInto;

/// Scrolling direction and offset.
#[derive(Clone, Copy, Debug)]
pub enum ScrollDirection {
	/// Scroll up.
	Up(u16),
	/// Scroll right.
	Right(u16),
	/// Scroll down.
	Down(u16),
	/// Scroll left.
	Left(u16),
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
	data: Vec<String>,
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

	/// Returns the row data as `String`.
	pub fn get(&self) -> String {
		self.data.join("\n")
	}

	/// Returns the number of lines in the row.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns `true` if the row has a length of 0.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
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
				self.limit_height();
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
	fn limit_height(&mut self) {
		let height = self.max_height;
		self.data = self
			.data
			.drain(0..(height).into())
			.enumerate()
			.map(|(i, line)| {
				if i == (height - 1).into() {
					String::from("...")
				} else {
					line
				}
			})
			.collect::<Vec<String>>()
	}
}
