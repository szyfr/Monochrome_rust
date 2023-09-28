

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