use super::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use super::keyboard::KeyEvent;

pub trait InputResponder {
	fn on_mouse_down(&mut self, event: MouseClickEvent) -> bool { false }
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) -> bool { false }
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) -> bool { false }
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) -> bool { false }
	
	fn on_key_down(&mut self, event: KeyEvent) -> bool { false }
	
	fn on_key_up(&mut self, event: KeyEvent) -> bool { false }
}
