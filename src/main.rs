extern crate sdl2;

mod gui;
mod utils;
mod view;
mod model;

use gui::core::gui_application::GUIApplication;
use gui::widgets::widget_gui_app::WidgetGUIApp;
use gui::widgets::layouts::box_layout::BoxLayout;
use gui::widgets::button::Button;
use gui::widgets::label::Label;
use gui::widgets::container::Container;
use gui::core::mainloop::run_gui_app;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
	let title = "VinylFlow";
	let width = 640;
	let height = 480;
	let layout = BoxLayout::horizontal();
	
	let mut app = WidgetGUIApp::new(title, width, height, Box::new(layout));
	{
		let root = app.root();
		root.add(Rc::new(RefCell::new(Button::labelled("Test", 14))));
		root.add(Rc::new(RefCell::new(Label::of("Demo", 15))));
		let mut container = Container::vbox();
		container.add(Rc::new(RefCell::new(Button::labelled("One", 12))));
		container.add(Rc::new(RefCell::new(Button::labelled("Two", 12))));
		container.add(Rc::new(RefCell::new(Button::labelled("Three", 12))));
		root.add(Rc::new(RefCell::new(container)));
	}
	run_gui_app(&mut app);
}
