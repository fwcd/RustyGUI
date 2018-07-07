use super::widget_bounds::WidgetBounds;
use utils::size::Size;
use utils::vec2i::Vec2i;
use gui::core::graphics::Graphics;
use gui::themes::theme::Theme;

/// A GUI widget
pub trait Widget {
	fn bounds(&self) -> &WidgetBounds;
	
	/// This is NOT an API method and should NOT be called
	/// outside of Layout implementations.
	fn internal_set_bounds(&mut self, bounds: WidgetBounds);
	
	fn get_preferred_size(&self, graphics: &Graphics) -> Size;
	
	fn render(&self, graphics: &mut Graphics, theme: &Theme);
	
	fn top_left(&self) -> Vec2i {
		self.bounds().rect().top_left()
	}
	
	fn move_by(&mut self, delta: Vec2i) {
		let new_bounding_rect = self.bounds().rect().moved_by(delta);
		self.internal_set_bounds(WidgetBounds::of(new_bounding_rect));
		self.internal_on_move_by(delta);
	}
	
	fn move_to(&mut self, new_top_left: Vec2i) {
		let delta = new_top_left - self.top_left();
		self.move_by(delta);
	}
	
	/// This is NOT an API method and should ONLY
	/// be implemented.
	fn internal_on_move_by(&mut self, delta: Vec2i) {}
}
