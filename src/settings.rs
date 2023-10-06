

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, fs::{read_to_string, File}, str::FromStr, io::Write, fmt::Display};
use crate::{utilities::debug, raylib};


//= Structures
pub struct Settings {
	pub screenWidth : i32,
	pub screenHeight : i32,
	pub screenFps : i32,

	pub keybindings : HashMap<String, Keybinding>,

	pub language : Language,
}
pub struct Keybinding {
	pub origin		: Origin,
	pub controller	: i32,
	pub code		: i32,
}
pub enum Origin {
	Keyboard,
	Mouse,
	Controller,
}
impl FromStr for Origin {
	type Err = ();
	fn from_str( input : &str ) -> Result<Origin, Self::Err> {
		match input {
			"keyboard"		=> Ok(Origin::Keyboard),
			"mouse"			=> Ok(Origin::Mouse),
			"controller"	=> Ok(Origin::Controller),
			_				=> Err(()),
		}
	}
}
impl Display for Origin {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Origin::Keyboard => write!(f, "keyboard"),
			Origin::Mouse => write!(f, "mouse"),
			Origin::Controller => write!(f, "controller"),
		}
	}
}
pub enum Language {
	English,
	French,
	German,
}
impl FromStr for Language {
	type Err = ();
	fn from_str( input : &str ) -> Result<Language, Self::Err> {
		match input {
			"english"	=> Ok(Language::English),
			"french"	=> Ok(Language::French),
			"german"	=> Ok(Language::German),
			_			=> Err(()),
		}
	}
}
impl Display for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Language::English => write!(f, "english"),
			Language::French => write!(f, "french"),
			Language::German => write!(f, "german"),
		}
	}
}


//= Procedures
pub fn load() -> Settings {
	let mut output = Settings{
		screenWidth		: 0,
		screenHeight 	: 0,
		screenFps 		: 0,
		keybindings 	: HashMap::new(),
		language 		: Language::English,
	};

	//* Attempt to load file */
	let fileResult = read_to_string("settings.json");
	if fileResult.is_err() {
		debug::log("[ERROR] - Failed to find settings file. Creating new file.\n");
		output = generate_settings();
		return output;
	}

	//* Convert to Json */
	let str = fileResult.unwrap();
	let jsonFile: serde_json::Value = serde_json::from_str(&str).unwrap();

	//* Read Json */
	output.screenWidth	= jsonFile["screen_width"].as_i64().unwrap() as i32;
	output.screenHeight = jsonFile["screen_height"].as_i64().unwrap() as i32;
	output.screenFps	= jsonFile["screen_fps"].as_i64().unwrap() as i32;
	output.language		= Language::from_str(jsonFile["language"].as_str().unwrap()).unwrap();

	for val in jsonFile["keybindings"].as_array().unwrap() {
		let name = val.as_array().unwrap()[0].as_str().unwrap();
		let info = val.as_array().unwrap()[1].as_array().unwrap();
		let kb = Keybinding{
			origin		: Origin::from_str(info[0].as_str().unwrap()).unwrap(),
			controller	: info[1].as_i64().unwrap() as i32,
			code		: info[2].as_i64().unwrap() as i32, 
		};
		output.keybindings.insert(name.to_string(), kb);
	}

	return output;
}

fn generate_settings() -> Settings {
	let mut newSettings = Settings{
		screenWidth		: 1280,
		screenHeight 	:  720,
		screenFps 		:   80,
		keybindings 	: HashMap::new(),
		language 		: Language::English,
	};
	
	newSettings.keybindings.insert("up".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 87 });
	newSettings.keybindings.insert("down".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 83 });
	newSettings.keybindings.insert("left".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 65 });
	newSettings.keybindings.insert("right".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 68 });
	newSettings.keybindings.insert("rotate_left".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 81 });
	newSettings.keybindings.insert("rotate_right".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 69 });
	newSettings.keybindings.insert("confirm".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 32 });

	save_file(&newSettings);

	return newSettings;
}

fn save_file( settings : &Settings ) {
	let mut newSettingsFile : String = String::from("");
	let mut counter = 0;

	newSettingsFile.push_str("{\n");
	newSettingsFile.push_str(format!("\t\"screen_width\": {},\n", settings.screenWidth).as_str());
	newSettingsFile.push_str(format!("\t\"screen_height\": {},\n", settings.screenHeight).as_str());
	newSettingsFile.push_str(format!("\t\"screen_fps\": {},\n", settings.screenFps).as_str());
	newSettingsFile.push_str(format!("\t\"language\": \"{}\",\n", settings.language).as_str());
	newSettingsFile.push_str("\t\"keybindings\": [\n");
	for key in &settings.keybindings {
		if counter == settings.keybindings.len()-1 {
			newSettingsFile.push_str(format!("\t\t[\"{}\", [\"{}\",{},{}]]\n", key.0, key.1.origin, key.1.controller,key.1.code).as_str());
		} else {
			newSettingsFile.push_str(format!("\t\t[\"{}\", [\"{}\",{},{}]],\n", key.0, key.1.origin, key.1.controller,key.1.code).as_str());
		}
		
		counter += 1;
	}
	newSettingsFile.push_str("\t]\n");
	newSettingsFile.push_str("}");

	let newFile = File::create("settings.json");
	let _ = newFile.unwrap().write_all(newSettingsFile.as_bytes());
}

pub fn button_pressed( key : &str, settings : &Settings ) -> bool {
	//* Leave if key not found */
	if !HashMap::contains_key(&settings.keybindings, key) {
		debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.");
		return false;
	}

	let key = &settings.keybindings[key];

	match key.origin {
		Origin::Keyboard	=> return raylib::button_pressed(key.code),
		Origin::Mouse		=> return raylib::mouse_button_pressed(key.code),
		Origin::Controller	=> if raylib::gamepad_available(key.controller) { return raylib::gamepad_button_pressed(key.code, key.controller) },
	}

	return false;
}
pub fn button_down( key : &str, settings : &Settings ) -> bool {
	//* Leave if key not found */
	if !HashMap::contains_key(&settings.keybindings, key) {
		debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.");
		return false;
	}

	let key = &settings.keybindings[key];

	match key.origin {
		Origin::Keyboard	=> return raylib::button_down(key.code),
		Origin::Mouse		=> return raylib::mouse_button_down(key.code),
		Origin::Controller	=> if raylib::gamepad_available(key.controller) { return raylib::gamepad_button_down(key.code, key.controller) },
	}

	return false;
}
pub fn button_released( key : &str, settings : &Settings ) -> bool {
	//* Leave if key not found */
	if !HashMap::contains_key(&settings.keybindings, key) {
		debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.");
		return false;
	}

	let key = &settings.keybindings[key];

	match key.origin {
		Origin::Keyboard	=> return raylib::button_released(key.code),
		Origin::Mouse		=> return raylib::mouse_button_released(key.code),
		Origin::Controller	=> if raylib::gamepad_available(key.controller) { return raylib::gamepad_button_released(key.code, key.controller) },
	}

	return false;
}
pub fn button_up( key : &str, settings : &Settings ) -> bool {
	//* Leave if key not found */
	if !HashMap::contains_key(&settings.keybindings, key) {
		debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.");
		return false;
	}

	let key = &settings.keybindings[key];

	match key.origin {
		Origin::Keyboard	=> return raylib::button_up(key.code),
		Origin::Mouse		=> return raylib::mouse_button_up(key.code),
		Origin::Controller	=> if raylib::gamepad_available(key.controller) { return raylib::gamepad_button_up(key.code, key.controller) },
	}

	return false;
}