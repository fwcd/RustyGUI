use super::widget::Widget;
use super::bounds::WidgetBounds;
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
		graphics.draw_rect(self.base.bounds.rect().shrink_centered_by(self.preferred_size.height as i32 / 4), ShapeDrawParams::fill());
	}
	
	fn preferred_size(&self, graphics: &Graphics) -> Size { self.preferred_size }
	
	fn bounds(&self) -> &WidgetBounds { &self.base.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) { self.base.bounds = bounds }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		self.base.needs_relayout = true;
		true
	}
	
	fn needs_relayout(&self) -> bool { self.base.needs_relayout }
}
