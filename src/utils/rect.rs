use super::vec2i::Vec2i;
use super::size::Size;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
	top_left: Vec2i,
	size: Size
}

impl Rectangle {
	pub fn of(top_left: Vec2i, size: Size) -> Rectangle {
		Rectangle { top_left: top_left, size: size }
	}
	
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> Rectangle {
		Rectangle::of(Vec2i::of(x, y), Size::of(width, height))
	}
	
	pub fn at(top_left: Vec2i, width: u32, height: u32) -> Rectangle {
		Rectangle::of(top_left, Size::of(width, height))
	}
	
	pub fn between(top_left: Vec2i, bottom_right: Vec2i) -> Rectangle {
		let size = Size::of_vec(bottom_right - top_left);
		Rectangle::of(top_left, size)
	}
	
	pub fn contains(&self, pos: Vec2i) -> bool {
		let tl = self.top_left();
		let br = self.bottom_right();
		pos.x > tl.x && pos.x < br.x && pos.y > tl.y && pos.y < br.y
	}
	
	pub fn merge(&self, other: Rectangle) -> Rectangle {
		Rectangle::between(
			self.top_left().min(other.top_left()),
			self.bottom_right().max(other.bottom_right())
		)
	}
	
	pub fn shrink_centered_by(&self, delta: i32) -> Rectangle {
		Rectangle::new(self.top_left.x + delta, self.top_left.y + delta, (self.width() as i32 - (delta * 2)) as u32, (self.height() as i32 - (delta * 2)) as u32)
	}
	
	pub fn moved_to(&self, new_top_left: Vec2i) -> Rectangle {
		Rectangle::of(new_top_left, self.size)
	}
	
	pub fn moved_by(&self, delta: Vec2i) -> Rectangle {
		Rectangle::of(self.top_left + delta, self.size)
	}
	
	pub fn center(&self) -> Vec2i { self.top_left + (self.size / 2).to_vec() } 
	
	pub fn top_left(&self) -> Vec2i { self.top_left }
	
	pub fn bottom_right(&self) -> Vec2i { self.top_left + self.size.to_vec() }
	
	pub fn size(&self) -> Size { self.size }
	
	pub fn width(&self) -> u32 { self.size.width }
	
	pub fn height(&self) -> u32 { self.size.height }
}
