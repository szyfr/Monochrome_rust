

//= Allow
#![allow(non_snake_case)]


//= Imports
use std::collections::HashMap;
use crate::{settings, camera, player, overworld, world, events, graphics, localization};


//= Structs

/// Structure containing all current information on the games state.
pub struct Gamestate {
	pub settings : settings::Settings,

	pub localization : HashMap<String, String>,

	pub fonts		: HashMap<String, raylib_ffi::Font>,
	pub textures	: HashMap<String, raylib_ffi::Texture>,
	pub models		: HashMap<String, raylib_ffi::Model>,
	pub animations	: HashMap<String, overworld::Animation>,

	pub currentMap	: HashMap<[i32;3], world::Tile>,
	pub unitMap		: HashMap<String, overworld::Unit>,
	pub triggerMap	: HashMap<[i32;3], String>,
	pub eventList	: HashMap<String, events::Event>,
	pub eventHandler: events::EventHandler,

	pub camera		: camera::Camera,
	pub player		: player::Player,
}


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
		currentMap		: HashMap::new(),
		unitMap			: HashMap::new(),
		triggerMap		: HashMap::new(),
		eventList		: HashMap::new(),
		eventHandler	: events::create_eventhandler(),
		camera			: camera::init(),
		player			: player::init(),
	};
	output.localization = localization::load(&output.settings.language);

	return output;
}