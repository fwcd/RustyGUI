use super::graphics::Graphics;
use super::input_responder::InputResponder;
use gui::widgets::container::Container;

/// An application that uses a mouse/keyboard-based
/// graphical user interface.
pub trait GUIApplication: InputResponder {
	fn title(&self) -> String;
	
	fn width(&self) -> u32;
	
	fn height(&self) -> u32;
	
	fn root(&mut self) -> &mut Container;
	
	fn render(&mut self, graphics: &mut Graphics) {}
}
