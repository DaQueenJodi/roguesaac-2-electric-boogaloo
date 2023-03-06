pub mod symbols;
use symbols::Symbol;
pub mod monsters;
use monsters::{Monster, MonsterFlavor};
pub mod room;
use room::RoomLayout;
pub mod player;
use player::Player;
pub mod tui;

fn main() {
	let player = Player::new();
	let mons = create_monsters!(MonsterFlavor::Spider, (4, 7),  MonsterFlavor::Spider, (5, 8));
	let mut room = RoomLayout::new();
	room.set_size((10, 10));
	room.enemy(mons[0].clone());
	room.enemy(mons[1].clone());
	room.player(player);
	loop {
		room.turn();
		println!("{room}");
	}
}
