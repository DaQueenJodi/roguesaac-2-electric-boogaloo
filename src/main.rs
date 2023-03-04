pub mod symbols;
use symbols::Symbol;
pub mod monsters;
pub mod room;
use room::{Room, RoomObj};

fn main() {
	let mon = monsters::Spider::default();
	let mut room = Room::new(10, 10);
	room.layout[2][3] = RoomObj::Monster(Box::new(mon));
	let mut syms = String::new();
	for row in room.layout {
		for obj in row {
			syms += obj.symbol();
		}
	}
	println!("{}", syms);
}
