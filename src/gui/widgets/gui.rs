use gui::core::graphics::Graphics;
use gui::core::gui_application::GUIApplication;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::core::input_responder::InputResponder;
use gui::themes::theme::Theme;
use utils::shared::{share, Shared, WeakShared};
use utils::size::Size;
use std::rc::Rc;
use super::widget::Widget;
use super::bounds::WidgetBounds;
use super::layouts::layout::Layout;
use super::container::Container;

pub struct WidgetGUI {
	theme: Theme,
	root: Container,
	this: WeakShared<WidgetGUI>
}

impl WidgetGUI {
	pub fn new(width: u32, height: u32, base_layout: Box<Layout>) -> Shared<Self> {
		let mut root = Container::new(base_layout);
		root.set_bounds(WidgetBounds::new(0, 0, width, height));
		root.set_preferred_size(Size::of(width, height));
		root.set_has_background(false);
		let instance = share(WidgetGUI {
			theme: Theme::light(),
			root: root,
			this: WeakShared::new()
		});
		{
			let mut instance_ref = instance.borrow_mut();
			let this = Rc::downgrade(&instance);
			instance_ref.root.set_gui(this.clone());
			instance_ref.this = this;
		}
		instance
	}
	
	pub fn root(&mut self) -> &mut Container { &mut self.root }
	
	pub fn theme(&self) -> &Theme { &self.theme }
	
	pub fn set_theme(&mut self, theme: Theme) { self.theme = theme }
	
	pub fn render(&mut self, graphics: &mut Graphics) {
		graphics.set_color(self.theme.bg().strong());
		graphics.clear();
		self.root.update_layout_if_needed(graphics);
		self.root.render(graphics, &self.theme);
	}
}

impl InputResponder for WidgetGUI {
	fn on_mouse_down(&mut self, event: MouseClickEvent) -> bool { self.root.on_mouse_down(event) }
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) -> bool { self.root.on_mouse_up(event) }
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) -> bool { self.root.on_mouse_move(event) }
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) -> bool { self.root.on_mouse_drag(event) }
	
	fn on_key_down(&mut self, event: KeyEvent) -> bool { self.root.on_key_down(event) }
	
	fn on_key_up(&mut self, event: KeyEvent) -> bool { self.root.on_key_up(event) }
}
