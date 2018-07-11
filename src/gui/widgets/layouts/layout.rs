use gui::widgets::layouted_widget::LayoutedWidget;
use gui::widgets::bounds::WidgetBounds;
use gui::core::graphics::Graphics;

pub trait Layout {
	fn arrange(&self, widgets: &mut Vec<LayoutedWidget>, parent_bounds: &WidgetBounds, graphics: &Graphics);
}
