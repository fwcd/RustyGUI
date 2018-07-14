use super::widget::Widget;
use super::bounds::WidgetBounds;
use super::base::WidgetBase;
use super::layouts::layout::Layout;
use super::layouts::box_layout::BoxLayout;
use super::layouted_widget::LayoutedWidget;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;
use utils::reduce::Reduce;
use utils::size::Size;
use utils::shared::{Shared, share};
use utils::vec2i::Vec2i;
use std::rc::Rc;
use std::cell::RefMut;

pub struct Container {
	base: WidgetBase,
	childs: Vec<LayoutedWidget>,
	layout: Box<Layout>,
	current_id: i32,
	has_background: bool,
	preferred_size_override: Option<Size>
}

impl Container {
	pub fn new<L>(layout: L) -> Self where L: 'static + Layout {
		Container {
			base: WidgetBase::empty(),
			childs: Vec::new(),
			layout: Box::new(layout),
			current_id: 0,
			has_background: true,
			preferred_size_override: None
		}
	}
	
	pub fn hbox() -> Self { Self::new(BoxLayout::horizontal()) }
	
	pub fn vbox() -> Self { Self::new(BoxLayout::vertical()) }
	
	pub fn set_layout(&mut self, layout: Box<Layout>) { self.layout = layout }
	
	pub fn add<W>(&mut self, child: W) where W: 'static + Widget {
		self.add_shared(share(child));
	}
	
	pub fn insert<W>(&mut self, child: W, layout_hint: &str) where W: 'static + Widget {
		self.insert_shared(share(child), layout_hint);
	}
	
	pub fn add_shared(&mut self, child: Shared<Widget>) {
		self.insert_shared(child, "");
	}
	
	pub fn insert_shared(&mut self, child: Shared<Widget>, layout_hint: &str) {
		let current_id = self.current_id;
		self.insert_with_id(child, layout_hint, current_id);
		self.current_id += 1;
	}
	
	pub fn insert_with_id(&mut self, child: Shared<Widget>, layout_hint: &str, id: i32) {
		{
			let mut child_ref = child.borrow_mut();
			child_ref.set_this(Rc::downgrade(&child));
			child_ref.set_gui(self.base.gui().clone());
		}
		let widget = LayoutedWidget::of(child, layout_hint, id);
		self.childs.push(widget);
		self.set_needs_relayout(true);
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
	
	pub fn set_preferred_size(&mut self, size: Size) { self.preferred_size_override = Some(size) }
	
	fn compute_preferred_size(&self, graphics: &Graphics) -> Size {
		self.childs.iter()
			.map(|child| child.borrow().preferred_bounds(graphics).rect())
			.reduce(|a, b| a.merge(b))
			.map(|rect| rect.size())
			.unwrap_or(Size::of(0, 0))
			+ (self.actual_padding() * 2)
	}
	
	fn actual_padding(&self) -> Vec2i {
		if self.layout.uses_parent_padding() {
			self.base.padding()
		} else { Vec2i::of(0, 0) }
	}
}

impl Widget for Container {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		// Possibly draw background
		if self.has_background {
			let color = theme.bg().translucent();
			graphics.draw_rect(self.bounds().rect(), ShapeDrawParams::fill(color));
		}
		
		// Draw child widgets
		for child in &self.childs {
			child.borrow_mut().render(graphics, theme);
		}
	}
	
	fn preferred_size(&self, graphics: &Graphics) -> Size {
		self.preferred_size_override.unwrap_or_else(|| self.compute_preferred_size(graphics))
	}
	
	fn name(&self) -> &str { "Container" }
	
	fn bounds(&self) -> &WidgetBounds { &self.base.bounds() }
	
	fn internal_on_update_layout(&mut self, graphics: &Graphics) {
		let bounds = self.preferred_bounds(graphics);
		self.layout.arrange(&mut self.childs, &bounds, graphics);
	}
	
	fn for_each_child(&mut self, each: &mut FnMut(RefMut<Widget>)) {
		for child in &self.childs {
			let mut child_widget = child.widget();
			each(child_widget.borrow_mut());
		}
	}
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
