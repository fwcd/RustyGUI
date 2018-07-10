use super::widget::Widget;
use super::widget_bounds::WidgetBounds;
use super::layouts::layout::Layout;
use super::layouts::box_layout::BoxLayout;
use gui::core::graphics::Graphics;
use gui::themes::theme::Theme;
use utils::reduce::Reduce;
use utils::vec2i::Vec2i;
use utils::size::Size;
use utils::shared::Shared;
use std::collections::VecDeque;

#[derive(Clone)]
struct LayoutedWidgetRef {
	pub widget: Shared<Widget>,
	pub layout_hint: String,
	pub id: i32
}

pub struct Container {
	bounds: WidgetBounds,
	padding: Vec2i,
	childs: Vec<LayoutedWidgetRef>,
	layout: Box<Layout>,
	added_widgets: VecDeque<LayoutedWidgetRef>,
	removed_widgets: VecDeque<LayoutedWidgetRef>,
	current_id: i32
}

impl Container {
	pub fn new(layout: Box<Layout>) -> Self {
		Container {
			bounds: WidgetBounds::empty(),
			padding: Vec2i::of(10, 10),
			childs: Vec::new(),
			layout: layout,
			added_widgets: VecDeque::new(),
			removed_widgets: VecDeque::new(),
			current_id: 0
		}
	}
	
	pub fn hbox() -> Self { Self::new(Box::new(BoxLayout::horizontal())) }
	
	pub fn vbox() -> Self { Self::new(Box::new(BoxLayout::vertical())) }
	
	pub fn add(&mut self, child: Shared<Widget>) {
		self.insert(child, "");
	}
	
	pub fn insert(&mut self, child: Shared<Widget>, layout_hint: &str) {
		let current_id = self.current_id;
		self.insert_with_id(child, layout_hint, current_id);
		self.current_id += 1;
	}
	
	pub fn insert_with_id(&mut self, child: Shared<Widget>, layout_hint: &str, id: i32) {
		let boxed_layout_hint = layout_hint.to_string();
		let widget = LayoutedWidgetRef {
			widget: child,
			layout_hint: boxed_layout_hint,
			id: id
		};
		
		self.added_widgets.push_back(widget.clone());
		self.childs.push(widget);
	}
	
	pub fn remove_with_id(&mut self, id: i32) {
		let index = self.index_of_id(id).expect("Could not find index of the child widget");
		let removed = self.childs.remove(index);
		self.removed_widgets.push_back(removed);
	}
	
	fn index_of_id(&self, id: i32) -> Option<usize> {
		let mut index: usize = 0;
		for item in &self.childs {
			if item.id == id {
				return Some(index);
			}
			index += 1;
		}
		return None;
	}
}

impl Widget for Container {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		// Layout added widgets
		while !self.added_widgets.is_empty() {
			let added = self.added_widgets.pop_front().unwrap();
			self.layout.on_add_widget(added.widget, &added.layout_hint, graphics)
		}
		
		// Layout based on removed widgets
		while !self.removed_widgets.is_empty() {
			let removed = self.removed_widgets.pop_front().unwrap();
			self.layout.on_remove_widget(removed.widget, &removed.layout_hint, graphics)
		}
		
		// Draw child widgets
		for child in &self.childs {
			child.widget.borrow_mut().render(graphics, theme);
		}
	}
	
	fn get_preferred_size(&self, _graphics: &Graphics) -> Size {
		self.childs.iter()
			.map(|child| child.widget.borrow().bounds().rect())
			.reduce(|a, b| a.merge(b))
			.map(|rect| rect.size())
			.unwrap_or(Size::of(0, 0))
			+ (self.padding * 2)
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) {
		let delta = self.bounds.offset_to(&bounds);
		
		for child in &self.childs {
			child.widget.borrow_mut().move_by(delta);
		}
		
		self.bounds = bounds;
	}
}
