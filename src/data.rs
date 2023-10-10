

//= Allow
#![allow(non_snake_case)]


//= Imports
use std::collections::HashMap;
use crate::settings;
use crate::camera;
use crate::player;
use crate::overworld;
use crate::world;


//= Structs
pub struct Gamestate {
	pub settings : settings::Settings,

	pub localization : HashMap<String, String>,

	pub fonts		: HashMap<String, raylib_ffi::Font>,
	pub textures	: HashMap<String, raylib_ffi::Texture>,
	pub models		: HashMap<String, raylib_ffi::Model>,
	pub animations	: HashMap<String, overworld::Animation>,

	pub currentMap	: HashMap<[i32;3], world::Tile>,
	pub unitMap		: HashMap<String, overworld::Unit>,

	pub camera		: camera::Camera,
	pub player		: player::Player,
}


//= Procedures