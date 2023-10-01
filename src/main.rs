

//= Imports
use std::collections::HashMap;
use monorust::{raylib, data, settings, localization};


//= Main
fn main() {
	let mut gamestate = data::Gamestate{
		settings : settings::load(),
		localization: HashMap::new(),
	};
	gamestate.localization = localization::load(&gamestate.settings.language);

	//* Raylib */
	raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window( &gamestate );
	
	//* Graphics */

	//* Camera / Player */

	// ! TEMP

	while !raylib::window_should_close() {
		//* Update */

		//* Draw */
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

	//* Raylib */
	raylib::close_window();
}