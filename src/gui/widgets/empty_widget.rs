use super::widget::Widget;
use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use gui::core::graphics::Graphics;
use gui::themes::theme::Theme;

#[derive(Debug)]
pub struct EmptyWidget {
	bounds: WidgetBounds
}

impl EmptyWidget {
	pub fn new() -> EmptyWidget { EmptyWidget { bounds: WidgetBounds::empty() } }
}

impl Widget for EmptyWidget {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size { Size::of(0, 0) }
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
}
