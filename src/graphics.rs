

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{collections::HashMap, fs::{read_dir, read_to_string}};

use crate::{raylib::{self, textures::Texture, images::Image, rectangles::Rectangle, fonts::Font, models::Model}, overworld::Animation, utilities::debug};


//= Structures
pub struct Graphics {
	pub fonts:		HashMap<String, Font>,
	pub textures:	HashMap<String, Texture>,
	pub models:		HashMap<String, Model>,
	pub animations:	HashMap<String, Animation>,

	pub shader:		Option<raylib_ffi::Shader>,
	pub timeLoc:	i32,
	pub sizeLoc:	i32,
	pub tileShader:	Option<raylib_ffi::Shader>,
	pub colorLoc:	i32,
}


//= Procedures

impl Graphics {

	/// Initialize
	pub fn init() -> Self {
		Self {
			fonts:		HashMap::new(),
			textures:	HashMap::new(),
			models:		HashMap::new(),
			animations:	HashMap::new(),
	
			shader:		None,
			timeLoc:	0,
			sizeLoc:	0,
			tileShader:	None,
			colorLoc:	0,
		}
	}

	/// Load all
	pub fn load(&mut self) {
		self.shader = Some(raylib::load_shader("", "data/shaders/lighting.fs"));
		self.timeLoc = raylib::get_shader_location(self.shader.unwrap(), "time");
		self.sizeLoc = raylib::get_shader_location(self.shader.unwrap(), "textureSize");
		self.tileShader = Some(raylib::load_shader("", "data/shaders/tile.fs"));
		self.colorLoc = raylib::get_shader_location(self.tileShader.unwrap(), "color");
		
		self.load_fonts();
		self.load_textures();
		self.load_models();
		self.load_animations();
	} 

	/// Load fonts
	pub fn load_fonts(&mut self) {
		//* Default */
		self.fonts.insert("default".to_string(), Font::load("data/font.ttf"));
	}

	/// Loads textures
	pub fn load_textures(&mut self) {
		//* World */
		self.textures.insert("terrain_texture".to_string(), Texture::load("data/tiles/texture_0.png"));

		//* Sprites */
		//* Player */
		let mut img = Image::load("data/sprites/overworld/player_1.png");
		for i in 0..img.width/img.height {
			let subimg = img.from_image(Rectangle { x: (16 * i) as f32, y: 0.0, width: 16.0, height: 16.0 });
			self.textures.insert("player_1_".to_string() + &i.to_string(), subimg.load_texture());
		}
		//* MonsterBall / placeholder */
		img = Image::load("data/sprites/overworld/ball.png");
		for i in 0..img.width/img.height {
			let subimg = img.from_image(Rectangle { x: (16 * i) as f32, y: 0.0, width: 16.0, height: 16.0 });
			self.textures.insert("ball_".to_string() + &i.to_string(), subimg.load_texture());
		}
		//* mon_152 */
		img = Image::load("data/sprites/battle/mon_152.png");
		for i in 0..img.width/img.height {
			let subimg = img.from_image(Rectangle { x: (32 * i) as f32, y: 0.0, width: 32.0, height: 32.0 });
			self.textures.insert("mon_152_".to_string() + &i.to_string(), subimg.load_texture());
		}
		//* mon_155 */
		img = Image::load("data/sprites/battle/mon_155.png");
		for i in 0..img.width/img.height {
			let subimg = img.from_image(Rectangle { x: (32 * i) as f32, y: 0.0, width: 32.0, height: 32.0 });
			self.textures.insert("mon_155_".to_string() + &i.to_string(), subimg.load_texture());
		}
		//* mon_158 */
		img = Image::load("data/sprites/battle/mon_158.png");
		for i in 0..img.width/img.height {
			let subimg = img.from_image(Rectangle { x: (32 * i) as f32, y: 0.0, width: 32.0, height: 32.0 });
			self.textures.insert("mon_158_".to_string() + &i.to_string(), subimg.load_texture());
		}

		//* UI */
		//* Textbox */
		img = Image::load("data/sprites/ui/textbox.png").resize_nn(4);
		self.textures.insert("ui_textbox_general".to_string(), img.load_texture());
		img.unload();
		//* Blackbox */
		img = Image::load("data/sprites/ui/blackbox.png").resize_nn(4);
		self.textures.insert("ui_blackbox_general".to_string(), img.load_texture());
		img.unload();
		//* Battle Status */
		img = Image::load("data/sprites/ui/battleStatus.png").resize_nn(3);
		self.textures.insert("ui_battlestatus_general".to_string(), img.load_texture());
		img.unload();

		self.textures.insert("ui_pointer_general".to_string(), Texture::load("data/sprites/ui/pointer.png"));
		self.textures.insert("ui_input_general".to_string(), Texture::load("data/sprites/ui/input.png"));
		self.textures.insert("bg_forest_day".to_string(), Texture::load("data/sprites/ui/background_forest.png"));

		//* Emotes */
		img = Image::load("data/sprites/ui/emotes.png");
		//* Shock */
		let shockImage = img.from_image(Rectangle::tex_rect(0, [16, 16]));
		self.textures.insert("emote_shock".to_string(), shockImage.load_texture());
		shockImage.unload();
		//* Confusion */
		let confusionImage = img.from_image(Rectangle::tex_rect(1, [16, 16]));
		self.textures.insert("emote_confusion".to_string(), confusionImage.load_texture());
		confusionImage.unload();
		//* Sad */
		let sadImage = img.from_image(Rectangle::tex_rect(2, [16, 16]));
		self.textures.insert("emote_sad".to_string(), sadImage.load_texture());
		sadImage.unload();

		//* Animations */
		//* flash */
		img = Image::load("data/sprites/animations/flash.png");
		for i in 0..3 {
			let subImg = img.from_image(Rectangle::tex_rect(i, [256,144]));
			self.textures.insert("ui_animation_flash_".to_string() + &i.to_string(), subImg.load_texture());
			subImg.unload();
		}
		img.unload();
		//* trainer_battle_1 */
		img = Image::load("data/sprites/animations/trainer_battle_1.png");
		for i in 0..3 {
			let subImg = img.from_image(Rectangle::tex_rect(i, [256,144]));
			self.textures.insert("ui_animation_trainer_battle_1_".to_string() + &i.to_string(), subImg.load_texture());
			subImg.unload();
		}
		img.unload();
	}

	/// Loads models
	pub fn load_models(&mut self) {
		let rawDirectory = read_dir("data/tiles/").unwrap();
		let texture = self.textures["terrain_texture"];

		for i in rawDirectory {
			let mem = i.unwrap().path().clone();
			let str = mem.to_str().unwrap();
			if str.contains(".obj") {
				let mut name = str.to_string();
				name = name.replace(".obj", "");
				name = name.replace("data/tiles/", "");

				let mut model = Model::load(str);
				model.set_material_texture(texture);
				if name == "battle_1".to_string() || name == "battle_2".to_string() || name == "battle_3".to_string() {
					unsafe { (*model.materials.wrapping_add(0)).shader = self.tileShader.unwrap(); }
				} else {
					unsafe { (*model.materials.wrapping_add(0)).shader = self.shader.unwrap(); }
				}
				
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

