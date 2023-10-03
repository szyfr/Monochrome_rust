

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use raylib_ffi::Vector3;

//= Imports
use crate::{data, camera::Camera};


//= Procedures
pub fn begin_drawing() {
	unsafe { raylib_ffi::BeginDrawing(); }
}

pub fn end_drawing() {
	unsafe { raylib_ffi::EndDrawing(); }
}

pub fn clear_background( color : raylib_ffi::Color ) {
	unsafe { raylib_ffi::ClearBackground(color); }
}

pub fn draw_text( text : *const std::os::raw::c_char, posX : i32, posY : i32, fontSize : i32, color : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawText(text, posX, posY, fontSize, color); }
}

pub fn window_should_close() -> bool {
	unsafe { return raylib_ffi::WindowShouldClose(); }
}

pub fn set_trace_log_level( logLevel : raylib_ffi::enums::TraceLogLevel ) {
	unsafe { raylib_ffi::SetTraceLogLevel(logLevel as i32); }
}

pub fn init_window( gamestate : &data::Gamestate ) {
	unsafe {
		raylib_ffi::InitWindow(
			gamestate.settings.screenWidth,
			gamestate.settings.screenHeight,
			raylib_ffi::rl_str!(gamestate.localization["title"]),
		);
	}
}

pub fn set_target_fps( fps : i32 ) {
	unsafe { raylib_ffi::SetTargetFPS(fps); }
}

pub fn set_exit_key( key : raylib_ffi::enums::KeyboardKey ) {
	unsafe { raylib_ffi::SetExitKey(key as i32); }
}

pub fn close_window() {
	unsafe { raylib_ffi::CloseWindow(); }
}

pub fn load_font( filename : &str ) -> raylib_ffi::Font {
	unsafe { return raylib_ffi::LoadFont(raylib_ffi::rl_str!(filename)); }
}

pub fn load_texture( filename : &str ) -> raylib_ffi::Texture {
	unsafe { return raylib_ffi::LoadTexture(raylib_ffi::rl_str!(filename)) }
}

pub fn draw_text_pro( font : raylib_ffi::Font, text : &str, position : raylib_ffi::Vector2, origin : raylib_ffi::Vector2, rotation : f32, fontSize : f32, spacing : f32, tint : raylib_ffi::Color ) {
	unsafe {
		raylib_ffi::DrawTextPro(
			font,
			raylib_ffi::rl_str!(text),
			position,
			origin,
			rotation,
			fontSize,
			spacing,
			tint,
		);
	}
}

pub fn get_frame_time() -> f32 {
	unsafe { return raylib_ffi::GetFrameTime(); }
}

pub fn begin_3d_mode( camera : &Camera ) {
	unsafe {
		let rlCamera = raylib_ffi::Camera3D{
			position:	camera.position,
			target:		camera.target,
			up:			Vector3{x:0.0,y:1.0,z:0.0},
			fovy:		camera.fovy,
			projection:	raylib_ffi::enums::CameraProjection::Perspective as i32,
		};

		raylib_ffi::BeginMode3D(rlCamera);
	}
}

pub fn end_3d_mode() {
	unsafe { raylib_ffi::EndMode3D(); }
}

pub fn draw_grid( slices : i32, spacing : f32 ) {
	unsafe { raylib_ffi::DrawGrid(slices, spacing); }
}