use super::widget::Widget;
use utils::shared::Shared;
use std::cell::{Ref, RefMut};

#[derive(Clone)]
pub struct LayoutedWidget {
	widget: Shared<Widget>,
	layout_hint: String,
	id: i32
}

impl LayoutedWidget {
	pub fn of(widget: Shared<Widget>, layout_hint: &str, id: i32) -> Self {
		LayoutedWidget {
			widget: widget,
			layout_hint: layout_hint.to_string(),
			id: id
		}
	}
	
	pub fn widget(&self) -> Shared<Widget> { self.widget.clone() }
	
	pub fn borrow(&self) -> Ref<Widget> { self.widget.borrow() }
	
	pub fn borrow_mut(&self) -> RefMut<Widget> { self.widget.borrow_mut() }
	
	pub fn layout_hint(&self) -> &String { &self.layout_hint }
	
	pub fn id(&self) -> i32 { self.id }
}
