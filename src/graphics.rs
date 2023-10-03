

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{collections::HashMap, fs::read_dir};
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

pub fn load_models() -> HashMap<String, raylib_ffi::Model> {
	let mut output: HashMap<String, raylib_ffi::Model> = HashMap::new();
	let rawDirectory = read_dir("data/tiles/").unwrap();

	for i in rawDirectory {
		let mem = i.unwrap().path().clone();
		let str = mem.to_str().unwrap();
		if str.contains(".obj") {
			let mut name = str.to_string();
			name = name.replace(".obj", "");
			name = name.replace("data/tiles/", "");
			output.insert(name, raylib::load_model(str));
		}
	}

	return output;
}