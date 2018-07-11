use gui::core::color::Color;

pub struct Palette {
	strong: Color,
	soft: Color,
	translucent: Color,
	alternative: Color
}

impl Palette {
	pub fn strong(&self) -> Color { self.strong }
	
	pub fn soft(&self) -> Color { self.soft }
	
	pub fn translucent(&self) -> Color { self.translucent }
	
	pub fn alternative(&self) -> Color { self.alternative }
}

// TODO: Write a builder macro to reduce the amount of boilerplate
// required to encapsulate the fields properly

pub struct PaletteBuilder {
	palette: Palette
}

impl PaletteBuilder {
	pub fn of(base_color: Color) -> Self {
		PaletteBuilder { palette: Palette {
			strong: base_color,
			soft: base_color,
			translucent: base_color,
			alternative: base_color
		} }
	}
	
	pub fn strong(mut self, color: Color) -> Self { self.palette.strong = color; self }
	
	pub fn soft(mut self, color: Color) -> Self { self.palette.soft = color; self }
	
	pub fn translucent(mut self, color: Color) -> Self { self.palette.translucent = color; self }
	
	pub fn alternative(mut self, color: Color) -> Self { self.palette.alternative = color; self }
	
	pub fn build(self) -> Palette { self.palette }
}
