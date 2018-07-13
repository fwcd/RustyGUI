use super::widget::Widget;
use super::label::Label;
use super::bounds::WidgetBounds;
use super::base::WidgetBase;
use super::gui::WidgetGUI;
use utils::shared::{Shared, WeakShared, share};
use utils::size::Size;
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;
use std::cell::RefMut;

pub struct Button {
	base: WidgetBase,
	label: Shared<Label>,
	active: bool,
	is_round: bool
}

impl Button {
	pub fn new(label: Label) -> Button {
		let mut instance = Button {
			base: WidgetBase::empty(),
			label: share(label),
			active: false,
			is_round: false
		};
		instance.label.borrow_mut().move_by(instance.base.padding());
		instance
	}
	
	pub fn is_round(&self) -> bool { self.is_round }
	
	pub fn set_round(&mut self, is_round: bool) { self.is_round = is_round }
	
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
		
		let bounds = self.base.bounds().rect();
		let params = ShapeDrawParams::fill(color);
		
		if self.is_round {
			graphics.draw_oval_in(bounds, params);
		} else {
			graphics.draw_rect(bounds, params);
		}
		self.label.borrow_mut().render(graphics, theme);
	}
	
	fn preferred_size(&self, graphics: &Graphics) -> Size {
		self.label.borrow().preferred_size(graphics) + (self.base.padding() * 2)
	}
	
	fn name(&self) -> &str { "Button" }
	
	fn handle_mouse_down(&mut self, event: MouseClickEvent) -> bool {
		self.label.borrow_mut().set_text("Clicked!");
		self.base.set_needs_relayout(true);
		self.active = true;
		true
	}
	
	fn handle_mouse_up(&mut self, event: MouseClickEvent) -> bool {
		self.active = false;
		true
	}
	
	fn for_each_child(&mut self, each: &mut FnMut(RefMut<Widget>)) {
		each(self.label.borrow_mut());
	}
	
	fn base(&self) -> &WidgetBase { &self.base }
	
	fn base_mut(&mut self) -> &mut WidgetBase { &mut self.base }
}
