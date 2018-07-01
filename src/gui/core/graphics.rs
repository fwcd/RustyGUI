use sdl2::video::Window;
use sdl2::render::Canvas;
use utils::vec2i::Vec2i;
use utils::geometry::Rectangle;
use super::draw_params::ShapeDrawParams;
use super::font_params::FontParams;

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
	
	// TODO: Color?
	
	pub fn draw_rect(rectangle: Rectangle, params: ShapeDrawParams) {
		// TODO: Draw using SDL2
	}
	
	pub fn draw_oval_in(rectangle: Rectangle, params: ShapeDrawParams) {
		// TODO: Draw using SDL2
	}
	
	pub fn draw_text(text: &str, pos: Vec2i, params: FontParams) {
		// TODO: Draw using SDL2
	}
}
