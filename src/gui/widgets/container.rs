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
	pub layout_hint: String
}

pub struct Container {
	bounds: WidgetBounds,
	padding: Vec2i,
	childs: Vec<LayoutedWidget>,
	layout: Box<Layout>
}

impl Container {
	pub fn new(layout: Box<Layout>) -> Container {
		Container {
			bounds: WidgetBounds::empty(),
			padding: Vec2i::of(10, 10),
			childs: Vec::new(),
			layout: layout
		}
	}
	
	pub fn hbox() -> Container { Container::new(Box::new(BoxLayout::horizontal())) }
	
	pub fn vbox() -> Container { Container::new(Box::new(BoxLayout::vertical())) }
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
	
	fn set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
}
