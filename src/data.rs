

//= Allow
#![allow(non_snake_case)]
#![allow(non_snake_case)]


//= Imports
use std::collections::HashMap;
use crate::{settings, camera, player, overworld, world, graphics, localization, gSettings};


//= Structs

/// Structure containing all current information on the games state.
pub struct Gamestate {
	pub settings : settings::Settings,

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
pub static mut SETTINGS : gSettings::Settings = gSettings::Settings{
	screenWidth: 1280,
	screenHeight: 720,
	screenFps: 80,
	keybindings: None,
	language: gSettings::Language::English,
};


//= Procedures

/// Creates a new gamestate from default values
pub fn init() -> Gamestate {
	let mut output = Gamestate{
		settings		: settings::load(),
		localization	: HashMap::new(),
		fonts			: HashMap::new(),
		textures		: HashMap::new(),
		models			: HashMap::new(),
		animations		: graphics::load_animations(),
		worldData		: world::init_empty(),
		camera			: camera::init(),
		player			: player::init(),
	};
	output.localization = localization::load(&output.settings.language);

	return output;
}