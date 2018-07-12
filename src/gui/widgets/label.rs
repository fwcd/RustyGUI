use super::widget::Widget;
use super::base::WidgetBase;
use utils::size::Size;
use gui::core::graphics::Graphics;
use gui::core::font_params::FontParams;
use gui::themes::theme::Theme;

pub struct Label {
	base: WidgetBase,
	font_params: FontParams,
	text: String
}

impl Label {
	pub fn new(text: &str, font_params: FontParams) -> Label {
		Label {
			base: WidgetBase::empty(),
			font_params: font_params,
			text: text.to_string()
		}
	}
	
	pub fn of(text: &str, font_size: u16) -> Label {
		Label::new(text, FontParams::of_size(font_size))
	}
	
	pub fn text(&self) -> &str { self.text.as_str() }
	
	pub fn set_text(&mut self, text: &str) { self.text = text.to_string() }
}

impl Widget for Label {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		graphics.set_color(theme.fg().strong());
		graphics.draw_string(self.text.as_str(), self.top_left(), self.font_params);
	}
	
	fn preferred_size(&self, graphics: &Graphics) -> Size {
		graphics.string_size(self.text.as_str(), self.font_params)
	}
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
