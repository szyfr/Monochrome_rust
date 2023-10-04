

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{collections::HashMap, fs::{read_dir, read_to_string}};
use crate::{raylib, overworld::Animation, utilities::debug};


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

pub fn load_animations() -> HashMap<String, Animation> {
	let mut output: HashMap<String, Animation> = HashMap::new();

	//* Attempt to load file */
	let fileResult = read_to_string("data/sprites/overworld/animations.json");
	if fileResult.is_err() {
		debug::log("[ERROR] - Failed to load animations file.\n");
		return output;
	}

	//* Convert to Json and read */
	let jsonFile: serde_json::Value = serde_json::from_str(&fileResult.unwrap()).unwrap();
	for i in jsonFile.as_object().unwrap() {
		let mut ani = Animation{
			frames:	Vec::new(),
			delay:	i.1[0].as_i64().unwrap() as i32,
		};
		for b in i.1[1].as_array().unwrap() {
			ani.frames.push(b.as_i64().unwrap() as i32);
		}
		output.insert(i.0.to_string(), ani);
	}

	return output;
}