use utils::vec2i::Vec2i;
use utils::geometry::Rectangle;
use gui::core::graphics::Graphics;

pub struct WidgetDrawParams {
	top_left: Vec2i,
	theme: &Theme
}

pub struct RenderedArea {
	bounds: Rectangle
}

impl WidgetDrawParams {
	pub fn at(top_left: Vec2i) -> WidgetDrawParams {
		WidgetDrawParams { top_left: top_left }
	}
	
	pub fn top_left(&self) -> &Vec2i { &self.top_left }
}

impl RenderedArea {
	pub fn of(bounds: Rectangle) -> RenderedArea {
		RenderedArea { bounds: bounds }
	}
	
	pub fn at(top_left: Vec2i, width: u32, height: u32) -> RenderedArea {
		RenderedArea { bounds: Rectangle::at(top_left, width, height) }
	}
	
	pub fn bounds(&self) -> &Rectangle { &self.bounds }
}
