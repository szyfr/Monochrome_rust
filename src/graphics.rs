

//= Allows


//= Imports
use std::collections::HashMap;
use crate::raylib;


//= Procedures
pub fn load_textures() -> HashMap<String, raylib_ffi::Texture> {
	let mut output: HashMap<String, raylib_ffi::Texture> = HashMap::new();

	//* UI */
	output.insert("ui_textbox_general".to_string(), raylib::load_texture("data/sprites/ui/textbox.png"));
	output.insert("ui_pointer_general".to_string(), raylib::load_texture("data/sprites/ui/pointer.png"));

	return output;
}

pub fn load_fonts() -> HashMap<String, raylib_ffi::Font> {
	let mut output: HashMap<String, raylib_ffi::Font> = HashMap::new();

	//* Default */
	output.insert("default".to_string(), raylib::load_font("data/font.ttf"));

	return output;
}