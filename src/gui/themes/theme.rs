use super::palette::{Palette, PaletteBuilder};
use gui::core::color::Color;

pub struct Theme {
	fg: Palette,
	bg: Palette
}

impl Theme {
	pub fn new() -> Theme { Theme::light() }
	
	pub fn light() -> Theme {
		Theme {
			fg: PaletteBuilder::of(Color::black())
					.strong(Color::black())
					.soft(Color::dark_gray())
					.translucent(Color::black().with_alpha(64))
					.alternative(Color::blue())
					.build(),
			bg: PaletteBuilder::of(Color::white())
					.strong(Color::white())
					.soft(Color::light_gray())
					.translucent(Color::black().with_alpha(64))
					.alternative(Color::blue())
					.build()
		}
	}
	
	pub fn dark() -> Theme {
		Theme {
			fg: PaletteBuilder::of(Color::white())
					.strong(Color::white())
					.soft(Color::light_gray())
					.translucent(Color::white().with_alpha(64))
					.alternative(Color::orange())
					.build(),
			bg: PaletteBuilder::of(Color::black())
					.strong(Color::black())
					.soft(Color::dark_gray())
					.translucent(Color::white().with_alpha(64))
					.alternative(Color::orange())
					.build()
		}
	}
	
	pub fn fg(&self) -> &Palette { &self.fg }
	
	pub fn bg(&self) -> &Palette { &self.bg }
}
