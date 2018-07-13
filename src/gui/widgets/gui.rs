use super::widget::Widget;
use super::bounds::WidgetBounds;
use super::layouts::layout::Layout;
use super::container::Container;
use super::gui_input_responder::GUIInputResponder;
use gui::core::graphics::Graphics;
use gui::core::gui_application::GUIApplication;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::core::input_responder::InputResponder;
use gui::themes::theme::Theme;
use utils::shared::{share, Shared, WeakShared};
use utils::size::Size;
use std::rc::Rc;
use std::cell::{Ref, RefMut};

pub struct WidgetGUI {
	theme: Theme,
	root: Shared<Container>,
	this: WeakShared<WidgetGUI>,
	dragged: Option<WeakShared<Widget>>,
	current_global_widget_id: u32
}

impl WidgetGUI {
	pub fn new(width: u32, height: u32, base_layout: Box<Layout>) -> Shared<Self> {
		let root = share(Container::new(base_layout));
		{
			let mut root_ref = root.borrow_mut();
			root_ref.base_mut().set_bounds(WidgetBounds::new(0, 0, width, height));
			root_ref.set_preferred_size(Size::of(width, height));
			root_ref.set_has_background(false);
		}
		let instance = share(WidgetGUI {
			theme: Theme::light(),
			root: root,
			this: WeakShared::new(),
			dragged: None,
			current_global_widget_id: 0
		});
		{
			let mut instance_ref = instance.borrow_mut();
			let this = Rc::downgrade(&instance);
			instance_ref.root.borrow_mut().set_gui(this.clone());
			instance_ref.this = this;
		}
		instance
	}
	
	pub fn next_global_widget_id(&mut self) -> u32 {
		let id = self.current_global_widget_id;
		self.current_global_widget_id += 1;
		id
	}
	
	pub fn root(&self) -> Shared<Container> { self.root.clone() }
	
	pub fn borrow_root_mut(&self) -> RefMut<Container> { self.root.borrow_mut() }
	
	pub fn borrow_root(&self) -> Ref<Container> { self.root.borrow() }
	
	pub fn theme(&self) -> &Theme { &self.theme }
	
	pub fn set_theme(&mut self, theme: Theme) { self.theme = theme }
	
	pub fn dragged(&self) -> Option<WeakShared<Widget>> { self.dragged.as_ref().map(|it| it.clone()) }
	
	pub fn set_dragged(&mut self, dragged: WeakShared<Widget>) { self.dragged = Some(dragged) }
	
	pub fn render(&mut self, graphics: &mut Graphics) {
		graphics.clear(self.theme.bg().strong());
		let mut root = self.root.borrow_mut();
		root.update_layout_if_needed(graphics);
		root.render(graphics, &self.theme);
	}
}

impl InputResponder for WidgetGUI {
	fn on_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		let root = self.root.clone();
		let mut root_ref = root.borrow_mut();
		root_ref.on_mouse_down(self, event)
	}
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) -> bool {
		if let Some(weak_dragged) = self.dragged() {
			// Only call mouseDrag event on the dragged widget when present
			if let Some(dragged) = weak_dragged.upgrade() {
				dragged.borrow_mut().on_mouse_up(self, event);
			} else {
				debug!("Warning: Dragged widget is present in GUI, but it's weak pointer does not point anywhere.");
			}
		self.dragged = None;
		}
		
		let root = self.root.clone();
		let mut root_ref = root.borrow_mut();
		root_ref.on_mouse_up(self, event)
	}
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) -> bool {
		let root = self.root.clone();
		let mut root_ref = root.borrow_mut();
		root_ref.on_mouse_move(self, event)
	}
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) -> bool {
		if let Some(weak_dragged) = self.dragged() {
			// Only call mouseDrag event on the dragged widget when present
			if let Some(dragged) = weak_dragged.upgrade() {
				dragged.borrow_mut().on_mouse_drag(self, event);
			} else {
				debug!("Warning: Dragged widget is present in GUI, but it's weak pointer does not point anywhere.");
			}
		}
		
		let root = self.root.clone();
		let mut root_ref = root.borrow_mut();
		root_ref.on_mouse_drag(self, event)
	}
	
	fn on_key_down(&mut self, event: KeyEvent) -> bool {
		let root = self.root.clone();
		let mut root_ref = root.borrow_mut();
		root_ref.on_key_down(self, event)
	}
	
	fn on_key_up(&mut self, event: KeyEvent) -> bool {
		let root = self.root.clone();
		let mut root_ref = root.borrow_mut();
		root_ref.on_key_up(self, event)
	}
}
