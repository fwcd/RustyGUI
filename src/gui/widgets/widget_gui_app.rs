use gui::core::graphics::Graphics;
use gui::core::gui_application::GUIApplication;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::core::input_responder::InputResponder;
use utils::shared::Shared;
use std::cell::{Ref, RefMut};
use super::layouts::layout::Layout;
use super::gui::WidgetGUI;

pub struct WidgetGUIApp {
	title: String,
	width: u32,
	height: u32,
	gui: Shared<WidgetGUI>
}

impl WidgetGUIApp {
	pub fn new<L>(title: &str, width: u32, height: u32, base_layout: L) -> Self where L: Layout + 'static {
		WidgetGUIApp {
			title: title.to_string(),
			width: width,
			height: height,
			gui: WidgetGUI::new(width, height, base_layout)
		}
	}
	
	pub fn borrow_gui(&self) -> Ref<WidgetGUI> { self.gui.borrow() }
	
	pub fn borrow_gui_mut(&self) -> RefMut<WidgetGUI> { self.gui.borrow_mut() }
}

impl GUIApplication for WidgetGUIApp {
	fn render(&mut self, graphics: &mut Graphics) { self.gui.borrow_mut().render(graphics) }
	
	fn title(&self) -> String { self.title.to_string() }
	
	fn width(&self) -> u32 { self.width }
	
	fn height(&self) -> u32 { self.height }
}

impl InputResponder for WidgetGUIApp {
	fn on_mouse_down(&mut self, event: MouseClickEvent) -> bool { self.gui.borrow_mut().on_mouse_down(event) }
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) -> bool { self.gui.borrow_mut().on_mouse_up(event) }
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) -> bool { self.gui.borrow_mut().on_mouse_move(event) }
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) -> bool { self.gui.borrow_mut().on_mouse_drag(event) }
	
	fn on_key_down(&mut self, event: KeyEvent) -> bool { self.gui.borrow_mut().on_key_down(event) }
	
	fn on_key_up(&mut self, event: KeyEvent) -> bool { self.gui.borrow_mut().on_key_up(event) }
}
