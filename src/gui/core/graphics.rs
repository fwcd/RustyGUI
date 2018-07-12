use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::rect::Rectangle as APIRect;
use super::color::Color as APIColor;
use super::draw_params::ShapeDrawParams;
use super::font_params::FontParams;
use super::api_bridge::{sdl2_color_of, sdl2_rect_of};
use std::path::Path;

pub trait Graphics {
	fn clear(&mut self);
	
	fn set_color(&mut self, color: APIColor);
	
	fn draw_rect(&mut self, rectangle: APIRect, params: ShapeDrawParams);
	
	fn string_size(&self, text: &str, params: FontParams) -> Size;
	
	fn draw_oval_in(&mut self, rectangle: APIRect, params: ShapeDrawParams);
	
	fn draw_string(&mut self, text: &str, pos: Vec2i, params: FontParams);
}
