

//= Imports
use std::collections::HashMap;
use monorust::{raylib, data, settings, localization, graphics, camera, player, overworld, world};


//= Main
fn main() {
	let mut gamestate = data::Gamestate{
		settings		: settings::load(),
		localization	: HashMap::new(),
		fonts			: HashMap::new(),
		textures		: HashMap::new(),
		models			: HashMap::new(),
		animations		: graphics::load_animations(),
		currentMap		: HashMap::new(),
		unitMap			: HashMap::new(),
		camera			: camera::init(),
		player			: player::init(),
	};
	gamestate.localization = localization::load(&gamestate.settings.language);

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
	gamestate.unitMap = world::load_entities("newbark".to_string());

	while !raylib::window_should_close() {
		//* Update */
		gamestate.camera = camera::update(&gamestate);
		gamestate.player = player::controls(&gamestate);

		//* Draw */
		raylib::begin_drawing();
		{
			raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});

			raylib::begin_3d_mode(&gamestate.camera);

			//raylib::draw_grid(100, 1.0);

			gamestate = world::draw_world(gamestate);

			//gamestate.player.unit = overworld::draw_unit(
			//	&gamestate.animations,
			//	gamestate.models["unit"],
			//	gamestate.player.unit,
			//	gamestate.camera.rotation,
			//);

			raylib::end_3d_mode();

			//raylib::draw_text_pro(
			//	gamestate.fonts["default"],
			//	"Fuck!",
			//	raylib_ffi::Vector2{x:0.5,y:5.0},
			//	raylib_ffi::Vector2{x:0.0,y:0.0},
			//	0.0,
			//	16.0,
			//	0.0,
			//	raylib_ffi::colors::BLACK,
			//);
			raylib::draw_fps(0,0);
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
}