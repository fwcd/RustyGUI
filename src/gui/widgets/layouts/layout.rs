use gui::widgets::widget::Widget;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Layout {
	fn on_add_component(&self, widget: Rc<RefCell<Widget>>, layout_hint: &String);
	
	fn on_remove_component(&self, widget: Rc<RefCell<Widget>>, layout_hint: &String);
}
