use utils::vec2i::Vec2i;

#[derive(Copy, Clone)]
pub enum MouseButton {
	Left,
	Middle,
	Right,
	Other
}

#[derive(Copy, Clone)]
pub struct MousePosEvent {
	pub pos: Vec2i
}

#[derive(Copy, Clone)]
pub struct MouseClickEvent {
	pub pos: Vec2i,
	pub button: MouseButton
}

#[derive(Copy, Clone)]
pub struct MouseMoveEvent {
	pub last_pos: Vec2i,
	pub pos: Vec2i
}

#[derive(Copy, Clone)]
pub struct MouseDragEvent {
	pub last_pos: Vec2i,
	pub pos: Vec2i,
	pub button: MouseButton
}

impl MousePosEvent {
	pub fn at(pos: Vec2i) -> Self {
		Self { pos: pos }
	}
}

impl MouseClickEvent {
	pub fn at(pos: Vec2i, button: MouseButton) -> Self {
		Self { pos: pos, button: button }
	}
}

impl MouseMoveEvent {
	pub fn between(last_pos: Vec2i, pos: Vec2i) -> Self {
		Self { last_pos: last_pos, pos: pos }
	}
}

impl MouseDragEvent {
	pub fn between(last_pos: Vec2i, pos: Vec2i, button: MouseButton) -> Self {
		Self { last_pos: last_pos, pos: pos, button: button }
	}
	
	pub fn delta(&self) -> Vec2i { self.pos - self.last_pos }
}
