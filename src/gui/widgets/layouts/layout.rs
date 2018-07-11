use gui::widgets::layouted_widget::LayoutedWidget;
use gui::core::graphics::Graphics;
use utils::vec2i::Vec2i;

pub trait Layout {
	fn arrange(&self, widgets: &mut Vec<LayoutedWidget>, top_left: Vec2i, graphics: &Graphics);
}
