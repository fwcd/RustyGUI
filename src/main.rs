extern crate sdl2;

mod gui;
mod utils;
mod view;
mod model;

use view::vinylflow_view::VinylFlowView;
use gui::core::mainloop::run_gui_app;

fn main() {
	let mut app = VinylFlowView::new();
	run_gui_app(&mut app);
}
