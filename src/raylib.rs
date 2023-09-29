

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]

//= Imports


//= Procedures
pub fn begin_drawing() {
	unsafe {
		raylib_ffi::BeginDrawing();
	}
}

pub fn end_drawing() {
	unsafe {
		raylib_ffi::EndDrawing();
	}
}

pub fn clear_background( color : raylib_ffi::Color ) {
	unsafe {
		raylib_ffi::ClearBackground(color);
	}
}

pub fn draw_text( text : *const std::os::raw::c_char, posX : i32, posY : i32, fontSize : i32, color : raylib_ffi::Color ) {
	unsafe {
		raylib_ffi::DrawText(text, posX, posY, fontSize, color);
	}
}

pub fn window_should_close() -> bool {
	unsafe {
		return raylib_ffi::WindowShouldClose();
	}
}

pub fn set_trace_log_level( logLevel : raylib_ffi::enums::TraceLogLevel ) {
	unsafe {
		raylib_ffi::SetTraceLogLevel(logLevel as i32);
	}
}

pub fn init_window( width : i32, height : i32, title : &str ) {
	unsafe {
		raylib_ffi::InitWindow(
			width,
			height,
			raylib_ffi::rl_str!(title),
		);
	}
}

pub fn set_target_fps( fps : i32 ) {
	unsafe {
		raylib_ffi::SetTargetFPS(fps);
	}
}

pub fn set_exit_key( key : raylib_ffi::enums::KeyboardKey ) {
	unsafe {
		raylib_ffi::SetExitKey(key as i32);
	}
}

pub fn close_window() {
	unsafe {
		raylib_ffi::CloseWindow();
	}
}