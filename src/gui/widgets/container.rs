use super::widget::Widget;
use super::bounds::WidgetBounds;
use super::layouts::layout::Layout;
use super::layouts::box_layout::BoxLayout;
use super::layouted_widget::LayoutedWidget;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::core::input_responder::InputResponder;
use gui::themes::theme::Theme;
use utils::reduce::Reduce;
use utils::vec2i::Vec2i;
use utils::size::Size;
use utils::shared::Shared;

pub struct Container {
	bounds: WidgetBounds,
	padding: Vec2i,
	childs: Vec<LayoutedWidget>,
	layout: Box<Layout>,
	current_id: i32,
	has_background: bool,
	needs_relayout: bool
}

impl Container {
	pub fn new(layout: Box<Layout>) -> Self {
		Container {
			bounds: WidgetBounds::empty(),
			padding: Vec2i::of(10, 10),
			childs: Vec::new(),
			layout: layout,
			current_id: 0,
			has_background: true,
			needs_relayout: true
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
		let widget = LayoutedWidget::of(child, layout_hint, id);
		
		self.childs.push(widget);
	}
	
	pub fn remove_with_id(&mut self, id: i32) {
		let index = self.index_of_id(id).expect("Could not find index of the child widget");
		let removed = self.childs.remove(index);
	}
	
	fn index_of_id(&self, id: i32) -> Option<usize> {
		let mut index: usize = 0;
		for item in &self.childs {
			if item.id() == id {
				return Some(index);
			}
			index += 1;
		}
		return None;
	}
	
	pub fn has_background(&self) -> bool { self.has_background }
	
	pub fn set_has_background(&mut self, has_background: bool) { self.has_background = has_background }
}

impl Widget for Container {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		// Possibly draw background
		if self.has_background {
			graphics.set_color(theme.bg_color_soft());
			graphics.draw_rect(self.bounds.rect(), ShapeDrawParams::fill());
		}
		
		// Draw child widgets
		for child in &self.childs {
			child.borrow_mut().render(graphics, theme);
		}
	}
	
	fn get_preferred_size(&self, _graphics: &Graphics) -> Size {
		self.childs.iter()
			.map(|child| child.borrow().bounds().rect())
			.reduce(|a, b| a.merge(b))
			.map(|rect| rect.size())
			.unwrap_or(Size::of(0, 0))
			+ (self.padding * 2)
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) {
		let delta = self.bounds.offset_to(&bounds);
		
		for child in &self.childs {
			child.borrow_mut().move_by(delta);
		}
		
		self.bounds = bounds;
	}
	
	fn update_layout(&mut self, graphics: &Graphics) {
		for child in &self.childs {
			child.borrow_mut().update_layout_if_needed(graphics);
		}
		
		let top_left = self.top_left();
		self.layout.arrange(&mut self.childs, top_left, graphics);
		
		self.needs_relayout = false;
	}
	
	fn childs(&self) -> Vec<Shared<Widget>> {
		self.childs.iter().map(|it| it.widget()).collect::<Vec<Shared<Widget>>>()
	}
	
	fn this_needs_relayout(&self) -> bool { self.needs_relayout }
}
