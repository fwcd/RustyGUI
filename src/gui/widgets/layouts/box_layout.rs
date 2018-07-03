use gui::widgets::widget::Widget;
use super::layout::Layout;

pub struct BoxLayout {
	is_horizontal: bool
}

impl BoxLayout {
	pub fn horizontal() -> BoxLayout {
		BoxLayout { is_horizontal: true }
	}
	
	pub fn vertical() -> BoxLayout {
		BoxLayout { is_horizontal: false }
	}
}

impl Layout for BoxLayout {
	fn on_add_component(&self, widget: &Widget, layout_hint: String, all: &Vec<Box<Widget>>) {
		// TODO
	}
	
	fn on_remove_component(&self, widget: &Widget, layout_hint: String, all: &Vec<Box<Widget>>) {
		// TODO
	}
}
