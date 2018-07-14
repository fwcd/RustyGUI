extern crate rustygui;
#[macro_use]
extern crate log;
extern crate simple_logger;

use rustygui::gui::themes::theme::Theme;
use rustygui::gui::widgets::widget_gui_app::WidgetGUIApp;
use rustygui::gui::widgets::layouts::box_layout::BoxLayout;
use rustygui::gui::widgets::layouts::border_layout::{BorderLayout, TOP_POS, LEFT_POS, RIGHT_POS, BOTTOM_POS, CENTER_POS};
use rustygui::gui::widgets::button::Button;
use rustygui::gui::widgets::label::Label;
use rustygui::gui::widgets::slider::Slider;
use rustygui::gui::widgets::widget_utils::widget_of;
use rustygui::gui::widgets::container::Container;
use rustygui::gui::sdl2::mainloop::run_gui_app;

fn main() {
	simple_logger::init_with_level(log::Level::Trace).expect("Could not initialize logger");
	
	let title = "RustyGUI";
	let width = 640;
	let height = 480;
	let layout = BoxLayout::horizontal();
	
	info!("Initializing application...");
	
	let mut app = WidgetGUIApp::new(title, width, height, layout);
	{
		let mut gui = app.borrow_gui_mut();
		gui.set_theme(Theme::dark());
		
		let mut root = gui.borrow_root_mut();
		let mut test_button = Button::labelled("Test", 32);
		test_button.set_action(|b| b.set_text("Clicked"));
		root.add(test_button);
		root.add(Label::of("Demo", 15));
		root.add(Slider::new(0.0..=10.0));
		root.add({
			let mut container = Container::vbox();
			container.add(Button::labelled("One", 12));
			container.add(Button::labelled("Two", 12));
			container.add({
				let mut nested = Container::new(BorderLayout::new());
				nested.insert(Button::labelled("Nested button", 12), TOP_POS);
				nested.insert(Button::labelled("Just a large button", 12), LEFT_POS);
				nested.insert(Label::of("Label", 12), RIGHT_POS);
				nested
			});
			container.add({
				let mut nested = Container::hbox();
				nested.add(Button::labelled("A", 12));
				nested.add(Button::labelled("B", 12));
				nested.add(Button::labelled("C", 12));
				nested
			});
			container.add(Button::labelled("Three", 12));
			container
		});
	}
	run_gui_app(&mut app);
}