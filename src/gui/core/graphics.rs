use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::rect::Rectangle as APIRect;
use super::color::Color as APIColor;
use super::draw_params::ShapeDrawParams;
use super::font_params::FontParams;

pub trait Graphics {
	fn clear(&mut self);
	
	fn set_color(&mut self, color: APIColor);
	
	fn draw_rect(&mut self, rectangle: APIRect, params: ShapeDrawParams);
	
	fn string_size(&self, text: &str, params: FontParams) -> Size;
	
	fn draw_oval(&mut self, center: Vec2i, radius_x: u32, radius_y: u32, params: ShapeDrawParams);
	
	fn draw_string(&mut self, text: &str, pos: Vec2i, params: FontParams);
	
	fn draw_circle(&mut self, center: Vec2i, radius: u32, params: ShapeDrawParams) {
		self.draw_oval(center, radius, radius, params)
	}
	
	fn draw_oval_in(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		self.draw_oval(rectangle.center(), rectangle.width() / 2, rectangle.height() / 2, params)
	}
}
