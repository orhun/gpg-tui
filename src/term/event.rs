use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::sync::mpsc;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};
use std::thread;
use std::time::{Duration, Instant};

/// Representation of terminal events
/// ([`Crossterm events`] + [`Tick`]).
///
/// [`Crossterm events`]: crossterm::event::Event
/// [`Tick`]: Event::Tick
#[derive(Clone, Copy, Debug)]
pub enum Event {
	/// Key press.
	Key(KeyEvent),
	/// Mouse click/scroll.
	Mouse(MouseEvent),
	/// Terminal resize.
	Resize(u16, u16),
	/// Terminal tick.
	Tick,
}

/// Basic event handler for terminal [`events`].
///
/// Event types are handled in a common handler thread
/// and returned to a receiver.
///
/// [`events`]: Event
#[derive(Debug)]
pub struct EventHandler {
	/// Event sender.
	sender: mpsc::Sender<Event>,
	/// Event receiver.
	receiver: mpsc::Receiver<Event>,
	/// Event handler thread.
	handler: thread::JoinHandle<()>,
	/// Is the key input disabled?
	pub key_input_disabled: Arc<AtomicBool>,
}

impl EventHandler {
	/// Constructs a new instance of `EventHandler`.
	pub fn new(tick_rate: u64) -> Self {
		let tick_rate = Duration::from_millis(tick_rate);
		let (sender, receiver) = mpsc::channel();
		let key_input_disabled = Arc::new(AtomicBool::new(false));
		let handler = {
			let sender = sender.clone();
			let key_input_disabled = key_input_disabled.clone();
			thread::spawn(move || {
				let mut last_tick = Instant::now();
				loop {
					let timeout = tick_rate
						.checked_sub(last_tick.elapsed())
						.unwrap_or_else(|| Duration::from_secs(0));
					if !key_input_disabled.load(Ordering::Relaxed)
						&& event::poll(timeout).expect("no events available")
					{
						match event::read().expect("unable to read event") {
							CrosstermEvent::Key(e) => {
								sender.send(Event::Key(e))
							}
							CrosstermEvent::Mouse(e) => {
								sender.send(Event::Mouse(e))
							}
							CrosstermEvent::Resize(w, h) => {
								sender.send(Event::Resize(w, h))
							}
						}
						.expect("failed to send terminal event")
					}
					if last_tick.elapsed() >= tick_rate {
						sender
							.send(Event::Tick)
							.expect("failed to send tick event");
						last_tick = Instant::now();
					}
				}
			})
		};
		Self {
			sender,
			receiver,
			handler,
			key_input_disabled,
		}
	}

	/// Receive the next event from handler.
	///
	/// > This function will always block the current thread if
	/// there is no data available and it's possible for more data to be sent.
	///
	/// (Note that [`Tick`] event is frequently received depending on the tick rate.)
	///
	/// [`Tick`]: Event::Tick
	pub fn next(&self) -> Result<Event, mpsc::RecvError> {
		self.receiver.recv()
	}
}

#[cfg(feature = "tui-tests")]
#[cfg(test)]
mod tests {
	use super::*;
	use crossterm::event::{KeyCode, KeyModifiers};
	use pretty_assertions::assert_eq;
	#[test]
	fn test_term_event() -> Result<()> {
		let events = EventHandler::new(100);
		for step in 0..2 {
			if step == 1 {
				let sender = events.sender.clone();
				thread::spawn(move || {
					sender.send(Event::Key(KeyEvent::new(
						KeyCode::Esc,
						KeyModifiers::NONE,
					)))
				});
			}
			match events.next()? {
				Event::Key(key_event) => {
					if key_event.code == KeyCode::Esc {
						assert_eq!(1, step);
						break;
					}
				}
				Event::Tick => assert_eq!(0, step),
				_ => {}
			};
		}
		Ok(())
	}
}
