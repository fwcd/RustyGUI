pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8
}

impl Color {
	pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
		return Color { red: r, green: g, blue: b, alpha: a };
	}
	
	pub fn rgb(r: u8, g: u8, b: u8) -> Color {
		return Color { red: r, green: g, blue: b, alpha: 255 };
	}
}
