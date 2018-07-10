use super::vec2i::Vec2i;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Size {
	pub width: u32,
	pub height: u32
}

impl Size {
	pub fn of(width: u32, height: u32) -> Size {
		Size { width: width, height: height }
	}
	
	pub fn of_vec(vector: Vec2i) -> Size {
		Size { width: vector.x as u32, height: vector.y as u32 }
	}
	
	pub fn to_vec(&self) -> Vec2i {
		Vec2i::of(self.width as i32, self.height as i32)
	}
}

impl Add<Vec2i> for Size {
	type Output = Size;
	
	fn add(self, rhs: Vec2i) -> Size {
		Size::of(
			((self.width as i32) + rhs.x) as u32,
			((self.height as i32) + rhs.y) as u32
		)
	}
}
