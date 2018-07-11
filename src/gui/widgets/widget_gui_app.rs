use gui::core::graphics::Graphics;
use gui::core::gui_application::GUIApplication;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::core::input_responder::InputResponder;
use gui::themes::theme::Theme;
use utils::size::Size;
use super::widget::Widget;
use super::bounds::WidgetBounds;
use super::layouts::layout::Layout;
use super::container::Container;

pub struct WidgetGUIApp {
	title: String,
	width: u32,
	height: u32,
	theme: Theme,
	root: Container
}

impl WidgetGUIApp {
	pub fn new(title: &str, width: u32, height: u32, base_layout: Box<Layout>) -> WidgetGUIApp {
		let mut root = Container::new(base_layout);
		root.set_bounds(WidgetBounds::new(0, 0, width, height));
		root.set_preferred_size(Size::of(width, height));
		root.set_has_background(false);
		WidgetGUIApp {
			title: title.to_string(),
			width: width,
			height: height,
			theme: Theme::light(),
			root: root
		}
	}
	
	pub fn theme(&self) -> &Theme { &self.theme }
	
	pub fn set_theme(&mut self, theme: Theme) { self.theme = theme }
}

impl GUIApplication for WidgetGUIApp {
	fn root(&mut self) -> &mut Container { &mut self.root }
	
	fn render(&mut self, graphics: &mut Graphics) {
		graphics.set_color(self.theme.bg().strong());
		graphics.clear();
		self.root.update_layout_if_needed(graphics);
		self.root.render(graphics, &self.theme);
	}
	
	fn title(&self) -> String { self.title.to_string() }
	
	fn width(&self) -> u32 { self.width }
	
	fn height(&self) -> u32 { self.height }
}

impl InputResponder for WidgetGUIApp {
	fn on_mouse_down(&mut self, event: MouseClickEvent) -> bool { self.root.on_mouse_down(event) }
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) -> bool { self.root.on_mouse_up(event) }
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) -> bool { false }
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) -> bool { false }
	
	fn on_key_down(&mut self, event: KeyEvent) -> bool { false }
	
	fn on_key_up(&mut self, event: KeyEvent) -> bool { false }
}
