use std::time::Duration;
use std::thread;
use std::path::Path;
use sdl2::event::Event;
use sdl2::init;
use sdl2::ttf;
use utils::vec2i::Vec2i;
use gui::core::graphics::Graphics;
use gui::core::gui_application::GUIApplication;
use gui::core::mouse::{MouseClickEvent, MouseDragEvent, MouseMoveEvent};
use gui::core::mouse::MouseButton as APIMouseButton;
use gui::core::api_bridge::{api_mouse_button_of};

pub fn run_gui_app(app: &mut GUIApplication) {
	// Initialize SDL2
	let context = init().expect("Could not initialize SDL2 context");
	let ttf = ttf::init().expect("Could not initialize TTF context");
	let video = context.video().expect("Could not initialize video context");
	
	// Initialize font
	let font_path = Path::new("resources/Arial.ttf");
	
	// Create a window and a canvas
	let window = video.window(app.title().as_str(), app.width(), app.height()).position_centered().build().expect("Error while creating window");
	let canvas = window.into_canvas().build().expect("Error while creating canvas");
	let mut graphics = Graphics::from(canvas, ttf, font_path);
	
	// Initialize event loop
	let mut event_pump = context.event_pump().expect("Error while fetching event pump");
	let iterations_per_second = 60;
	let sleep_per_iteration = Duration::new(0, 1_000_000_000u32 / iterations_per_second);
	
	let mut last_mouse_pos: Option<Vec2i> = None;
	let mut mouse_pressed = false;
	let mut mouse_button: Option<APIMouseButton> = None;
	
	'mainloop: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => break 'mainloop,
				Event::MouseButtonDown {x, y, mouse_btn, ..} => {
					let pos = Vec2i::of(x, y);
					let button = api_mouse_button_of(mouse_btn);
					app.on_mouse_down(MouseClickEvent::at(pos, button));
					mouse_button = Some(button);
					mouse_pressed = true;
					last_mouse_pos = Some(pos);
				},
				Event::MouseButtonUp {x, y, mouse_btn, ..} => {
					let pos = Vec2i::of(x, y);
					let button = api_mouse_button_of(mouse_btn);
					app.on_mouse_up(MouseClickEvent::at(pos, button));
					mouse_pressed = false;
					mouse_button = None;
					last_mouse_pos = Some(pos);
				},
				Event::MouseMotion {x, y, ..} => {
					let pos = Vec2i::of(x, y);
					if let Some(last_pos) = last_mouse_pos {
						if mouse_pressed {
							app.on_mouse_drag(MouseDragEvent::between(last_pos, pos, mouse_button.unwrap()));
						} else {
							app.on_mouse_move(MouseMoveEvent::between(last_pos, pos));
						}
					}
					last_mouse_pos = Some(pos);
				},
				_ => {}
			}
		}
		
		app.render(&mut graphics);
		graphics.internal_show();
		thread::sleep(sleep_per_iteration);
	}
}
