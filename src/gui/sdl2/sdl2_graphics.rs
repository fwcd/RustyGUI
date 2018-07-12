use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, Texture, TextureQuery, BlendMode};
use sdl2::ttf;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::rect::Rect as SDL2Rect;
use sdl2::pixels::Color as SDL2Color;
use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::rect::Rectangle as APIRect;
use gui::core::color::Color as APIColor;
use gui::core::draw_params::ShapeDrawParams;
use gui::core::font_params::FontParams;
use gui::core::api_bridge::{sdl2_color_of, sdl2_rect_of};
use gui::core::graphics::Graphics;
use std::path::Path;

/// A class wrapping the SDL2 graphics context to decouple
/// the application from the API.
pub struct SDL2Graphics<'g> {
	canvas: Canvas<Window>,
	ttf: Sdl2TtfContext,
	font_path: &'g Path
}

impl <'g> SDL2Graphics<'g> {
	pub fn from(canvas: Canvas<Window>, ttf: Sdl2TtfContext, font_path: &'g Path) -> Self {
		let mut canvas_mut = canvas;
		canvas_mut.set_blend_mode(BlendMode::Blend);
		SDL2Graphics {
			canvas: canvas_mut,
			ttf: ttf,
			font_path: font_path
		}
	}
	
	pub fn show(&mut self) {
		self.canvas.present();
	}
	
	fn texture_size(texture: &Texture) -> Size {
		let TextureQuery { width, height, .. } = texture.query();
		Size::of(width, height)
	}
	
	fn string_to_texture(
		ttf: &Sdl2TtfContext,
		font_path: &Path,
		texture_creator: &'g TextureCreator<WindowContext>,
		text: &str,
		params: FontParams,
		color: SDL2Color
	) -> Texture<'g> {
		let mut font = ttf.load_font(font_path, params.font_size())
				.expect("Error while loading font");
		Self::update_font_style(&mut font, params);
		let surface = font.render(text)
				.blended(color)
				.expect("Error while rendering text to surface");
		texture_creator.create_texture_from_surface(&surface)
				.expect("Error while creating font texture from surface")
	}
	
	fn update_font_style(font: &mut Font, params: FontParams) {
		let mut style = ttf::STYLE_NORMAL;
		if params.is_bold() { style |= ttf::STYLE_BOLD; }
		if params.is_italic() { style |= ttf::STYLE_ITALIC; }
		if params.is_underlined() { style |= ttf::STYLE_UNDERLINE; }
		font.set_style(style);
	}
}

impl <'g> Graphics for SDL2Graphics<'g> {
	fn clear(&mut self) {
		self.canvas.clear();
	}
	
	fn set_color(&mut self, color: APIColor) {
		self.canvas.set_draw_color(sdl2_color_of(color));
	}
	
	fn draw_rect(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		let sdl2_rect = sdl2_rect_of(rectangle);
		if params.filled() {
			let _ = self.canvas.fill_rect(sdl2_rect);
		}
		if params.outlined() {
			let _ = self.canvas.draw_rect(sdl2_rect);
		}
	}
	
	fn string_size(&self, text: &str, params: FontParams) -> Size {
		let texture_creator = self.canvas.texture_creator();
		let texture = Self::string_to_texture(&self.ttf, self.font_path, &texture_creator, text, params, self.canvas.draw_color());
		Self::texture_size(&texture)
	}
	
	fn draw_oval_in(&mut self, rectangle: APIRect, params: ShapeDrawParams) {
		// TODO
	}
	
	fn draw_string(&mut self, text: &str, pos: Vec2i, params: FontParams) {
		let texture_creator = self.canvas.texture_creator();
		let texture = Self::string_to_texture(&self.ttf, self.font_path, &texture_creator, text, params, self.canvas.draw_color());
		let size = Self::texture_size(&texture);
		let bounds = SDL2Rect::new(pos.x, pos.y, size.width, size.height);
		self.canvas.copy(&texture, None, Some(bounds))
				.expect("Error while copying font texture to canvas");
	}
}
