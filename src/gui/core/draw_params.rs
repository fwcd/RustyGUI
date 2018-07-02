pub struct ShapeDrawParams {
	filled: bool,
	outlined: bool
}

impl ShapeDrawParams {
	pub fn fill() -> ShapeDrawParams {
		return ShapeDrawParams { filled: true, outlined: false }
	}
	
	pub fn outline() -> ShapeDrawParams {
		return ShapeDrawParams { filled: false, outlined: true }
	}
	
	pub fn filled(&self) -> bool { self.filled }
	
	pub fn outlined(&self) -> bool { self.outlined }
}
