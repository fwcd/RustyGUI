use gui::core::color::Color;

pub struct Theme {
	fg_color1: Color,
	fg_color2: Color,
	bg_color1: Color,
	bg_color2: Color
}

impl Theme {
	pub fn new() -> Theme { Theme::light() }
	
	pub fn light() -> Theme {
		Theme {
			fg_color1: Color::black(),
			fg_color2: Color::dark_gray(),
			bg_color1: Color::white(),
			bg_color2: Color::light_gray()
		}
	}
	
	pub fn dark() -> Theme {
		Theme {
			fg_color1: Color::white(),
			fg_color2: Color::light_gray(),
			bg_color1: Color::black(),
			bg_color2: Color::dark_gray()
		}
	}
	
	pub fn fg_colors(mut self, fg_color1: Color, fg_color2: Color) -> Theme {
		self.fg_color1 = fg_color1;
		self.fg_color2 = fg_color2;
		return self;
	}
	
	pub fn bg_colors(mut self, bg_color1: Color, bg_color2: Color) -> Theme {
		self.bg_color1 = bg_color1;
		self.bg_color2 = bg_color2;
		return self;
	}
	
	pub fn fg_color1(&self) -> &Color { self.fg_color1 }
	
	pub fn fg_color2(&self) -> &Color { self.fg_color2 }
	
	pub fn bg_color1(&self) -> &Color { self.bg_color1 }
	
	pub fn bg_color2(&self) -> &Color { self.bg_color2 }
}
