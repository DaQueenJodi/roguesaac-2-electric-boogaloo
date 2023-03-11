use crate::symbols::Symbol;
use crate::room::RoomLayout;
use	crate::input::MoveDirection;
#[derive(Clone)]
pub struct Player {
	health: u32,
	pos: (usize, usize),
    turn_couter: u64
}

pub enum Direction {
	Left,
	Down,
	Up,
	Right
}

impl From<MoveDirection> for Direction {
	fn from(dir: MoveDirection) -> Direction {
		match dir {
			MoveDirection::Left => Direction::Left,
			MoveDirection::Down => Direction::Down,
			MoveDirection::Up => Direction::Up,
			MoveDirection::Right => Direction::Right,
		}
	}
}

impl Player {
	pub fn new() -> Player {
		Player {
			health: 0,
			pos: (0, 0),
            turn_couter: 0
		}
	}
	pub fn pos(&self) -> (usize, usize) {
		self.pos
	}
	pub fn move_dir(&mut self, dir: Direction, room: RoomLayout) {
		match dir {
			Direction::Left => if self.pos.0 > 0 {
				self.pos.0 -= 1
			},
			Direction::Down => if self.pos.1 < room.size().1 - 1{
				self.pos.1 += 1
			}
			Direction::Up => if self.pos.1 > 0 {
				self.pos.1 -= 1
			}
			Direction::Right => if self.pos.0 < room.size().0  - 1{
				self.pos.0 += 1
			}
		}
	}
    pub fn turn(&mut self) {
        self.turn_couter += 1;
    }
}

impl Symbol for Player {
	fn symbol(&self) -> &str {
		"@"
	}
}

