use utils::rect::Rectangle;
use utils::vec2i::Vec2i;
use utils::size::Size;

#[derive(Debug)]
pub struct WidgetBounds {
	rect: Rectangle
}

impl WidgetBounds {
	pub fn of(rect: Rectangle) -> WidgetBounds {
		WidgetBounds { rect: rect }
	}
	
	pub fn from(top_left: Vec2i, size: Size) -> WidgetBounds {
		Self::of(Rectangle::of(top_left, size))
	}
	
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> WidgetBounds {
		Self::of(Rectangle::new(x, y, width, height))
	}
	
	pub fn empty() -> WidgetBounds {
		Self::new(0, 0, 0, 0)
	}
	
	pub fn map<F>(&self, mapper: F) -> WidgetBounds where F: Fn(Rectangle) -> Rectangle {
		Self::of(mapper(self.rect))
	}
	
	pub fn offset_to(&self, new_bounds: &WidgetBounds) -> Vec2i {
		new_bounds.top_left() - self.top_left()
	}
	
	pub fn rect(&self) -> Rectangle { self.rect }
	
	pub fn width(&self) -> u32 { self.rect.width() }
	
	pub fn height(&self) -> u32 { self.rect.height() }
	
	pub fn top_left(&self) -> Vec2i { self.rect.top_left() }
	
	pub fn bottom_right(&self) -> Vec2i { self.rect.bottom_right() }
}
