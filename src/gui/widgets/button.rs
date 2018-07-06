use super::widget::Widget;
use super::label::Label;
use super::widget_params::WidgetDrawParams;
use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use utils::vec2i::Vec2i;
use gui::core::graphics::Graphics;

pub struct Button {
	bounds: WidgetBounds,
	padding: Vec2i,
	label: Label
}

impl Button {
	pub fn new(label: Label) -> Button {
		let padding = Vec2i::of(10, 10);
		let mut padded_label = label;
		padded_label.move_to(padding);
		
		Button {
			bounds: WidgetBounds::empty(),
			padding: padding,
			label: padded_label
		}
	}
	
	pub fn labelled(text: &str, font_size: f32) -> Button {
		Button::new(Label::of(text, font_size))
	}
}

impl Widget for Button {
	fn render(&self, params: &mut WidgetDrawParams) {
		self.label.render(params);
	}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size {
		self.label.get_preferred_size(graphics) + (self.padding * 2)
	}
	
	fn internal_on_move_by(&mut self, delta: Vec2i) {
		self.label.move_by(delta);
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn internal_set_bounds(&mut self, bounds: WidgetBounds) { self.bounds = bounds }
}
