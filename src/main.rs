pub mod symbols;
pub mod monsters;
pub mod room;
pub mod player;
pub mod tui;
pub mod input;
use symbols::Symbol;
use input::InputAction;
use tui::TuiCtx;
use player::{Direction, Player};
use room::RoomLayout;
use monsters::{Monster, MonsterFlavor};

fn main() {
	let mut tui = TuiCtx::new();
	let player = Player::new();
	let mons = create_monsters!(MonsterFlavor::Spider, (4, 7),  MonsterFlavor::Spider, (5, 8));
	let mut room = RoomLayout::new();
	room.set_size((10, 10));
	room.enemy(mons[0].clone());
	room.enemy(mons[1].clone());
	room.player(player);
	while !tui.quit() {
		let action = tui.handle_events().unwrap();
		match action {
			InputAction::Quit => break,
			InputAction::Move(dir) => room.move_player(Direction::from(dir)),
			InputAction::None => {}
		}
        room.turn();
		tui.draw_room(&room);
	}
}
