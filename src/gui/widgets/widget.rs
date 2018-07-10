use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use utils::vec2i::Vec2i;
use gui::core::graphics::Graphics;
use gui::themes::theme::Theme;

/// A GUI widget
pub trait Widget {
	fn bounds(&self) -> &WidgetBounds;
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn set_bounds(&mut self, bounds: WidgetBounds);
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size;
	
	fn render(&mut self, graphics: &mut Graphics, theme: &Theme);
	
	fn top_left(&self) -> Vec2i {
		self.bounds().rect().top_left()
	}
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn move_by(&mut self, delta: Vec2i) {
		let new_bounding_rect = self.bounds().rect().moved_by(delta);
		self.set_bounds(WidgetBounds::of(new_bounding_rect));
	}
	
	/// This method should ONLY be called inside of
	/// Layout managers OR when no layout is used at all.
	/// Otherwise conflicts may occur.
	fn move_to(&mut self, new_top_left: Vec2i) {
		let delta = new_top_left - self.top_left();
		self.move_by(delta);
	}
}
