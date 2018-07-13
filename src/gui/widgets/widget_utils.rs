use super::widget::Widget;
use utils::shared::{share, Shared};
use std::rc::Rc;

pub fn widget_of<T>(child: T) -> Shared<T> where T: Widget + 'static {
	let shared_ref: Shared<T> = share(child);
	{
		let widget_ref: Shared<Widget> = shared_ref.clone() as Shared<Widget>;
		shared_ref.borrow_mut().set_this(Rc::downgrade(&widget_ref));
	}
	shared_ref
}
