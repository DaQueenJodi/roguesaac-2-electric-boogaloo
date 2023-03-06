use std::fmt;
use std::io;
use crate::monsters::Monster;
use crate::symbols::Symbol;
use crate::player::{Player, Direction};

pub struct RoomLayout {
	enemies: Vec<Monster>,
	player: Player,
	size_x: usize,
	size_y: usize
}

impl RoomLayout {
	pub fn enemy(&mut self, enemy: Monster) {
		self.enemies.push(enemy);
	}
	pub fn player(&mut self, player: Player) {
		self.player = player;
	}
	pub fn set_size(&mut self, size: (usize, usize)) {
		self.size_x = size.0;
		self.size_y = size.1;
	}
	pub fn new() -> RoomLayout {
		RoomLayout {
			player: Player::new(),
			enemies: Vec::new(),
			size_x: 0,
			size_y: 0
		}
	}
	pub fn turn(&mut self) {
		let mut buff = String::new();
		io::stdin().read_line(&mut buff).unwrap();
		let dir = match buff.trim() {
			"h" => Direction::Left,
			"j" => Direction::Down,
			"k" => Direction::Up,
			"l" => Direction::Right,
			_ => return
		};
		self.player.move_dir(dir);
	}
}

impl fmt::Display for RoomLayout {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut string = String::new();
		for y in 0..self.size_x {
			for x in 0..self.size_y {
				let pos = (x, y);
				if self.player.pos() == pos {
					string += self.player.symbol();
				} else if let Some(e) = self.enemies.iter().find(|e| e.pos() == pos) {
					string += e.symbol();
				} else {
					string.push(' ');
				}
			}
			string.push('\n');
		}
		write!(f, "{string}")
	}
}
