use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use super::gui::WidgetGUI;

pub trait GUIInputResponder {
	fn on_mouse_down(&mut self, _gui: &mut WidgetGUI, _event: MouseClickEvent) -> bool { false }
	
	fn on_mouse_up(&mut self, _gui: &mut WidgetGUI, _event: MouseClickEvent) -> bool { false }
	
	fn on_mouse_move(&mut self, _gui: &mut WidgetGUI, _event: MouseMoveEvent) -> bool { false }
	
	fn on_mouse_drag(&mut self, _gui: &mut WidgetGUI, _event: MouseDragEvent) -> bool { false }
	
	fn on_key_down(&mut self, _gui: &mut WidgetGUI, _event: KeyEvent) -> bool { false }
	
	fn on_key_up(&mut self, _gui: &mut WidgetGUI, _event: KeyEvent) -> bool { false }
}
