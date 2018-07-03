use gui::widgets::widget::Widget;

pub trait Layout {
	fn on_add_component(&self, widget: &Widget, layout_hint: String, all: &Vec<Box<Widget>>);
	
	fn on_remove_component(&self, widget: &Widget, layout_hint: String, all: &Vec<Box<Widget>>);
}
