

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use monorust::{raylib, data, graphics, camera, player, overworld, world, events};


//= Main
fn main() {
	//* Create Initial gamestate */
	unsafe { data::SETTINGS.load(); }
	let mut gamestate = data::init();

	//* Raylib */
	//raylib::set_trace_log_level(raylib_ffi::enums::TraceLogLevel::None);
	raylib::init_window(&gamestate);
	raylib::set_target_fps(data::get_screenfps());
	raylib::init_audio_device();

	//* Graphics */
	let mut _graphicsHandler = graphics::init();
	gamestate.fonts		= graphics::load_fonts();
	gamestate.textures	= graphics::load_textures();
	gamestate.models	= graphics::load_models();
	gamestate.player.unit.animator.textures = overworld::load_unit_textures("player_1");

	//* Camera / Player */

	// ! TEMP
	gamestate.worldData.load_all("newbark");
	gamestate.audio.play_music("new_bark_town".to_string());

	while !raylib::window_should_close() {
		//* Update */
		camera::update(&mut gamestate);
		player::controls(&mut gamestate);
		gamestate.audio.update();

		raylib::begin_drawing();
		{
			//raylib::clear_background(raylib_ffi::Color{r:57,g:57,b:57,a:255});
			raylib::clear_background(raylib_ffi::colors::SKYBLUE);

			raylib::begin_3d_mode(&gamestate.camera);

			world::draw_world(&mut gamestate);
			events::animation::draw_emotes(&mut gamestate);

			raylib::end_3d_mode();

			events::textbox::draw(&mut gamestate);
			events::animation::draw(&mut gamestate);

			raylib::draw_fps(0,0);
		}
		raylib::end_drawing();
	}

	//* Raylib */
	raylib::close_window();
	raylib::close_audio_device();
}

// ! Shader stuff Keeping this here for future reference
//gamestate.shader = Some(raylib::load_shader("", "data/shaders/write_depth.fs"));
//let target = raylib::load_render_texture_depth_tex(data::get_screenwidth(), data::get_screenheight());
//gamestate.shader = Some(raylib::load_shader("data/shaders/lighting.vs", "data/shaders/fog.fs"));
//unsafe {
//	*gamestate.shader.unwrap().locs.wrapping_add(raylib::ShaderLocationIndex::ShaderLocMatrixModel as usize) = raylib::get_shader_location(gamestate.shader.unwrap(), "matModel");
//	*gamestate.shader.unwrap().locs.wrapping_add(raylib::ShaderLocationIndex::ShaderLocVectorView as usize)  = raylib::get_shader_location(gamestate.shader.unwrap(), "viewPos");
//}

//let ambientLoc = raylib::get_shader_location(gamestate.shader.unwrap(), "ambient");
//raylib::set_shader_value(
//	gamestate.shader.unwrap(),
//	ambientLoc,
//	//[0.2f32,0.2f32,0.2f32,1f32].as_mut_ptr().cast(),
//	[3f32,3f32,3f32,1f32].as_mut_ptr().cast(),
//	raylib::ShaderUniformDataType::ShaderUniformVec4,
//);
//let fogDensity: f32 = 0.02;
//let fogDensityLoc = raylib::get_shader_location(gamestate.shader.unwrap(), "fogDensity");
//raylib::set_shader_value(
//	gamestate.shader.unwrap(),
//	fogDensityLoc,
//	[fogDensity].as_ptr().cast(),
//	raylib::ShaderUniformDataType::ShaderUniformFloat,
//);
//SetShaderValue(shader, shader.locs[SHADER_LOC_VECTOR_VIEW], &camera.position.x, SHADER_UNIFORM_VEC3);
//unsafe {
//	raylib::set_shader_value(
//		gamestate.shader.unwrap(),
//		*gamestate.shader.unwrap().locs.wrapping_add(raylib::ShaderLocationIndex::ShaderLocVectorView as usize),
//		[gamestate.camera.camPosition.x, gamestate.camera.camPosition.y, gamestate.camera.camPosition.z].as_ptr().cast(),
//		raylib::ShaderUniformDataType::ShaderUniformVec3,
//	);
//}