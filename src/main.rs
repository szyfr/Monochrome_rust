
//use std::collections::HashMap;

//use data;
use monorust::data;
use monorust::settings;


fn update() {

}
fn draw() {
	unsafe {
		raylib_ffi::draw!({
			raylib_ffi::ClearBackground(raylib_ffi::colors::WHITE);
			raylib_ffi::DrawText(
				raylib_ffi::rl_str!("FUCK!"),
				0,
				0,
				20,
				raylib_ffi::colors::BLACK,
			);
		});
	}
}
fn main() {
	init();
	unsafe {
		while !(raylib_ffi::WindowShouldClose()) {
			update();
			draw();
		}
	}
	close();
}

fn init() {
	//* Debug *//

	//* Settings / Localization *//
	settings::init();

	//* Raylib *//
	unsafe {
		raylib_ffi::SetTraceLogLevel(raylib_ffi::enums::TraceLogLevel::None as i32);
		raylib_ffi::InitWindow(
			data::SCREEN_WIDTH,
			data::SCREEN_HEIGHT,
			raylib_ffi::rl_str!("Monochrome")
		);
		if data::SCREEN_FPS != 0 { raylib_ffi::SetTargetFPS(data::SCREEN_FPS); }
		raylib_ffi::SetExitKey(raylib_ffi::enums::KeyboardKey::Null as i32);
	}
	
	//* Graphics *//

	//* Camera / Player *//

	// ! TEMP
}

fn close() {
	unsafe {
		raylib_ffi::CloseWindow();
	}
}