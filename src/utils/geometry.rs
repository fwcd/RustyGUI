use super::vec2i::Vec2i;

pub struct Rectangle {
	pub top_left: Vec2i,
	pub width: i32,
	pub height: i32
}

impl Rectangle {
	pub fn at(top_left: Vec2i, width: i32, height: i32) -> Rectangle {
		return Rectangle { top_left: top_left, width: width, height: height };
	}
	
	pub fn between(top_left: Vec2i, bottom_right: Vec2i) -> Rectangle {
		let size = bottom_right - top_left;
		return Rectangle { top_left: top_left, width: size.x, height: size.y };
	}
	
	pub fn size(&self) -> Vec2i {
		return Vec2i::of(self.width, self.height);
	}
}
