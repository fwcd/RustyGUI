use super::layout::Layout;
use gui::core::graphics::Graphics;
use gui::widgets::layouted_widget::LayoutedWidget;
use gui::widgets::bounds::WidgetBounds;
use utils::vec2i::Vec2i;

pub struct BoxLayout {
	is_horizontal: bool,
	padding: i32
}

impl BoxLayout {
	fn new(is_horizontal: bool, padding: i32) -> Self {
		BoxLayout {
			is_horizontal: is_horizontal,
			padding: padding
		}
	}
	
	pub fn horizontal() -> Self { Self::new(true, 10) }
	
	pub fn vertical() -> Self { Self::new(false, 10) }
}

impl Layout for BoxLayout {
	fn arrange(&self, widgets: &mut Vec<LayoutedWidget>, parent_bounds: &WidgetBounds, graphics: &Graphics) {
		let top_left = parent_bounds.top_left();
		let mut draw_pos = top_left + Vec2i::of(self.padding, self.padding);
		
		for boxed_widget in widgets {
			let mut widget = boxed_widget.borrow_mut();
			let size = widget.preferred_size(graphics);
			widget.set_bounds(WidgetBounds::from(draw_pos, size));
			
			let delta: Vec2i;
			
			if self.is_horizontal {
				delta = Vec2i::of(size.width as i32 + self.padding, 0);
			} else {
				delta = Vec2i::of(0, size.height as i32 + self.padding);
			}
			
			draw_pos = draw_pos + delta;
		}
	}
}
