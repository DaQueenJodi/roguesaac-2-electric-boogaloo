use crate::symbols::Symbol;

pub trait Monster: Symbol {
	fn name(&self) -> &str;
	fn health(&self) -> u32;
	fn take_damage(&mut self, damage: u32);
	fn on_death(&mut self);
	fn is_dead(&self) -> bool;
	fn turn(&mut self);
}

pub struct Spider {
	health: u32,
	dead: bool,
}

impl Default for Spider {
	fn default() -> Spider {
		Spider {
			health: 10,
			dead: false
		}
	}
}

impl Symbol for Spider {
	fn symbol(&self) -> &str { "s" }
}

impl Monster for Spider {
	fn name(&self) -> &str { "Spider" }
	fn health(&self) -> u32 { self.health }
	fn take_damage(&mut self, damage: u32) {
		if damage >= self.health {
			self.dead = true;
		} else {
			self.health -= damage;
		}
	}
	fn on_death(&mut self) { todo!() }
	fn is_dead(&self) -> bool { self.dead }
	fn turn(&mut self) { todo!() }
}
