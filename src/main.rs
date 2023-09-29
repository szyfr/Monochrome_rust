

//= Imports
//use std::collections::HashMap;

use monorust::data;
use monorust::settings;
use monorust::raylib;


//= Procedures
fn update() {

}

fn draw() {
	raylib::begin_drawing();
	{
		raylib::clear_background(raylib_ffi::colors::WHITE);
		raylib::draw_text(
			raylib_ffi::rl_str!("FUCK"),
			0,
			0,
			20,
			raylib_ffi::colors::BLACK,
		);
	}
	raylib::end_drawing();
}

fn main() {
	init();
	while !raylib::window_should_close() {
		update();
		draw();
	}
	close();
}

fn init() {
	//* Debug *//

	//* Settings / Localization *//
	settings::init();

	//* Raylib *//
	raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window( &"Monochrome" ); // TODO
	
	//* Graphics *//

	//* Camera / Player *//

	// ! TEMP
}

fn close() {
	raylib::close_window();
}