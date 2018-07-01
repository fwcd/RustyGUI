use sdl2::video::Window;
use sdl2::render::Canvas;

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
}
