use super::widget::Widget;
use super::widget_params::WidgetDrawParams;
use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use gui::core::graphics::Graphics;

pub struct EmptyWidget {
	bounds: WidgetBounds
}

impl EmptyWidget {
	pub fn new() -> EmptyWidget { EmptyWidget { bounds: WidgetBounds::empty() } }
}

impl Widget for EmptyWidget {
	fn render(&self, params: &mut WidgetDrawParams) {}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size { Size::of(0, 0) }
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn internal_set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
}
