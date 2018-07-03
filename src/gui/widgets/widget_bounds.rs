use utils::rect::Rectangle;

pub struct WidgetBounds {
	rect: Rectangle
}

impl WidgetBounds {
	pub fn of(rect: Rectangle) -> WidgetBounds {
		WidgetBounds { rect: rect }
	}
	
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> WidgetBounds {
		WidgetBounds::of(Rectangle::new(x, y, width, height))
	}
	
	pub fn empty() -> WidgetBounds {
		WidgetBounds::new(0, 0, 0, 0)
	}
	
	pub fn map<F>(&self, mapper: F) -> WidgetBounds where F: Fn(Rectangle) -> Rectangle {
		WidgetBounds::of(mapper(self.rect))
	}
	
	pub fn rect(&self) -> Rectangle { self.rect }
}
