use super::widget::Widget;
use super::label::Label;
use super::bounds::WidgetBounds;
use super::base::WidgetBase;
use super::gui::WidgetGUI;
use utils::shared::WeakShared;
use utils::size::Size;
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;

pub struct Button {
	base: WidgetBase,
	label: Label,
	active: bool
}

impl Button {
	pub fn new(label: Label) -> Button {
		let mut instance = Button {
			base: WidgetBase::empty(),
			label: label,
			active: false
		};
		instance.label.move_by(instance.base.padding);
		instance
	}
	
	pub fn labelled(text: &str, font_size: u16) -> Button {
		Button::new(Label::of(text, font_size))
	}
}

impl Widget for Button {
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme) {
		let mut color = theme.bg().translucent();
		
		if self.active {
			color = color.with_half_alpha();
		}
		
		graphics.set_color(color);
		graphics.draw_rect(self.base.bounds.rect(), ShapeDrawParams::fill());
		self.label.render(graphics, theme);
	}
	
	fn preferred_size(&self, graphics: &Graphics) -> Size {
		self.label.preferred_size(graphics) + (self.base.padding * 2)
	}
	
	fn bounds(&self) -> &WidgetBounds { &self.base.bounds }
	
	fn set_bounds(&mut self, bounds: WidgetBounds) {
		let delta = self.base.bounds.offset_to(&bounds);
		self.label.move_by(delta);
		self.base.bounds = bounds;
	}
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		trace!("Clicked a button");
		self.label.set_text("Clicked!");
		self.base.needs_relayout = true;
		self.active = true;
		true
	}
	
	fn handle_mouse_up(&mut self, event: MouseClickEvent) -> bool {
		self.active = false;
		true
	}
	
	fn needs_relayout(&self) -> bool { self.base.needs_relayout }
	
	fn set_gui(&mut self, gui: WeakShared<WidgetGUI>) { self.base.gui = gui }
}
