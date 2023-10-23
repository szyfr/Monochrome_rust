

//= Imports
use monorust::{raylib, data, graphics, camera, player, overworld, world, events};


//= Main
fn main() {
	//* Create Initial gamestate */
	unsafe { data::SETTINGS.load(); }
	let mut gamestate = data::init();

	//* Raylib */
	raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(&gamestate);
	raylib::set_target_fps(data::get_screenfps());
	
	//* Graphics */
	gamestate.fonts		= graphics::load_fonts();
	gamestate.textures	= graphics::load_textures();
	gamestate.models	= graphics::load_models();
	gamestate.player.unit.animator.textures = overworld::load_unit_textures("player_1");

	//* Camera / Player */

	// ! TEMP
	gamestate.worldData.load_world("newbark");
	gamestate.worldData.load_entities("newbark");
	gamestate.worldData.load_triggers("newbark");
	gamestate.worldData.load_events("newbark");

	while !raylib::window_should_close() {
		//* Update */
		camera::update(&mut gamestate);
		//events
		player::controls(&mut gamestate);

		//* Draw */
		raylib::begin_drawing();
		{
			raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});

			raylib::begin_3d_mode(&gamestate.camera);

			world::draw_world(&mut gamestate);

			raylib::end_3d_mode();

			events::textbox::draw(&mut gamestate);

			raylib::draw_fps(0,0);
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}