use super::widget::Widget;
use super::base::WidgetBase;
use utils::size::Size;
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;
use std::ops::RangeInclusive;

pub struct Slider {
	base: WidgetBase,
	preferred_size: Size,
	range: RangeInclusive<f32>
}

impl Slider {
	pub fn new(range: RangeInclusive<f32>) -> Slider {
		Slider {
			base: WidgetBase::empty(),
			preferred_size: Size::of(200, 50),
			range: range
		}
	}
	
	pub fn set_preferred_size(&mut self, size: Size) { self.preferred_size = size }
}

impl Widget for Slider {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		graphics.set_color(theme.bg().translucent());
		graphics.draw_rect(self.bounds().rect(), ShapeDrawParams::fill());
		graphics.set_color(theme.bg().strong());
		graphics.draw_rect(self.base.bounds().rect().shrink_centered_by(self.preferred_size.height as i32 / 4), ShapeDrawParams::fill());
	}
	
	fn preferred_size(&self, _graphics: &Graphics) -> Size { self.preferred_size }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		self.base.set_needs_relayout(true);
		true
	}
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
