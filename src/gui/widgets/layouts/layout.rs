use gui::widgets::widget::Widget;
use gui::core::graphics::Graphics;
use utils::shared::Shared;

pub trait Layout {
	fn on_add_widget(&mut self, shared_widget: Shared<Widget>, layout_hint: &String, graphics: &Graphics);
	
	fn on_remove_widget(&mut self, shared_widget: Shared<Widget>, layout_hint: &String, graphics: &Graphics);
}
