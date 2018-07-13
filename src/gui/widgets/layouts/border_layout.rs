use super::layout::Layout;
use gui::widgets::layouted_widget::LayoutedWidget;
use gui::widgets::widget::Widget;
use gui::widgets::bounds::WidgetBounds;
use gui::core::graphics::Graphics;
use std::cell::RefMut;
use std::cmp::min;
use utils::math::clamp_to_unsigned;

pub const TOP_POS: &str = "top";
pub const LEFT_POS: &str = "left";
pub const RIGHT_POS: &str = "right";
pub const BOTTOM_POS: &str = "bottom";
pub const CENTER_POS: &str = "center";

pub struct BorderLayout;

impl BorderLayout {
	pub fn new() -> Self { BorderLayout }
	
	fn width_of(widget: &RefMut<Widget>, max_width: u32, graphics: &Graphics) -> u32 {
		min(widget.preferred_size(graphics).width, max_width)
	}
	
	fn height_of(widget: &RefMut<Widget>, max_height: u32, graphics: &Graphics) -> u32 {
		min(widget.preferred_size(graphics).height, max_height)
	}
}

impl Layout for BorderLayout {
	fn arrange(&self, widgets: &mut Vec<LayoutedWidget>, parent_bounds: &WidgetBounds, graphics: &Graphics) {
		let mut top: Option<RefMut<Widget>> = None;
		let mut left: Option<RefMut<Widget>> = None;
		let mut right: Option<RefMut<Widget>> = None;
		let mut bottom: Option<RefMut<Widget>> = None;
		let mut center: Option<RefMut<Widget>> = None;
		
		for widget in widgets {
			let borrowed = widget.borrow_mut();
			match widget.layout_hint().as_str() {
				TOP_POS => top = Some(borrowed),
				LEFT_POS => left = Some(borrowed),
				RIGHT_POS => right = Some(borrowed),
				BOTTOM_POS => bottom = Some(borrowed),
				CENTER_POS => center = Some(borrowed),
				_ => {}
			}
		}
		
		let top_left = parent_bounds.top_left();
		let bottom_right = parent_bounds.bottom_right();
		let max_width = parent_bounds.width();
		let max_height = parent_bounds.height();
		
		let top_height = top.as_ref().map(|it| Self::height_of(it, max_height, graphics)).unwrap_or(0);
		let left_width = left.as_ref().map(|it| Self::width_of(it, max_width, graphics)).unwrap_or(0);
		let right_width = right.as_ref().map(|it| Self::width_of(it, max_width, graphics)).unwrap_or(0);
		let bottom_height = bottom.as_ref().map(|it| Self::height_of(it, max_height, graphics)).unwrap_or(0);
		let side_height = clamp_to_unsigned((max_height as i32 - top_height as i32) - bottom_height as i32);
		let center_width = clamp_to_unsigned((max_width as i32 - left_width as i32) - right_width as i32);
		
		let top_bounds = WidgetBounds::new(top_left.x, top_left.y, max_width, top_height);
		let left_bounds = WidgetBounds::new(top_left.x, top_left.y + top_height as i32, left_width, side_height);
		let right_bounds = WidgetBounds::new(bottom_right.x - right_width as i32, top_left.y + top_height as i32, right_width, side_height);
		let bottom_bounds = WidgetBounds::new(top_left.x, bottom_right.y - bottom_height as i32, max_width, bottom_height);
		let center_bounds = WidgetBounds::new(top_left.x + left_width as i32, top_left.y + top_height as i32, center_width, side_height);
		
		if let Some(top_widget) = top.as_mut() { top_widget.set_bounds_deeply(top_bounds); }
		if let Some(left_widget) = left.as_mut() { left_widget.set_bounds_deeply(left_bounds); }
		if let Some(right_widget) = right.as_mut() { right_widget.set_bounds_deeply(right_bounds); }
		if let Some(bottom_widget) = bottom.as_mut() { bottom_widget.set_bounds_deeply(bottom_bounds); }
		if let Some(center_widget) = center.as_mut() { center_widget.set_bounds_deeply(center_bounds); }
	}
	
	fn uses_parent_padding(&self) -> bool { false }
}
