#[derive(Copy, Clone)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}

impl Color {
	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
		Color { red: r, green: g, blue: b, alpha: a }
	}
	
	pub fn rgb(r: u8, g: u8, b: u8) -> Self {
		Color { red: r, green: g, blue: b, alpha: 255 }
	}
	
	pub fn with_alpha(&self, new_alpha: u8) -> Color { Color::rgba(self.red, self.green, self.blue, new_alpha) }
	
	pub fn red() -> Color        { Color::rgb(255, 0,   0)   }
	
	pub fn yellow() -> Color     { Color::rgb(255, 255, 0)   }
	
	pub fn green() -> Color      { Color::rgb(0,   255, 0)   }
	
	pub fn blue() -> Color       { Color::rgb(0,   0,   255) }
	
	pub fn black() -> Color      { Color::rgb(0,   0,   0)   }
	
	pub fn dark_gray() -> Color  { Color::rgb(64,  64,  64)  }
	
	pub fn gray() -> Color       { Color::rgb(128, 128, 128) }
	
	pub fn light_gray() -> Color { Color::rgb(192, 192, 192) }
	
	pub fn white() -> Color      { Color::rgb(255, 255, 255) }
}
