use utils::vec2i::Vec2i;

pub enum MouseButton {
	Left,
	Center,
	Right
}

pub struct MousePosEvent {
	pub pos: Vec2i
}

pub struct MouseClickEvent {
	pub pos: Vec2i,
	pub button: MouseButton
}

pub struct MouseMoveEvent {
	pub last_pos: Vec2i,
	pub pos: Vec2i
}

pub struct MouseDragEvent {
	pub last_pos: Vec2i,
	pub pos: Vec2i,
	pub button: MouseButton
}

impl MousePosEvent {
	pub fn at(pos: Vec2i) -> MousePosEvent {
		return MousePosEvent { pos: pos };
	}
}

impl MouseClickEvent {
	pub fn at(pos: Vec2i, button: MouseButton) -> MouseClickEvent {
		return MouseClickEvent { pos: pos, button: button };
	}
}

impl MouseMoveEvent {
	pub fn between(last_pos: Vec2i, pos: Vec2i) -> MouseMoveEvent {
		return MouseMoveEvent { last_pos: pos, pos: pos };
	}
}

impl MouseDragEvent {
	pub fn between(last_pos: Vec2i, pos: Vec2i, button: MouseButton) -> MouseDragEvent {
		return MouseDragEvent { last_pos: pos, pos: pos, button: button };
	}
}
