pub struct ShapeDrawParams {
	pub fill: bool,
	pub outline: bool
}

impl ShapeDrawParams {
	pub fn fill() -> ShapeDrawParams {
		return ShapeDrawParams { fill: true, outline: false }
	}
	
	pub fn outline() -> ShapeDrawParams {
		return ShapeDrawParams { fill: false, outline: true }
	}
}
