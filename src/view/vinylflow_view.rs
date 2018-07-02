use gui::core::gui_application::GUIApplication;
use gui::core::graphics::Graphics;
use gui::core::color::Color;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::core::draw_params::ShapeDrawParams;
use utils::geometry::Rectangle;
use utils::vec2i::Vec2i;

pub struct VinylFlowView {
	mouse_pressed: bool
}

impl VinylFlowView {
	pub fn new() -> VinylFlowView {
		VinylFlowView { mouse_pressed: false }
	}
}

impl GUIApplication for VinylFlowView {
	fn title(&self) -> String { "VinylFlow".to_string() }
	
	fn width(&self) -> u32 { 640 }
	
	fn height(&self) -> u32 { 480 }
	
	fn render(&self, graphics: &mut Graphics) {
		graphics.set_color(Color::black());
		graphics.clear();
		graphics.set_color(Color::green());
		let params: ShapeDrawParams;
		if (self.mouse_pressed) {
			params = ShapeDrawParams::fill();
		} else {
			params = ShapeDrawParams::outline();
		}
		graphics.draw_rect(Rectangle::at(Vec2i::of(100, 100), 200, 200), params);
	}
	
	fn on_mouse_down(&mut self, event: MouseClickEvent) {
		self.mouse_pressed = true;
	}
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) {
		self.mouse_pressed = false;
	}
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) {}
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) {}
	
	fn on_key_down(&mut self, event: KeyEvent) {}
	
	fn on_key_up(&mut self, event: KeyEvent) {}
}
