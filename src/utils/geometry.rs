use super::vec2i::Vec2i;

pub struct Rectangle {
	pub top_left: Vec2i,
	pub width: u32,
	pub height: u32
}

impl Rectangle {
	pub fn at(top_left: Vec2i, width: u32, height: u32) -> Rectangle {
		return Rectangle { top_left: top_left, width: width, height: height };
	}
	
	pub fn between(top_left: Vec2i, bottom_right: Vec2i) -> Rectangle {
		let size = bottom_right - top_left;
		return Rectangle { top_left: top_left, width: size.x as u32, height: size.y as u32 };
	}
	
	pub fn size(&self) -> Vec2i {
		return Vec2i::of(self.width as i32, self.height as i32);
	}
}
