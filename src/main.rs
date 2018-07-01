extern crate sdl2;

use sdl2::event::Event;

fn main() {
	let width = 640;
	let height = 480;
	let title = "VinylFlow";
	
	// Initialize SDL2
	let context = sdl2::init().unwrap();
	let video = context.video().unwrap();
	
	// Create a window
	let mut window = match video.window(title, width, height).position_centered().opengl().build() {
		Ok(window) => window,
		Err(error) => panic!("Error while creating window: {}", error)
	};
	
	window.show();
	
	let mut events = context.event_pump().unwrap();
	let mut running = true;
	
	while running {
		for event in events.poll_iter() {
			match event {
				Event::Quit {..} => running = false,
				_ => {}
			}
		}
	}
}
