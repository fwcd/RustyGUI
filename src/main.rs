extern crate sdl2;

mod gui;
mod utils;
mod view;
mod model;

use gui::widgets::widget_gui_app::WidgetGUIApp;
use gui::widgets::layouts::box_layout::BoxLayout;
use gui::core::mainloop::run_gui_app;

fn main() {
	let title = "VinylFlow";
	let width = 640;
	let height = 480;
	let layout = BoxLayout::horizontal();
	
	let mut app = WidgetGUIApp::new(title, width, height, Box::new(layout));
	run_gui_app(&mut app);
}
