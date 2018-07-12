use super::bounds::WidgetBounds;
use super::gui::WidgetGUI;
use super::base::WidgetBase;
use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::shared::{Shared, WeakShared};
use gui::core::graphics::Graphics;
use gui::core::input_responder::InputResponder;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::themes::theme::Theme;

/// A GUI widget
pub trait Widget: InputResponder {
	fn base(&self) -> &WidgetBase;
	
	fn base_mut(&mut self) -> &mut WidgetBase;
	
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme);
	
	fn preferred_size(&self, graphics: &Graphics) -> Size;
	
	fn preferred_bounds(&self, graphics: &Graphics) -> WidgetBounds {
		WidgetBounds::from(self.top_left(), self.preferred_size(graphics))
	}
	
	fn top_left(&self) -> Vec2i {
		self.bounds().rect().top_left()
	}
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn move_by(&mut self, delta: Vec2i) {
		let new_bounding_rect = self.bounds().rect().moved_by(delta);
		self.set_bounds(WidgetBounds::of(new_bounding_rect));
	}
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn move_to(&mut self, new_top_left: Vec2i) {
		let delta = new_top_left - self.top_left();
		self.move_by(delta);
	}
	
	fn update_layout(&mut self, graphics: &Graphics) {
		let top_left = self.top_left();
		let size = self.preferred_size(graphics);
		self.set_bounds(WidgetBounds::from(top_left, size));
	}
	
	fn update_layout_if_needed(&mut self, graphics: &Graphics) {
		if self.or_any_child_needs_relayout() {
			self.update_layout(graphics);
		}
	}
	
	fn or_any_child_needs_relayout(&self) -> bool {
		for child in self.childs() {
			if child.borrow().or_any_child_needs_relayout() { return true; }
		}
		self.needs_relayout()
	}
	
	fn childs(&self) -> Vec<Shared<Widget>> { Vec::new() }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool { false }
	
	fn handle_mouse_up(&mut self, event: MouseClickEvent) -> bool { false }
	
	fn handle_mouse_move(&mut self, event: MouseMoveEvent) -> bool { false }
	
	fn handle_mouse_drag(&mut self, event: MouseDragEvent) -> bool { false }
	
	fn handle_key_down(&mut self, event: KeyEvent) -> bool { false }
	
	fn handle_key_up(&mut self, event: KeyEvent) -> bool { false }
	
	// Convenience methods
	
	fn needs_relayout(&self) -> bool { self.base().needs_relayout() }
	
	fn bounds(&self) -> &WidgetBounds { self.base().bounds() }
	
	fn gui(&self) -> WeakShared<WidgetGUI> { self.base().gui() }
	
	fn set_gui(&mut self, gui: WeakShared<WidgetGUI>) { self.base_mut().set_gui(gui) }
	
	fn this(&self) -> WeakShared<Widget> { self.base().this() }
	
	fn set_this(&mut self, this: WeakShared<Widget>) { self.base_mut().set_this(this) }
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn set_bounds(&mut self, bounds: WidgetBounds) { self.base_mut().set_bounds(bounds) }
}

impl <W> InputResponder for W where W: Widget {
	fn on_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		for child in self.childs() {
			let mut borrowed_child = child.borrow_mut();
			let contains_pos = borrowed_child.bounds().rect().contains(event.pos);
			if contains_pos && borrowed_child.on_mouse_down(event) { return true; }
		}
		self.handle_mouse_down(event)
	}
	
	fn on_mouse_up(&mut self, event: MouseClickEvent) -> bool {
		for child in self.childs() {
			let mut borrowed_child = child.borrow_mut();
			let contains_pos = borrowed_child.bounds().rect().contains(event.pos);
			if contains_pos && borrowed_child.on_mouse_up(event) { return true; }
		}
		self.handle_mouse_up(event)
	}
	
	fn on_mouse_move(&mut self, event: MouseMoveEvent) -> bool {
		for child in self.childs() {
			let mut borrowed_child = child.borrow_mut();
			let contains_pos = borrowed_child.bounds().rect().contains(event.pos);
			if contains_pos && borrowed_child.on_mouse_move(event) { return true; }
		}
		self.handle_mouse_move(event)
	}
	
	fn on_mouse_drag(&mut self, event: MouseDragEvent) -> bool {
		for child in self.childs() {
			let mut borrowed_child = child.borrow_mut();
			let contains_pos = borrowed_child.bounds().rect().contains(event.pos);
			if contains_pos && borrowed_child.on_mouse_drag(event) { return true; }
		}
		self.handle_mouse_drag(event)
	}
	
	fn on_key_down(&mut self, event: KeyEvent) -> bool {
		for child in self.childs() {
			if child.borrow_mut().on_key_down(event) { return true; }
		}
		self.handle_key_down(event)
	}
	
	fn on_key_up(&mut self, event: KeyEvent) -> bool {
		for child in self.childs() {
			if child.borrow_mut().on_key_up(event) { return true; }
		}
		self.handle_key_up(event)
	}
}
