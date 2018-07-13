use super::widget::Widget;
use super::base::WidgetBase;
use super::bounds::WidgetBounds;
use super::widget_utils::widget_of;
use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::rect::Rectangle;
use utils::shared::{Shared, WeakShared};
use gui::core::mouse::{MouseClickEvent, MouseDragEvent};
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
	thumb_center: Vec2i,
	thumb_radius: u32,
	is_pressed: bool,
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
			thumb_center: Vec2i::of(0, 0),
			thumb_radius: 10,
			is_pressed: true,
			range: range,
			old_value: f32::INFINITY,
			value: value
		}
	}
	
	pub fn set_preferred_size(&mut self, size: Size) { self.preferred_size = size }
	
	fn value_to_pos(&self, value: f32) -> Vec2i {
		let min = *self.range.start();
		let max = *self.range.end();
		let normalized_value = (value - min) / (max - min);
		let padding_x = self.base.padding().x as i32;
		let track_length_x = self.preferred_size.width as i32 - (padding_x * 2);
		
		let rel_x = padding_x + (normalized_value * track_length_x as f32) as i32;
		let rel_y = self.bounds().height() as i32 / 2;
		
		self.top_left() + Vec2i::of(rel_x, rel_y)
	}
	
	fn pos_to_value(&self, pos: Vec2i) -> f32 {
		let top_left = self.top_left();
		let padding_x = self.base.padding().x;
		let min_x = top_left.x + padding_x;
		let max_x = (top_left.x + self.preferred_size.width as i32) - padding_x;
		let normalized_x = (pos.x - min_x) as f32 / (max_x - min_x) as f32;
		
		let min = *self.range.start();
		let max = *self.range.end();
		
		(normalized_x * (max - min)) + min
	}
	
	fn set_value_if_valid(&mut self, value: f32) {
		let min = *self.range.start();
		let max = *self.range.end();
		self.value = if value < min {
			min
		} else if value > max {
			max
		} else {
			value
		}
	}
}

impl Widget for Slider {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		let bounds = self.bounds().rect();
		
		// Draw background
		graphics.draw_rect(bounds, ShapeDrawParams::fill(theme.bg().translucent()));
		
		if self.old_value != self.value {
			// Thumb needs an update
			self.thumb_center = self.value_to_pos(self.value);
			self.old_value = self.value;
		}
		
		// Draw thumb
		graphics.draw_circle(self.thumb_center, self.thumb_radius, ShapeDrawParams::fill(theme.fg().translucent().with_alpha(180)));
	}
	
	fn preferred_size(&self, _graphics: &Graphics) -> Size { self.preferred_size }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		let value = self.pos_to_value(event.pos);
		self.set_value_if_valid(value);
		self.set_needs_relayout(true);
		self.is_pressed = true;
		true
	}
	
	fn handle_mouse_up(&mut self, event: MouseClickEvent) -> bool {
		self.is_pressed = false;
		true
	}
	
	fn handle_mouse_drag(&mut self, event: MouseDragEvent) -> bool {
		let value = self.pos_to_value(event.pos);
		self.set_value_if_valid(value);
		self.set_needs_relayout(true);
		true
	}
	
	fn name(&self) -> &str { "Slider" }
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
