use std::fmt;
use std::io;
use crate::monsters::Monster;
use crate::symbols::Symbol;
use crate::player::{Player, Direction};

#[derive(Clone)]
pub struct RoomLayout {
	enemies: Vec<Monster>,
	player: Player,
	size: (usize, usize)
}

impl RoomLayout {
	pub fn enemy(&mut self, enemy: Monster) {
		self.enemies.push(enemy);
	}
	pub fn player(&mut self, player: Player) {
		self.player = player;
	}
	pub fn size(&self) -> (usize, usize) {
		self.size
	}
	pub fn set_size(&mut self, size: (usize, usize)) {
		self.size = size;
	}
	pub fn new() -> RoomLayout {
		RoomLayout {
			player: Player::new(),
			enemies: Vec::new(),
			size: (0, 0)
		}
	}
	pub fn turn(&mut self) {
        for enemy in &mut self.enemies {
            enemy.turn()
        }
        self.player.turn()
	}
	pub fn move_player(&mut self, dir: Direction) {
		self.player.move_dir(dir, self.clone());
	}
}

impl fmt::Display for RoomLayout {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (size_x, size_y) = self.size;
		let mut string = String::new();
		// draw frame
		string.push(' ');
		for _ in 0..size_x {
			string.push('-');
		}
		string += "\r\n";
		for y in 0..size_y {
			string.push('|');
			for x in 0..size_x {
				let pos = (x, y);
				if self.player.pos() == pos {
					string += self.player.symbol();
				} else if let Some(e) = self.enemies.iter().find(|e| e.pos() == pos) {
					string += e.symbol();
				} else {
					string.push(' ');
				}
			}
			string.push('|');
			string += "\r\n";
		}
		// draw frame
		string.push(' ');
		for _ in 0..size_x {
			string.push('-');
		}
		write!(f, "{string}")
	}
}
