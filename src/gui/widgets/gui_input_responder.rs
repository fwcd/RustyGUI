use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use super::gui::WidgetGUI;

pub trait GUIInputResponder {
	fn on_mouse_down(&mut self, gui: &mut WidgetGUI, event: MouseClickEvent) -> bool { false }
	
	fn on_mouse_up(&mut self, gui: &mut WidgetGUI, event: MouseClickEvent) -> bool { false }
	
	fn on_mouse_move(&mut self, gui: &mut WidgetGUI, event: MouseMoveEvent) -> bool { false }
	
	fn on_mouse_drag(&mut self, gui: &mut WidgetGUI, event: MouseDragEvent) -> bool { false }
	
	fn on_key_down(&mut self, gui: &mut WidgetGUI, event: KeyEvent) -> bool { false }
	
	fn on_key_up(&mut self, gui: &mut WidgetGUI, event: KeyEvent) -> bool { false }
}
