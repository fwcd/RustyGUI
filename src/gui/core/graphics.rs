use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::rect::Rectangle as APIRect;
use super::color::Color as APIColor;
use super::draw_params::ShapeDrawParams;
use super::font_params::FontParams;
use super::api_bridge::{sdl2_color_of, sdl2_rect_of};

/// A class wrapping the SDL2 graphics context to decouple
/// the application from the API.
pub struct Graphics {
	canvas: Canvas<Window>,
	ttf_context: Sdl2TtfContext
}

impl Graphics {
	pub fn from(canvas: Canvas<Window>, ttf_context: Sdl2TtfContext) -> Graphics {
		return Graphics {
			canvas: canvas,
			ttf_context: ttf_context
		};
	}
	
	pub fn clear(&mut self) {
		self.canvas.clear();
	}
	
	pub fn set_color(&mut self, color: APIColor) {
		self.canvas.set_draw_color(sdl2_color_of(color));
	}
	
	pub fn draw_rect(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		let sdl2_rect = sdl2_rect_of(rectangle);
		if params.filled() {
			let _ = self.canvas.fill_rect(sdl2_rect);
		}
		if params.outlined() {
			let _ = self.canvas.draw_rect(sdl2_rect);
		}
	}
	
	pub fn string_size(&self, text: &str, params: FontParams) -> Size {
		unimplemented!() // TODO
	}
	
	pub fn draw_oval_in(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		unimplemented!() // TODO
	}
	
	pub fn draw_string(&mut self, text: &str, pos: Vec2i, params: FontParams) {
		unimplemented!() // TODO
	}
	
	pub fn show(&mut self) {
		self.canvas.present();
	}
}
