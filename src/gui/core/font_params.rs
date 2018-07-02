pub struct FontParams {
	font_size: f32,
	is_bold: bool,
	is_italic: bool,
	is_underlined: bool
}

impl FontParams {
	pub fn of_size(font_size: f32) -> FontParams {
		return FontParams {
			font_size: font_size,
			is_bold: false,
			is_italic: false,
			is_underlined: false
		}
	}
	
	pub fn bold(mut self, is_bold: bool) -> FontParams {
		self.is_bold = is_bold;
		return self;
	}
	
	pub fn italic(mut self, is_italic: bool) -> FontParams {
		self.is_italic = is_italic;
		return self;
	}
	
	pub fn underlined(mut self, is_underlined: bool) -> FontParams {
		self.is_underlined = is_underlined;
		return self;
	}
	
	pub fn is_bold(&self) -> bool { self.is_bold }
	
	pub fn is_italic(&self) -> bool { self.is_italic }
	
	pub fn is_underlined(&self) -> bool { self.is_underlined }
}
