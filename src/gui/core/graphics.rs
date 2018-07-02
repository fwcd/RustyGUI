use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use utils::vec2i::Vec2i;
use utils::geometry::Rectangle;
use super::draw_params::ShapeDrawParams;
use super::font_params::FontParams;

type SDL2Color = Color;
type GraphicsColor = super::color::Color;

// Conversion methods

fn sdl2_color_of(color: GraphicsColor) -> SDL2Color {
	return SDL2Color::RGBA(color.red, color.green, color.blue, color.alpha);
}

fn graphics_color_of(color: SDL2Color) -> GraphicsColor {
	return GraphicsColor::rgba(color.r, color.g, color.b, color.a);
}

fn sdl2_rect_of(rect: Rectangle) -> Rect {
	return Rect::new(rect.top_left.x, rect.top_left.y, rect.width, rect.height);
}

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
	
	pub fn set_color(&mut self, color: super::color::Color) {
		self.canvas.set_draw_color(sdl2_color_of(color));
	}
	
	pub fn draw_rect(&mut self, rectangle: Rectangle, params: ShapeDrawParams) {
		self.canvas.draw_rect(sdl2_rect_of(rectangle));
	}
	
	pub fn draw_oval_in(&mut self, rectangle: Rectangle, params: ShapeDrawParams) {
		// TODO: Draw using SDL2
	}
	
	pub fn draw_text(&mut self, text: &str, pos: Vec2i, params: FontParams) {
		// TODO: Draw using SDL2
	}
}
