use super::bounds::WidgetBounds;
use super::gui::WidgetGUI;
use utils::shared::WeakShared;
use utils::vec2i::Vec2i;

pub struct WidgetBase {
	pub bounds: WidgetBounds,
	pub padding: Vec2i,
	pub needs_relayout: bool,
	pub gui: WeakShared<WidgetGUI>
}

impl WidgetBase {
	pub fn empty() -> Self {
		Self::new(WidgetBounds::empty())
	}
	
	pub fn new(bounds: WidgetBounds) -> Self {
		WidgetBase {
			bounds: bounds,
			padding: Vec2i::of(10, 10),
			needs_relayout: true,
			gui: WeakShared::new()
		}
	}
}
