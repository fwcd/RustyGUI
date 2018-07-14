use super::widget::Widget;
use super::label::Label;
use super::widget_utils::widget_of;
use super::base::WidgetBase;
use utils::shared::{Shared, share};
use utils::size::Size;
use gui::core::mouse::MouseClickEvent;
use gui::core::graphics::Graphics;
use gui::core::draw_params::ShapeDrawParams;
use gui::themes::theme::Theme;
use std::cell::RefMut;

pub struct Button<'a> {
	base: WidgetBase,
	label: Shared<Label>,
	action: Option<Shared<Fn(&mut Button<'a>) + 'a>>,
	active: bool,
	is_round: bool
}

impl <'a> Button<'a> {
	pub fn new(label: Label) -> Self {
		let mut instance = Self {
			base: WidgetBase::empty(),
			label: widget_of(label),
			action: None,
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
	
	pub fn remove_action(&mut self) { self.action = None; }
	
	pub fn set_action<F>(&mut self, action: F) where F: 'a + Fn(&mut Self) { self.action = Some(share(action)) }
	
	pub fn text(&self) -> String { self.label.borrow().text().to_owned() }
	
	pub fn set_text(&mut self, text: &str) {
		self.label.borrow_mut().set_text(text);
		self.set_needs_relayout(true);
	}
	
	fn cloned_action(&self) -> Option<Shared<Fn(&mut Button<'a>) + 'a>> {
		self.action.as_ref().map(|it| it.clone())
	}
}

impl <'a> Widget for Button<'a> {
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
		if let Some(action) = &self.cloned_action() {
			action.borrow()(self);
		}
		self.set_needs_relayout(true);
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
