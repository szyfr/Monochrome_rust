

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use monorust::{raylib, data, player, world, events, battle};


//= Main
fn main() {
	//* Create Initial gamestate */
	unsafe { data::SETTINGS.load(); }
	let mut gamestate = data::init();

	//* Raylib */
	//raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(&gamestate);
	raylib::set_line_spacing(data::get_screenratio() as i32 * 56);
	raylib::set_target_fps(data::get_screenfps());
	raylib::init_audio_device();

	//* Graphics */
	gamestate.graphics.load();

	//* Camera / Player */

	// ! TEMP
	gamestate.worldData.load_all("newbark");
	gamestate.audio.play_music("new_bark_town".to_string());

	while !raylib::window_should_close() && gamestate.running {
		//* Update */
		gamestate.camera.update(
			gamestate.player.unit.position,
			gamestate.eventHandler.currentEvent == "".to_string(),
			gamestate.battleData.started,
		);
		player::controls(&mut gamestate);
		gamestate.worldData.update();
		gamestate.audio.update();
		if gamestate.battleData.started { gamestate.battleData.update(); }

		raylib::begin_drawing();
		{
			raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});
			//raylib::clear_background(raylib_ffi::colors::SKYBLUE);

			raylib::begin_3d_mode(&gamestate.camera);

			if !gamestate.battleData.started {
				world::draw_world(&mut gamestate);
				events::animation::draw_emotes(&mut gamestate);
			} else {
				//gamestate.battleData.draw(&mut gamestate);
				battle::draw(&mut gamestate);
			}

			raylib::end_3d_mode();

			events::textbox::draw(&mut gamestate);
			events::animation::draw(&mut gamestate);
			if gamestate.player.menu.open != player::MenuOptions::None { player::draw_menu(&gamestate); }

			raylib::draw_fps(0,0);
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
	raylib::close_audio_device();
}
