use gui::core::color::Color;

pub struct Theme {
	fg_color_strong: Color,
	fg_color_soft: Color,
	bg_color_strong: Color,
	bg_color_soft: Color
}

impl Theme {
	pub fn new() -> Theme { Theme::light() }
	
	pub fn light() -> Theme {
		Theme {
			fg_color_strong: Color::black(),
			fg_color_soft: Color::dark_gray(),
			bg_color_strong: Color::white(),
			bg_color_soft: Color::light_gray()
		}
	}
	
	pub fn dark() -> Theme {
		Theme {
			fg_color_strong: Color::white(),
			fg_color_soft: Color::light_gray(),
			bg_color_strong: Color::black(),
			bg_color_soft: Color::dark_gray()
		}
	}
	
	pub fn fg_colors(mut self, fg_color_strong: Color, fg_color_soft: Color) -> Theme {
		self.fg_color_strong = fg_color_strong;
		self.fg_color_soft = fg_color_soft;
		return self;
	}
	
	pub fn bg_colors(mut self, bg_color_strong: Color, bg_color_soft: Color) -> Theme {
		self.bg_color_strong = bg_color_strong;
		self.bg_color_soft = bg_color_soft;
		return self;
	}
	
	pub fn fg_color_strong(&self) -> Color { self.fg_color_strong }
	
	pub fn fg_color_soft(&self) -> Color { self.fg_color_soft }
	
	pub fn bg_color_strong(&self) -> Color { self.bg_color_strong }
	
	pub fn bg_color_soft(&self) -> Color { self.bg_color_soft }
}
