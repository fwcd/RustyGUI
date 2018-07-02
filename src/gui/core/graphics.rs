use sdl2::video::Window;
use sdl2::render::Canvas;
use utils::vec2i::Vec2i;
use utils::geometry::Rectangle as APIRect;
use super::color::Color as APIColor;
use super::draw_params::ShapeDrawParams;
use super::font_params::FontParams;
use super::api_bridge::{sdl2_color_of, sdl2_rect_of};

/// A class wrapping the SDL2 graphics context to decouple
/// the application from the API.
pub struct Graphics {
	canvas: Canvas<Window>
}

impl Graphics {
	pub fn from(canvas: Canvas<Window>) -> Graphics {
		return Graphics {
			canvas: canvas
		};
	}
	
	pub fn set_color(&mut self, color: APIColor) {
		self.canvas.set_draw_color(sdl2_color_of(color));
	}
	
	pub fn draw_rect(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		self.canvas.draw_rect(sdl2_rect_of(rectangle));
	}
	
	pub fn draw_oval_in(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		// TODO: Draw using SDL2
	}
	
	pub fn draw_text(&mut self, text: &str, pos: Vec2i, params: FontParams) {
		// TODO: Draw using SDL2
	}
}
