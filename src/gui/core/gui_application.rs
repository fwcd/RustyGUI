use super::graphics::Graphics;
use super::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use super::keyboard::KeyEvent;

/// An application that uses a mouse/keyboard-based
/// graphical user interface.
trait GUIApplication {
	fn render(&self, graphics: &mut Graphics) {}
	
	fn on_mouse_down(&mut self, event: MouseClickEvent) {}
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) {}
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) {}
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) {}
	
	fn on_key_down(&mut self, event: KeyEvent) {}
	
	fn on_key_up(&mut self, event: KeyEvent) {}
}
