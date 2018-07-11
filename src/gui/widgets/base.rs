use super::bounds::WidgetBounds;
use utils::vec2i::Vec2i;

pub struct WidgetBase {
	pub bounds: WidgetBounds,
	pub padding: Vec2i,
	pub needs_relayout: bool
}

impl WidgetBase {
	pub fn empty() -> Self {
		Self::new(WidgetBounds::empty())
	}
	
	pub fn new(bounds: WidgetBounds) -> Self {
		WidgetBase {
			bounds: bounds,
			padding: Vec2i::of(10, 10),
			needs_relayout: true
		}
	}
}
