use sdl2::pixels::Color as SDL2Color;
use sdl2::mouse::MouseButton as SDL2MouseButton;
use sdl2::rect::Rect as SDL2Rect;
use sdl2::gfx::primitives::ToColor;

use gui::core::color::Color as APIColor;
use gui::core::mouse::MouseButton as APIMouseButton;
use utils::rect::Rectangle as APIRect;

// Conversion methods

pub fn sdl2_color_of(color: APIColor) -> SDL2Color {
	SDL2Color::RGBA(color.red, color.green, color.blue, color.alpha)
}

pub fn api_color_of(color: SDL2Color) -> APIColor {
	APIColor::rgba(color.r, color.g, color.b, color.a)
}

pub fn sdl2_rect_of(rect: APIRect) -> SDL2Rect {
	SDL2Rect::new(rect.top_left().x, rect.top_left().y, rect.width(), rect.height())
}

pub fn api_rect_of(rect: SDL2Rect) -> APIRect {
	APIRect::new(rect.x(), rect.y(), rect.width(), rect.height())
}

pub fn sdl2_mouse_button_of(button: APIMouseButton) -> SDL2MouseButton {
	match button {
		APIMouseButton::Left => SDL2MouseButton::Left,
		APIMouseButton::Middle => SDL2MouseButton::Middle,
		APIMouseButton::Right => SDL2MouseButton::Right,
		_ => SDL2MouseButton::Unknown
	}
}

pub fn api_mouse_button_of(button: SDL2MouseButton) -> APIMouseButton {
	match button {
		SDL2MouseButton::Left => APIMouseButton::Left,
		SDL2MouseButton::Middle => APIMouseButton::Middle,
		SDL2MouseButton::Right => APIMouseButton::Right,
		_ => APIMouseButton::Other
	}
}

impl ToColor for APIColor {
	fn as_rgba(&self) -> (u8, u8, u8, u8) {
		(self.red, self.green, self.blue, self.alpha)
	}
}
