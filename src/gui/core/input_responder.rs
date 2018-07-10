use super::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use super::keyboard::KeyEvent;

pub trait InputResponder {
	fn on_mouse_down(&mut self, event: MouseClickEvent) {}
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) {}
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) {}
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) {}
	
	fn on_key_down(&mut self, event: KeyEvent) {}
	
	fn on_key_up(&mut self, event: KeyEvent) {}
}
