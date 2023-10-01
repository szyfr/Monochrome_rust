

//= Imports
use std::collections::HashMap;
use crate::settings;


//= Structs
pub struct Gamestate {
	pub settings : settings::Settings,

	pub localization : HashMap<String, String>,
}