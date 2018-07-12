use super::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct FontParams {
	font_size: u16,
	is_bold: bool,
	is_italic: bool,
	is_underlined: bool,
	color: Option<Color>
}

impl FontParams {
	pub fn of_size(font_size: u16) -> Self {
		FontParams {
			font_size: font_size,
			is_bold: false,
			is_italic: false,
			is_underlined: false,
			color: None
		}
	}
	
	pub fn set_color(&mut self, color: Color) { self.color = Some(color); }
	
	pub fn set_bold(&mut self, is_bold: bool) { self.is_bold = is_bold; }
	
	pub fn set_italic(&mut self, is_italic: bool) { self.is_italic = is_italic; }
	
	pub fn set_underlined(&mut self, is_underlined: bool) { self.is_underlined = is_underlined; }
	
	pub fn color(&self) -> Option<Color> { self.color }
	
	pub fn is_bold(&self) -> bool { self.is_bold }
	
	pub fn is_italic(&self) -> bool { self.is_italic }
	
	pub fn is_underlined(&self) -> bool { self.is_underlined }
	
	pub fn font_size(&self) -> u16 { self.font_size }
}
