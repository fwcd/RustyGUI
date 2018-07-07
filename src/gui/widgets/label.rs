use super::widget::Widget;
use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use utils::vec2i::Vec2i;
use gui::core::graphics::Graphics;
use gui::core::font_params::FontParams;
use gui::themes::theme::Theme;

pub struct Label {
	bounds: WidgetBounds,
	padding: Vec2i,
	font_params: FontParams,
	text: String
}

impl Label {
	pub fn new(text: &str, font_params: FontParams) -> Label {
		Label {
			bounds: WidgetBounds::empty(),
			padding: Vec2i::of(10, 10),
			font_params: font_params,
			text: text.to_string()
		}
	}
	
	pub fn of(text: &str, font_size: f32) -> Label {
		Label::new(text, FontParams::of_size(font_size))
	}
}

impl Widget for Label {
	fn render(&self, graphics: &mut Graphics, theme: &Theme) {
		graphics.draw_string(self.text.as_str(), self.top_left() + self.padding, self.font_params);
	}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size {
		graphics.string_size(self.text.as_str(), self.font_params) + (self.padding * 2)
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn internal_set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
}
