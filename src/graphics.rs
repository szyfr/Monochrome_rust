

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{collections::HashMap, fs::{read_dir, read_to_string}};
use crate::{raylib, overworld::Animation, utilities::debug};


//= Procedures

/// Loads all necessary textures into Hashmap indexed by their names.
pub fn load_textures() -> HashMap<String, raylib_ffi::Texture> {
	let mut output: HashMap<String, raylib_ffi::Texture> = HashMap::new();

	//* UI */
	let mut img = raylib::load_image("data/sprites/ui/textbox.png");
	raylib::image_resize_nn(&mut img, 4);
	output.insert("ui_textbox_general".to_string(), raylib::load_texture_from_image(img));
	raylib::unload_image(img);

	output.insert("ui_pointer_general".to_string(), raylib::load_texture("data/sprites/ui/pointer.png"));
	output.insert("ui_input_general".to_string(), raylib::load_texture("data/sprites/ui/input.png"));

	img = raylib::load_image("data/sprites/animations/flash.png");
	for i in 0..3 {
		let subImg = raylib::image_from_image(
			img,
			raylib_ffi::Rectangle{
				x: (i * img.width/3) as f32,
				y: 0.0,
				width: (img.width/3) as f32,
				height: img.height as f32,
			}
		);
		output.insert("ui_animation_flash_".to_string() + &i.to_string(), raylib::load_texture_from_image(subImg));
	}
	raylib::unload_image(img);

	img = raylib::load_image("data/sprites/animations/trainer_battle_1.png");
	for i in 0..3 {
		let subImg = raylib::image_from_image(
			img,
			raylib_ffi::Rectangle{
				x: (i * img.width/3) as f32,
				y: 0.0,
				width: (img.width/3) as f32,
				height: img.height as f32,
			}
		);
		output.insert("ui_animation_trainer_battle_1_".to_string() + &i.to_string(), raylib::load_texture_from_image(subImg));
	}
	raylib::unload_image(img);

	return output;
}

/// Loads all necessary fonts into Hashmap indexed by their names.
pub fn load_fonts() -> HashMap<String, raylib_ffi::Font> {
	let mut output: HashMap<String, raylib_ffi::Font> = HashMap::new();

	//* Default */
	output.insert("default".to_string(), raylib::load_font("data/font.ttf"));

	return output;
}

/// Loads all necessary models into Hashmap indexed by their names.
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
			output.insert(name.to_string(), raylib::load_model(str));
			print!("{}\n",name.to_string());
		}
	}

	return output;
}

/// Loads all necessary animations into Hashmap indexed by their names.
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