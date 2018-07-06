use super::widget::Widget;
use super::widget_params::WidgetDrawParams;
use super::widget_bounds::WidgetBounds;
use super::layouts::layout::Layout;
use super::layouts::box_layout::BoxLayout;
use gui::core::graphics::Graphics;
use utils::reduce::Reduce;
use utils::vec2i::Vec2i;
use utils::size::Size;

struct LayoutedWidget {
	pub widget: Box<Widget>,
	pub layout_hint: String,
	pub id: i32
}

pub struct Container {
	bounds: WidgetBounds,
	padding: Vec2i,
	childs: Vec<LayoutedWidget>,
	layout: Box<Layout>,
	current_id: i32
}

impl Container {
	pub fn new(layout: Box<Layout>) -> Container {
		Container {
			bounds: WidgetBounds::empty(),
			padding: Vec2i::of(10, 10),
			childs: Vec::new(),
			layout: layout,
			current_id: 0
		}
	}
	
	pub fn hbox() -> Container { Container::new(Box::new(BoxLayout::horizontal())) }
	
	pub fn vbox() -> Container { Container::new(Box::new(BoxLayout::vertical())) }
	
	pub fn add(&mut self, child: Box<Widget>) {
		self.insert(child, "");
	}
	
	pub fn insert(&mut self, child: Box<Widget>, layout_hint: &str) {
		let current_id = self.current_id;
		self.insert_with_id(child, layout_hint, current_id);
		self.current_id += 1;
	}
	
	pub fn insert_with_id(&mut self, child: Box<Widget>, layout_hint: &str, id: i32) {
		self.childs.push(LayoutedWidget {
			widget: child,
			layout_hint: layout_hint.to_string(),
			id: id
		});
	}
	
	pub fn remove_with_id(&mut self, id: i32) {
		let index = self.index_of_id(id).expect("Could not find index of the child widget");
		self.childs.remove(index);
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
	fn render(&self, params: &mut WidgetDrawParams) {
		for child in &self.childs {
			child.widget.render(params);
		}
	}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size {
		self.childs.iter()
			.map(|child| child.widget.bounds().rect())
			.reduce(|a, b| a.merge(b))
			.map(|rect| rect.size())
			.unwrap_or(Size::of(0, 0))
			+ (self.padding * 2)
	}
	
	fn internal_on_move_by(&mut self, delta: Vec2i) {
		for child in &mut self.childs {
			child.widget.move_by(delta);
		}
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn internal_set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
}
