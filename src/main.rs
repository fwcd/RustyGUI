extern crate sdl2;

mod gui;
mod utils;

use std::time::Duration;
use sdl2::event::Event;
use gui::core::graphics::Graphics;

fn main() {
	let width = 640;
	let height = 480;
	let title = "VinylFlow";
	
	// Initialize SDL2
	let context = sdl2::init().unwrap();
	let video = context.video().unwrap();
	
	// Create a window and a canvas
	let window = video.window(title, width, height).position_centered().build().expect("Error while creating window");
	let canvas = window.into_canvas().build().expect("Error while creating canvas");
	let mut graphics = Graphics::from(canvas);
	
	let mut event_pump = context.event_pump().unwrap();
	let mut running = true;
	let iterations_per_second = 60;
	let sleep_per_iteration = Duration::new(0, 1_000_000_000u32 / iterations_per_second);
	
	while running {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => running = false,
				_ => {}
			}
		}
		std::thread::sleep(sleep_per_iteration);
		
	}
}
