use crossterm::{
	QueueableCommand,
	ExecutableCommand, 
	cursor, 
	style::Print
};
use std::io;

type TuiStdout = std::io::BufWriter<std::io::Stdout>;

pub struct TuiCtx {
	stdout: TuiStdout,
}

impl TuiCtx {
	pub fn new() -> TuiCtx {
		TuiCtx {
			stdout: io::BufWriter::new(io::stdout())
		}
	}
	fn clear(&self) {

	}
	fn refresh(&self) {
		self.stdout.flush();
	}
	pub fn draw_room(&self, room: &RoomLayout) {
		
	}
}
