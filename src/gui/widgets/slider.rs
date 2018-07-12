use super::widget::Widget;
use super::base::WidgetBase;
use utils::size::Size;
use utils::rect::Rectangle;
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;
use std::ops::RangeInclusive;

pub struct Slider {
	base: WidgetBase,
	preferred_size: Size,
	range: RangeInclusive<f32>,
	knob_size: Size
}

impl Slider {
	pub fn new(range: RangeInclusive<f32>) -> Slider {
		let preferred_size = Size::of(200, 50);
		Slider {
			base: WidgetBase::empty(),
			preferred_size: preferred_size,
			range: range,
			knob_size: Size::of(preferred_size.height, preferred_size.height)
		}
	}
	
	pub fn set_preferred_size(&mut self, size: Size) { self.preferred_size = size }
}

impl Widget for Slider {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		// Draw background
		graphics.draw_rect(self.bounds().rect(), ShapeDrawParams::fill(theme.bg().translucent()));
		
		// Draw knob
		let knob_bounds = Rectangle::of(self.top_left(), self.knob_size);
		
		graphics.draw_oval_in(knob_bounds, ShapeDrawParams::fill(theme.fg().strong()));
	}
	
	fn preferred_size(&self, _graphics: &Graphics) -> Size { self.preferred_size }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		self.base.set_needs_relayout(true);
		true
	}
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
