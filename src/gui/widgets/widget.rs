use super::bounds::WidgetBounds;
use super::gui::WidgetGUI;
use super::base::WidgetBase;
use super::gui_input_responder::GUIInputResponder;
use utils::size::Size;
use utils::vec2i::Vec2i;
use utils::shared::WeakShared;
use gui::core::graphics::Graphics;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::keyboard::KeyEvent;
use gui::themes::theme::Theme;
use std::cell::RefMut;

/// A GUI widget
pub trait Widget: GUIInputResponder {
	fn base(&self) -> &WidgetBase;
	
	fn base_mut(&mut self) -> &mut WidgetBase;
	
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme);
	
	fn preferred_size(&self, graphics: &Graphics) -> Size;
	
	fn name(&self) -> &str;
	
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
		self.set_bounds_deeply(WidgetBounds::of(new_bounding_rect));
	}
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn move_to(&mut self, new_top_left: Vec2i) {
		let delta = new_top_left - self.top_left();
		self.move_by(delta);
	}
	
	fn update_layout_deeply(&mut self, graphics: &Graphics) {
		self.for_each_child(&mut |mut it|
			it.update_layout_if_needed(graphics)
		);
		self.update_layout(graphics);
	}
	
	fn update_layout(&mut self, graphics: &Graphics) {
		let top_left = self.top_left();
		let size = self.preferred_size(graphics);
		self.set_bounds_deeply(WidgetBounds::from(top_left, size));
		self.set_needs_relayout(false);
		self.internal_on_update_layout(graphics);
	}
	
	/// This is NOT an API method and should only be
	/// implemented, not called from outside of this trait.
	fn internal_on_update_layout(&mut self, _graphics: &Graphics) {}
	
	fn update_layout_if_needed(&mut self, graphics: &Graphics) {
		if self.or_any_child_needs_relayout() {
			self.update_layout_deeply(graphics);
		}
	}
	
	fn or_any_child_needs_relayout(&mut self) -> bool {
		let mut child_needs_relayout = false;
		self.for_each_child(&mut |mut it|
			child_needs_relayout |= it.or_any_child_needs_relayout()
		);
		child_needs_relayout || self.needs_relayout()
	}
	
	fn for_each_child(&mut self, _each: &mut FnMut(RefMut<Widget>)) {}
	
	fn handle_mouse_down(&mut self, _event: MouseClickEvent) -> bool { false }
	
	fn handle_mouse_up(&mut self, _event: MouseClickEvent) -> bool { false }
	
	fn handle_mouse_move(&mut self, _event: MouseMoveEvent) -> bool { false }
	
	fn handle_mouse_drag(&mut self, _event: MouseDragEvent) -> bool { false }
	
	fn handle_key_down(&mut self, _event: KeyEvent) -> bool { false }
	
	fn handle_key_up(&mut self, _event: KeyEvent) -> bool { false }
	
	fn set_needs_relayout(&mut self, needs_relayout: bool) {
		if needs_relayout {
			trace!("{} requests a relayout", self.name());
		} else {
			trace!("{} handles a relayout", self.name());
		}
		self.base_mut().set_needs_relayout(needs_relayout);
	}
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn set_bounds_deeply(&mut self, bounds: WidgetBounds) {
		let delta = self.bounds().offset_to(&bounds);
		self.for_each_child(&mut |mut it|
			it.move_by(delta)
		);
		self.base_mut().set_bounds(bounds);
	}
	
	// Convenience methods
	
	fn needs_relayout(&self) -> bool { self.base().needs_relayout() }
	
	fn bounds(&self) -> &WidgetBounds { self.base().bounds() }
	
	fn gui(&self) -> WeakShared<WidgetGUI> { self.base().gui() }
	
	fn set_gui(&mut self, gui: WeakShared<WidgetGUI>) { self.base_mut().set_gui(gui) }
	
	fn this(&self) -> Option<WeakShared<Widget>> { self.base().this() }
	
	fn set_this(&mut self, this: WeakShared<Widget>) { self.base_mut().set_this(this) }
}

impl <W> GUIInputResponder for W where W: Widget {
	fn on_mouse_down(&mut self, gui: &mut WidgetGUI, event: MouseClickEvent) -> bool {
		let mut handled = false;
		self.for_each_child(&mut |mut it| {
			let contains_pos = it.bounds().rect().contains(event.pos);
			if contains_pos { handled |= it.on_mouse_down(gui, event); }
		});
		if handled { return true; }
		handled = self.handle_mouse_down(event);
		if handled {
			if let Some(this) = self.this() {
				gui.set_dragged(this);
			} else {
				debug!("Warning: The widget that handled the mouseDown event did not have a 'this' pointer, thus no drag lock could be set");
			}
		}
		handled
	}
	
	fn on_mouse_up(&mut self, gui: &mut WidgetGUI, event: MouseClickEvent) -> bool {
		let mut handled = false;
		self.for_each_child(&mut |mut it| {
			let contains_pos = it.bounds().rect().contains(event.pos);
			if contains_pos { handled |= it.on_mouse_up(gui, event); }
		});
		if handled { return true; }
		self.handle_mouse_up(event)
	}
	
	fn on_mouse_move(&mut self, gui: &mut WidgetGUI, event: MouseMoveEvent) -> bool {
		let mut handled = false;
		self.for_each_child(&mut |mut it| {
			let contains_pos = it.bounds().rect().contains(event.pos);
			if contains_pos { handled |= it.on_mouse_move(gui, event); }
		});
		if handled { return true; }
		self.handle_mouse_move(event)
	}
	
	fn on_mouse_drag(&mut self, gui: &mut WidgetGUI, event: MouseDragEvent) -> bool {
		let mut handled = false;
		self.for_each_child(&mut |mut it| {
			let contains_pos = it.bounds().rect().contains(event.pos);
			if contains_pos { handled |= it.on_mouse_drag(gui, event); }
		});
		if handled { return true; }
		self.handle_mouse_drag(event)
	}
	
	// TODO: Focus mechanism that checks whether a widget accepts keyboard input
	
	fn on_key_down(&mut self, gui: &mut WidgetGUI, event: KeyEvent) -> bool {
		let mut handled = false;
		self.for_each_child(&mut |mut it| {
			handled |= it.on_key_down(gui, event);
		});
		if handled { return true; }
		self.handle_key_down(event)
	}
	
	fn on_key_up(&mut self, gui: &mut WidgetGUI, event: KeyEvent) -> bool {
		let mut handled = false;
		self.for_each_child(&mut |mut it| {
			handled |= it.on_key_up(gui, event);
		});
		if handled { return true; }
		self.handle_key_up(event)
	}
}
