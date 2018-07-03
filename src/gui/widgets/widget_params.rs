use utils::vec2i::Vec2i;
use gui::themes::theme::Theme;
use gui::core::graphics::Graphics;

pub struct WidgetDrawParams<'a> {
	graphics: &'a mut Graphics,
	theme: &'a Theme
}

impl <'a> WidgetDrawParams<'a> {
	pub fn new(graphics: &'a mut Graphics, theme: &'a Theme) -> WidgetDrawParams<'a> {
		WidgetDrawParams {
			graphics: graphics,
			theme: theme
		}
	}
	
	pub fn theme(&self) -> &Theme { self.theme }
	
	pub fn graphics(&mut self) -> &mut Graphics { self.graphics }
}
