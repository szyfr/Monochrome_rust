

//= Imports
use std::collections::HashMap;
use monorust::{raylib, data, settings, localization, graphics, camera};


//= Main
fn main() {
	let mut gamestate = data::Gamestate{
		settings		: settings::load(),
		localization	: HashMap::new(),
		fonts			: HashMap::new(),
		textures		: HashMap::new(),
		camera			: camera::init(),
	};
	gamestate.localization = localization::load(&gamestate.settings.language);

	//* Raylib */
	raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window( &gamestate );
	
	//* Graphics */
	gamestate.fonts		= graphics::load_fonts();
	gamestate.textures	= graphics::load_textures();

	//* Camera / Player */

	// ! TEMP

	while !raylib::window_should_close() {
		//* Update */

		//* Draw */
		raylib::begin_drawing();
		{
			raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});

			raylib::begin_3d_mode(&gamestate.camera);

			raylib::draw_grid(100, 1.0);

			raylib::end_3d_mode();

			raylib::draw_text_pro(
				gamestate.fonts["default"],
				"Fuck!",
				raylib_ffi::Vector2{x:0.5,y:5.0},
				raylib_ffi::Vector2{x:0.0,y:0.0},
				0.0,
				16.0,
				0.0,
				raylib_ffi::colors::BLACK,
			);
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}