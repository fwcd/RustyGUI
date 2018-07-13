use super::bounds::WidgetBounds;
use super::gui::WidgetGUI;
use super::widget::Widget;
use utils::shared::WeakShared;
use utils::vec2i::Vec2i;

pub struct WidgetBase {
	bounds: WidgetBounds,
	padding: Vec2i,
	needs_relayout: bool,
	gui: WeakShared<WidgetGUI>,
	this: Option<WeakShared<Widget>>
}

impl WidgetBase {
	pub fn empty() -> Self {
		Self::new(WidgetBounds::empty())
	}
	
	pub fn new(bounds: WidgetBounds) -> Self {
		Self {
			bounds: bounds,
			padding: Vec2i::of(10, 10),
			needs_relayout: true,
			gui: WeakShared::new(),
			this: None
		}
	}
	
	pub fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	pub fn padding(&self) -> Vec2i { self.padding }
	
	pub fn needs_relayout(&self) -> bool { self.needs_relayout }
	
	pub fn gui(&self) -> WeakShared<WidgetGUI> { self.gui.clone() }
	
	pub fn set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
	
	pub fn set_padding(&mut self, padding: Vec2i) { self.padding = padding }
	
	pub fn set_needs_relayout(&mut self, needs_relayout: bool) { self.needs_relayout = needs_relayout; }
	
	pub fn set_gui(&mut self, gui: WeakShared<WidgetGUI>) { self.gui = gui; }
	
	pub fn this(&self) -> Option<WeakShared<Widget>> { self.this.as_ref().map(|it| it.clone()) }
	
	pub fn set_this(&mut self, this: WeakShared<Widget>) { self.this = Some(this) }
}
