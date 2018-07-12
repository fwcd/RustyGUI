use super::vec2i::Vec2i;

pub struct LineSeg {
	start: Vec2i,
	end: Vec2i
}

impl LineSeg {
	pub fn between(start: Vec2i, end: Vec2i) -> Self {
		LineSeg { start: start, end: end }
	}
	
	pub fn length(&self) -> f64 { (self.end - self.start).length() }
	
	pub fn start(&self) -> Vec2i { self.start }
	
	pub fn end(&self) -> Vec2i { self.end }
}
