extern crate sdl2;
#[macro_use]
extern crate log;
extern crate simple_logger;

mod gui;
mod utils;
mod view;
mod model;

use gui::core::gui_application::GUIApplication;
use gui::themes::theme::Theme;
use gui::widgets::widget_gui_app::WidgetGUIApp;
use gui::widgets::layouts::box_layout::BoxLayout;
use gui::widgets::layouts::border_layout::{BorderLayout, TOP_POS, LEFT_POS, RIGHT_POS, BOTTOM_POS, CENTER_POS};
use gui::widgets::button::Button;
use gui::widgets::label::Label;
use gui::widgets::container::Container;
use gui::core::mainloop::run_gui_app;
use utils::shared::share;

fn main() {
	simple_logger::init_with_level(log::Level::Trace).expect("Could not initialize logger");
	
	let title = "VinylFlow";
	let width = 640;
	let height = 480;
	let layout = BoxLayout::horizontal();
	
	info!("Initializing application...");
	
	let mut app = WidgetGUIApp::new(title, width, height, Box::new(layout));
	app.set_theme(Theme::light());
	{
		let root = app.root();
		root.set_layout(Box::new(BorderLayout::new()));
		// root.add(share(Button::labelled("Test", 64)));
		// root.add(share(Button::labelled("Test2", 64)));
		// root.add(share(Label::of("Demo", 15)));
		// let mut container = Container::vbox();
		// container.add(share(Button::labelled("One", 12)));
		// container.add(share(Button::labelled("Two", 12)));
		// let mut nested = Container::hbox();
		// nested.add(share(Button::labelled("Nested button", 12)));
		// nested.add(share(Label::of("Label", 12)));
		// container.add(share(nested));
		// container.add(share(Button::labelled("Three", 12)));
		// root.add(share(container));
		root.insert(share(Button::labelled("Test", 14)), TOP_POS);
		root.insert(share(Button::labelled("Test", 14)), BOTTOM_POS);
		root.insert(share(Button::labelled("Test", 14)), LEFT_POS);
		root.insert(share(Button::labelled("Test", 14)), RIGHT_POS);
	}
	run_gui_app(&mut app);
}
