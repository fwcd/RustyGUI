use super::color::Color;

#[derive(Copy, Clone)]
pub struct ShapeDrawParams {
	filled: bool,
	outlined: bool,
	color: Color
}

impl ShapeDrawParams {
	pub fn fill(color: Color) -> Self {
		Self { filled: true, outlined: false, color: color }
	}
	
	pub fn outline(color: Color) -> Self {
		Self { filled: false, outlined: true, color: color }
	}
	
	pub fn color(&self) -> Color { self.color }
	
	pub fn filled(&self) -> bool { self.filled }
	
	pub fn outlined(&self) -> bool { self.outlined }
}
