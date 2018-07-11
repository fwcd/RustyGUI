use super::layout::Layout;
use gui::widgets::layouted_widget::LayoutedWidget;
use gui::widgets::empty::EmptyWidget;
use gui::widgets::bounds::WidgetBounds;
use gui::core::graphics::Graphics;
use utils::vec2i::Vec2i;
use utils::shared::Shared;
use std::cell::RefMut;
use std::cmp::min;

pub const TOP_POS = "top";
pub const LEFT_POS = "left";
pub const RIGHT_POS = "right";
pub const BOTTOM_POS = "bottom";
pub const CENTER_POS = "center";

pub struct BorderLayout;

impl BorderLayout {
	fn width_of(widget: RefMut<Widget>, max_width: u32) -> u32 {
		min(widget.bounds().width(), max_width)
	}
	
	fn height_of(widget: RefMut<Widget>, max_height: u32) -> u32 {
		min(widget.bounds().height(), max_height)
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
			match widget.layout_hint() {
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
		
		let top_height = top.map(|it| Self::height_of(it)).unwrap_or(0);
		let left_width = left.map(|it| Self::width_of(it)).unwrap_or(0);
		let right_width = right.map(|it| Self::width_of(it)).unwrap_or(0);
		let bottom_height = bottom.map(|it| Self::height_of(it)).unwrap_or(0);
		let side_height = (max_height - top_height) - bottom_height;
		let center_width = (max_width - left_width) - right_width;
		
		let top_bounds = WidgetBounds::new(top_left.x, top_left.y, max_width, top_height);
		let left_bounds = WidgetBounds::new(top_left.x, top_left.y + top_height, left_width, side_height);
		let right_bounds = WidgetBounds::new(bottom_right.x - right_width, top_left.y + top_height, right_width, side_height);
		let bottom_bounds = WidgetBounds::new(top_left.x, bottom_right.y - bottom_height, max_width, bottom_height);
		let center_bounds = WidgetBounds::new(top_left.x + left_width, top_left.y + top_height, center_width, side_height);
		
		if let Some(top_widget) = top { top_widget.set_bounds(top_bounds); }
		if let Some(left_widget) = left { left_widget.set_bounds(left_bounds); }
		if let Some(right_widget) = right { right_widget.set_bounds(right_bounds); }
		if let Some(bottom_widget) = bottom { bottom_widget.set_bounds(bottom_bounds); }
		if let Some(center_widget) = center { center_widget.set_bounds(center_bounds); }
	}
}
