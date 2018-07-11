use super::widget::Widget;
use super::label::Label;
use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use utils::vec2i::Vec2i;
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;

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
	
	pub fn labelled(text: &str, font_size: u16) -> Button {
		Button::new(Label::of(text, font_size))
	}
}

impl Widget for Button {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		graphics.set_color(theme.bg_color_soft());
		graphics.draw_rect(self.bounds.rect(), ShapeDrawParams::fill());
		self.label.render(graphics, theme);
	}
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size {
		self.label.get_preferred_size(graphics) + (self.padding * 2)
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) {
		let delta = self.bounds.offset_to(&bounds);
		self.label.move_by(delta);
		self.bounds = bounds;
	}
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		trace!("Clicked a button");
		self.label.set_text("Clicked!");
		true
	}
}
