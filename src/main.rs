

//= Imports
use monorust::{raylib, data, graphics, camera, player, overworld, world};


//= Main
fn main() {
	//* Create Initial gamestate */
	let mut gamestate = data::init();

	//* Raylib */
	raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(&gamestate);
	raylib::set_target_fps(gamestate.settings.screenFps);
	
	//* Graphics */
	gamestate.fonts		= graphics::load_fonts();
	gamestate.textures	= graphics::load_textures();
	gamestate.models	= graphics::load_models();
	gamestate.player.unit.animator.textures = overworld::load_unit_textures("player_1");

	//* Camera / Player */

	// ! TEMP
	gamestate.currentMap = world::load_world("newbark".to_string());
	gamestate.unitMap    = world::load_entities("newbark".to_string());
	gamestate.triggerMap = world::load_triggers("newbark".to_string());
	gamestate.eventList  = world::load_events("newbark".to_string());

	while !raylib::window_should_close() {
		//* Update */
		gamestate.camera = camera::update(&gamestate);
		gamestate.player = player::controls(&gamestate);

		//* Draw */
		raylib::begin_drawing();
		{
			raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});

			raylib::begin_3d_mode(&gamestate.camera);

			gamestate = world::draw_world(gamestate);

			raylib::end_3d_mode();

			raylib::draw_fps(0,0);
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}