use super::widget::Widget;
use super::base::WidgetBase;
use super::bounds::WidgetBounds;
use utils::cast::Castable;
use utils::size::Size;
use utils::rect::Rectangle;
use utils::vec2i::Vec2i;
use utils::shared::{share, Shared, shared_to_mut};
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;
use std::ops::RangeInclusive;
use std::cell::RefMut;
use std::f32;

pub struct Slider {
	// view
	base: WidgetBase,
	preferred_size: Size,
	thumb: Shared<SliderThumb>,
	// model
	range: RangeInclusive<f32>,
	old_value: f32,
	value: f32
}

impl Slider {
	pub fn new(range: RangeInclusive<f32>) -> Self {
		let value = *range.start();
		Self {
			base: WidgetBase::empty(),
			preferred_size: Size::of(200, 30),
			thumb: share(SliderThumb::of_radius(10)),
			range: range,
			old_value: f32::INFINITY,
			value: value
		}
	}
	
	pub fn set_preferred_size(&mut self, size: Size) { self.preferred_size = size }
	
	fn compute_thumb_center(&self) -> Vec2i {
		let min = *self.range.start();
		let max = *self.range.end();
		let normalized_value = (self.value - min) / (max - min);
		let padding_x = self.base.padding().x as i32;
		let track_length_x = self.preferred_size.width as i32 - (padding_x * 2);
		
		let rel_x = padding_x + (normalized_value * track_length_x as f32) as i32;
		let rel_y = self.bounds().height() as i32;
		
		self.top_left() + Vec2i::of(rel_x, rel_y)
	}
}

impl Widget for Slider {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		let bounds = self.bounds().rect();
		
		// Draw background
		graphics.draw_rect(bounds, ShapeDrawParams::fill(theme.bg().translucent()));
		
		// Draw thumb
		let mut thumb = self.thumb.borrow_mut();
		
		if self.old_value != self.value {
			// Thumb needs an update
			let thumb_offset = Vec2i::of(self.base.padding().x, bounds.height() as i32 / 2);
			let thumb_center = self.top_left() + thumb_offset;
			
			thumb.set_center(thumb_center);
			self.old_value = self.value;
		}
		
		thumb.render(graphics, theme);
	}
	
	fn preferred_size(&self, _graphics: &Graphics) -> Size { self.preferred_size }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		self.set_needs_relayout(true);
		true
	}
	
	fn name(&self) -> &str { "Slider" }
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
	
	fn for_each_child(&mut self, each: &mut FnMut(RefMut<Widget>)) {
		each(self.thumb.borrow_mut());
	}
}

struct SliderThumb {
	base: WidgetBase,
	radius: u32,
	is_pressed: bool
}

impl SliderThumb {
	pub fn of_radius(radius: u32) -> Self {
		let diameter = radius * 2;
		Self {
			base: WidgetBase::new(WidgetBounds::new(0, 0, diameter, diameter)),
			radius: radius,
			is_pressed: false
		}
	}
	
	pub fn set_center(&mut self, pos: Vec2i) {
		let delta = pos - self.base.bounds().rect().center();
		self.move_by(delta);
	}
}

impl Widget for SliderThumb {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		let mut color = theme.fg().translucent().with_alpha(180);
		
		if self.is_pressed {
			color = color.with_half_alpha();
		}
		
		graphics.draw_oval_in(self.bounds().rect(), ShapeDrawParams::fill(color));
	}
	
	fn name(&self) -> &str { "SliderThumb" }
	
	fn preferred_size(&self, _graphics: &Graphics) -> Size {
		let diameter = self.radius * 2;
		Size::of(diameter, diameter)
	}
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		self.set_needs_relayout(true);
		self.is_pressed = true;
		true
	}
	
	fn handle_mouse_up(&mut self, event: MouseClickEvent) -> bool {
		self.is_pressed = false;
		true
	}
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
