

//= Allow
#![allow(non_snake_case)]
#![allow(non_snake_case)]


//= Imports
use std::collections::HashMap;
use crate::{settings, camera, player, overworld, world, graphics, localization};


//= Structs

/// Structure containing all current information on the games state.
pub struct Gamestate {
	pub localization : HashMap<String, String>,

	pub fonts		: HashMap<String, raylib_ffi::Font>,
	pub textures	: HashMap<String, raylib_ffi::Texture>,
	pub models		: HashMap<String, raylib_ffi::Model>,
	pub animations	: HashMap<String, overworld::Animation>,

	pub worldData	: world::World,

	pub camera		: camera::Camera,
	pub player		: player::Player,
}


//= Globals
pub static mut SETTINGS : settings::Settings = settings::Settings{
	screenWidth: 1280,
	screenHeight: 720,
	screenFps: 80,
	screenRatio: 1.0,
	text_speed: 5,
	keybindings: None,
	language: settings::Language::English,
};


//= Procedures

/// Creates a new gamestate from default values
pub fn init() -> Gamestate {
	let output = Gamestate{
		localization	: localization::load(),
		fonts			: HashMap::new(),
		textures		: HashMap::new(),
		models			: HashMap::new(),
		animations		: graphics::load_animations(),
		worldData		: world::init_empty(),
		camera			: camera::init(),
		player			: player::init(),
	};

	return output;
}

/// Non-unsafe button calls
pub fn key_pressed( k: &str ) -> bool {
	unsafe { return SETTINGS.key_pressed(k); }
}
pub fn key_down( k: &str ) -> bool {
	unsafe { return SETTINGS.key_down(k); }
}
pub fn key_released( k: &str ) -> bool {
	unsafe { return SETTINGS.key_released(k); }
}
pub fn key_up( k: &str ) -> bool {
	unsafe { return SETTINGS.key_up(k); }
}
/// Non-Unsafe variables
pub fn get_screenwidth() -> i32 {
	unsafe { return SETTINGS.screenWidth; }
}
pub fn get_screenheight() -> i32 {
	unsafe { return SETTINGS.screenHeight; }
}
pub fn get_screenfps() -> i32 {
	unsafe { return SETTINGS.screenFps; }
}
pub fn get_language() -> settings::Language {
	unsafe { return SETTINGS.language; }
}
pub fn get_screenratio() -> f32 {
	unsafe { return SETTINGS.screenRatio; }
}
pub fn get_textspeed() -> i32 {
	unsafe { return SETTINGS.text_speed; }
}