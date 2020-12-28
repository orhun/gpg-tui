use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};

/// Draw a test block.
pub fn draw_test_block<B: Backend>(frame: &mut Frame<'_, B>, rect: Rect) {
	let block = Block::default().title("Block").borders(Borders::ALL);
	frame.render_widget(block, rect);
}
