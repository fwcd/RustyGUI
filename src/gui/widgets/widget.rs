use super::widget_params::WidgetDrawParams;
use super::widget_params::RenderedArea;

pub trait Widget {
	fn render(params: WidgetDrawParams) -> RenderedArea;
}
