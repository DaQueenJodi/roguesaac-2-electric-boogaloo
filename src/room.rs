use crate::monsters::Monster;
use crate::symbols::Symbol;
pub enum RoomObj {
	Monster(Box<dyn Monster>),
	Nothing,
	WallHorizontal,
	WallVertical
}

impl Symbol for RoomObj {
	fn symbol(&self) -> &str {
		match self {
			RoomObj::Nothing => " ",
			RoomObj::WallHorizontal => "_",
			RoomObj::WallVertical => "|",
			RoomObj::Monster(mon) => mon.symbol()
		}
	}
}

pub struct Room {
	pub layout: Vec<Vec<RoomObj>>
}

impl Room {
	pub fn new(size_x: usize, size_y: usize) -> Room {
		let mut layout = Vec::with_capacity(size_y);
		for _ in 0..size_y {
			layout.push(Vec::with_capacity(size_x));
		}
		Room {
			layout
		}
	}
}
