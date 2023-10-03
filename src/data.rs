

//= Imports
use std::collections::HashMap;
use crate::settings;
use crate::camera;


//= Structs
pub struct Gamestate {
	pub settings : settings::Settings,

	pub localization : HashMap<String, String>,

	pub fonts		: HashMap<String, raylib_ffi::Font>,
	pub textures	: HashMap<String, raylib_ffi::Texture>,

	pub camera		: camera::Camera,
}