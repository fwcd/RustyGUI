use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{max, min};

/// A mathematical vector holding two integer components.
#[derive(Copy, Clone, Debug)]
pub struct Vec2i {
	pub x: i32,
	pub y: i32
}

impl Vec2i {
	pub fn of(x: i32, y: i32) -> Vec2i {
		Vec2i { x: x, y: y }
	}
	
	pub fn length(&self) -> f64 {
		(((self.x * self.x) + (self.y * self.y)) as f64).sqrt()
	}
	
	pub fn dot(&self, rhs: Vec2i) -> i32 {
		(self.x * rhs.x) + (self.y * rhs.y)
	}
	
	pub fn max(&self, rhs: Vec2i) -> Vec2i {
		Vec2i::of(max(self.x, rhs.x), max(self.y, rhs.y))
	}
	
	pub fn min(&self, rhs: Vec2i) -> Vec2i {
		Vec2i::of(min(self.x, rhs.x), min(self.y, rhs.y))
	}
}

impl Add<Vec2i> for Vec2i {
	type Output = Vec2i;
	fn add(self, rhs: Vec2i) -> Vec2i { Vec2i::of(self.x + rhs.x, self.y + rhs.y) }
}

impl Sub<Vec2i> for Vec2i {
	type Output = Vec2i;
	fn sub(self, rhs: Vec2i) -> Vec2i { Vec2i::of(self.x - rhs.x, self.y - rhs.y) }
}

impl Mul<i32> for Vec2i {
	type Output = Vec2i;
	fn mul(self, rhs: i32) -> Vec2i { Vec2i::of(self.x * rhs, self.y * rhs) }
}

impl Div<i32> for Vec2i {
	type Output = Vec2i;
	fn div(self, rhs: i32) -> Vec2i { Vec2i::of(self.x / rhs, self.y / rhs) }
}
