use super::widget::Widget;
use super::base::WidgetBase;
use utils::size::Size;
use gui::core::graphics::Graphics;
use gui::themes::theme::Theme;

pub struct EmptyWidget {
	base: WidgetBase
}

impl EmptyWidget {
	pub fn new() -> EmptyWidget { EmptyWidget { base: WidgetBase::empty() } }
}

impl Widget for EmptyWidget {
	fn render(&mut self, _graphics: &mut Graphics, _theme: &Theme) {}
	
	fn preferred_size(&self, _graphics: &Graphics) -> Size { Size::of(0, 0) }
	
	fn name(&self) -> &str { "Empty" }
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
