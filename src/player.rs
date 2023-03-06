use crate::symbols::Symbol;
use crate::room::RoomLayout;
pub struct Player {
	health: u32,
	pos: (usize, usize)
}

pub enum Direction {
	Left,
	Down,
	Up,
	Right
}

impl Player {
	pub fn new() -> Player {
		Player {
			health: 0,
			pos: (0, 0)
		}
	}
	pub fn pos(&self) -> (usize, usize) {
		self.pos
	}
	pub fn move_dir(&mut self, dir: Direction) {
		// TODO: make this actually check if it's able to move
		match dir {
			Direction::Left => self.pos.0 -= 1,
			Direction::Down => self.pos.1 += 1,
			Direction::Up => self.pos.1 -= 1,
			Direction::Right => self.pos.0 += 1
		}
	}
}

impl Symbol for Player {
	fn symbol(&self) -> &str {
		"@"
	}
}

