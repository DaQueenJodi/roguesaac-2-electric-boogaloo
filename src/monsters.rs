use crate::symbols::Symbol;
use crate::room::RoomLayout;
#[derive(Clone)]
pub enum MonsterFlavor {
	Spider
}

impl Symbol for MonsterFlavor {
	fn symbol(&self) -> &str {
		match self {
			MonsterFlavor::Spider => "s"
		}
	}
}
#[derive(Clone)]
pub struct Monster {
	name: String,
	flavor: MonsterFlavor,
	health: f32,
	pos: (usize, usize),
	dead: bool,
    turn_couter: u64
}


impl Monster {
	pub fn new(flavor: MonsterFlavor) -> Monster {
		Monster {
			// TODO: make the name based on the flavor
			name: String::new(),
			flavor,
			dead: false,
			pos: (0, 0),
            turn_couter: 0,
			// TODO: make health based on the flavor
			health: 0.0
		}
	}
	pub fn with_pos(flavor: MonsterFlavor, pos: (usize, usize)) -> Monster {
		Monster {
			// TODO: make the name based on the flavor
			name: String::new(),
			flavor,
			dead: false,
			pos,
			turn_couter: 0,
			turn_callbacks: 
			// TODO: make health based on the flavor
			health: 0.0
		}
	}
	pub fn pos(&self) -> (usize, usize) {
		self.pos
	}
    pub fn turn(&mut self) {
        if self.turn_couter % 3 == 0 {
            self.move();
        }
        self.turn_couter += 1;
    }
}

impl Symbol for Monster {
	fn symbol(&self) -> &str {
		self.flavor.symbol()
	}
}


#[macro_export]
macro_rules! create_monsters {
	( $( $flavor:expr, $pos:expr),* ) => {
		{
			let mut monsters = Vec::new();
			$(
				monsters.push(crate::monsters::Monster::with_pos($flavor, $pos));
			 )*
				monsters
		}
	}
}
