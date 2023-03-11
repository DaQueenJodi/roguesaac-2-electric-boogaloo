use std::time::Duration;
use crate::input::InputAction;
use crate::input::MoveDirection;
use crossterm::{
	QueueableCommand,
	ExecutableCommand, 
	cursor,
	terminal::{
		Clear,
		ClearType
	},
	event::{self, Event, KeyModifiers as modifiers, KeyCode::Char},

	style::Print
};
use std::io::Write;
use crate::room::RoomLayout;
use std::io;

type TuiStdout = std::io::BufWriter<std::io::Stdout>;

pub struct TuiCtx {
	stdout: TuiStdout,
	quit: bool
}

impl TuiCtx {
	pub fn new() -> TuiCtx {
		crossterm::terminal::enable_raw_mode();
		let mut stdout = io::BufWriter::new(io::stdout());
		stdout.execute(cursor::Hide).unwrap();
		TuiCtx {
			stdout,
			quit: false
		}
	}
	fn clear(&mut self) {
		self.stdout
			.queue(Clear(ClearType::All)).unwrap()
			.queue(cursor::MoveTo(0, 0)).unwrap();
	}
	fn refresh(&mut self) {
		self.stdout.flush();
	}
	pub fn draw_room(&mut self, room: &RoomLayout) {
		self.clear();
		self.stdout.queue(Print(format!("{}", room))).unwrap();
		self.refresh();
	}
	pub fn handle_events(&mut self) -> crossterm::Result<InputAction>{
		let action: InputAction;
		if event::poll(Duration::from_millis(100))? {
			action = match event::read()? {
				Event::Key(event) => {
					match (event.modifiers, event.code) {
						(modifiers::CONTROL, Char('q')) => InputAction::Quit,
						(modifiers::NONE, Char(c)) => {
								match c {
									'h' => InputAction::Move(MoveDirection::Left),
									'j' => InputAction::Move(MoveDirection::Down),
									'k' => InputAction::Move(MoveDirection::Up),
									'l' => InputAction::Move(MoveDirection::Right),
									_ => InputAction::None
								}
						}
						_ => InputAction::None,
					}
				}
				_ => InputAction::None,
			};
		} else { action = InputAction::None }
		Ok(action)
	}
	pub fn quit(&self) -> bool {
		self.quit
	}
}

impl Drop for TuiCtx {
	// TODO: restore everything
	fn drop(&mut self) {
		self.stdout.execute(cursor::Show).unwrap();
		crossterm::terminal::disable_raw_mode();
	}
}
