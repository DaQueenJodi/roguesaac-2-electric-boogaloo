pub enum MoveDirection {
	// TODO: diagnonals (maybe)
	Left,
	Down,
	Up,
	Right
}

pub enum InputAction {
	Move(MoveDirection),
	Quit,
	None
}
