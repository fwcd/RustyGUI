use gui::core::graphics::Graphics;
use gui::widgets::widget::Widget;
use gui::widgets::widget_bounds::WidgetBounds;
use super::layout::Layout;
use utils::vec2i::Vec2i;
use utils::shared::Shared;

pub struct BoxLayout {
	is_horizontal: bool,
	padding: i32,
	current_pos: Vec2i
}

impl BoxLayout {
	fn new(is_horizontal: bool, padding: i32) -> Self {
		BoxLayout {
			is_horizontal: is_horizontal,
			padding: padding,
			current_pos: Vec2i::of(padding, padding)
		}
	}
	
	pub fn horizontal() -> Self { Self::new(true, 10) }
	
	pub fn vertical() -> Self { Self::new(false, 10) }
}

impl Layout for BoxLayout {
	fn on_add_widget(&mut self, shared_widget: Shared<Widget>, layout_hint: &String, graphics: &Graphics) {
		let mut widget = shared_widget.borrow_mut();
		let size = widget.get_preferred_size(graphics);
		widget.set_bounds(WidgetBounds::from(self.current_pos, size));
		
		let delta: Vec2i;
		
		if self.is_horizontal {
			delta = Vec2i::of(size.width as i32 + self.padding, 0);
		} else {
			delta = Vec2i::of(0, size.height as i32 + self.padding);
		}
		
		self.current_pos = self.current_pos + delta;
	}
	
	fn on_remove_widget(&mut self, shared_widget: Shared<Widget>, layout_hint: &String, graphics: &Graphics) {
		// TODO
	}
}
