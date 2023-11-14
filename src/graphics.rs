

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{collections::HashMap, fs::{read_dir, read_to_string}};
use raylib_ffi::Font;
use raylib_ffi::Texture;
use raylib_ffi::Model;

use crate::{raylib, overworld::Animation, utilities::debug};


//= Structures
pub struct Graphics {
	pub fonts:		HashMap<String, Font>,
	pub textures:	HashMap<String, Texture>,
	pub models:		HashMap<String, Model>,
	pub animations:	HashMap<String, Animation>,

	pub shader:		Option<raylib_ffi::Shader>,
	pub timeLoc:	i32,
	pub sizeLoc:	i32,
}


//= Procedures

impl Graphics {
	/// Load all
	pub fn load(&mut self) {
		self.shader = Some(raylib::load_shader("", "data/shaders/lighting.fs"));
		self.timeLoc = raylib::get_shader_location(self.shader.unwrap(), "time");
		self.sizeLoc = raylib::get_shader_location(self.shader.unwrap(), "textureSize");
		
		self.load_fonts();
		self.load_textures();
		self.load_models();
		self.load_animations();
	} 

	/// Load fonts
	pub fn load_fonts(&mut self) {
		//* Default */
		self.fonts.insert("default".to_string(), raylib::load_font("data/font.ttf"));
	}

	/// Loads textures
	pub fn load_textures(&mut self) {
		//* UI */
		let mut img = raylib::load_image("data/sprites/ui/textbox.png");
		raylib::image_resize_nn(&mut img, 4);
		self.textures.insert("ui_textbox_general".to_string(), raylib::load_texture_from_image(img));
		raylib::unload_image(img);

		self.textures.insert("ui_pointer_general".to_string(), raylib::load_texture("data/sprites/ui/pointer.png"));
		self.textures.insert("ui_input_general".to_string(), raylib::load_texture("data/sprites/ui/input.png"));
		self.textures.insert("bg_forest_day".to_string(), raylib::load_texture("data/sprites/overworld/background_forest.png"));

		//* Emotes */
		img = raylib::load_image("data/sprites/ui/emotes.png");
		//* Shock */
		let mut subImg_emote = raylib::image_from_image(
			img,
			raylib_ffi::Rectangle{ x: 0.0, y: 0.0, width: 16.0, height: 16.0 }
		);
		self.textures.insert("emote_shock".to_string(), raylib::load_texture_from_image(subImg_emote));
		raylib::unload_image(subImg_emote);
		//* Confusion */
		subImg_emote = raylib::image_from_image(
			img,
			raylib_ffi::Rectangle{ x: 16.0, y: 0.0, width: 16.0, height: 16.0 }
		);
		self.textures.insert("emote_confusion".to_string(), raylib::load_texture_from_image(subImg_emote));
		raylib::unload_image(subImg_emote);
		//* Sad */
		subImg_emote = raylib::image_from_image(
			img,
			raylib_ffi::Rectangle{ x: 32.0, y: 0.0, width: 16.0, height: 16.0 }
		);
		self.textures.insert("emote_sad".to_string(), raylib::load_texture_from_image(subImg_emote));
		raylib::unload_image(subImg_emote);

		//* Animations */
		//* flash */
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
			self.textures.insert("ui_animation_flash_".to_string() + &i.to_string(), raylib::load_texture_from_image(subImg));
		}
		raylib::unload_image(img);

		//* trainer_battle_1 */
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
			self.textures.insert("ui_animation_trainer_battle_1_".to_string() + &i.to_string(), raylib::load_texture_from_image(subImg));
		}
		raylib::unload_image(img);
	}

	/// Loads models
	pub fn load_models(&mut self) {
		let rawDirectory = read_dir("data/tiles/").unwrap();

		for i in rawDirectory {
			let mem = i.unwrap().path().clone();
			let str = mem.to_str().unwrap();
			if str.contains(".obj") {
				let mut name = str.to_string();
				name = name.replace(".obj", "");
				name = name.replace("data/tiles/", "");
				let model = raylib::load_model(str);
				unsafe { (*model.materials.wrapping_add(0)).shader = self.shader.unwrap(); }
				self.models.insert(name.to_string(), model);
			}
		}
	}

	/// Loads animations
	pub fn load_animations(&mut self) {
		//* Attempt to load file */
		let fileResult = read_to_string("data/sprites/overworld/animations.json");
		if fileResult.is_err() {
			debug::log("[ERROR] - Failed to load animations file.\n");
			return;
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
			self.animations.insert(i.0.to_string(), ani);
		}
	}
}

/// Initialize
pub fn init() -> Graphics {
	return Graphics {
		fonts:		HashMap::new(),
		textures:	HashMap::new(),
		models:		HashMap::new(),
		animations:	HashMap::new(),

		shader:		None,
		timeLoc:	0,
		sizeLoc:	0,
	};
}
