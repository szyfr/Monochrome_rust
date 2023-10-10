

//= Allow
#![allow(non_snake_case)]


//= Imports
use std::collections::HashMap;
use crate::events;
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
	pub unitMap		: [Option<overworld::Unit>;20],
	pub unitTest : HashMap<String, overworld::Unit>,
	pub triggerMap	: HashMap<[i32;3], String>,
	pub eventList	: HashMap<String, events::Event>,
	pub eventHandler: events::EventHandler,

	pub camera		: camera::Camera,
	pub player		: player::Player,
}


//= Procedures