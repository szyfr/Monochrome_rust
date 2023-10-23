

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, fs::{read_to_string, File}, str::FromStr, fmt::Display, io::Write};
use crate::{utilities::debug, raylib};


//= Enumerations

/// Language for Localization
#[derive(Clone, Copy)]
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

/// Key origin
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


//= Structures

/// Storage for all settings
pub struct Settings {
	pub screenWidth : i32,
	pub screenHeight : i32,
	pub screenFps : i32,

	pub keybindings : Option<HashMap<String, Keybinding>>,

	pub language : Language,
}

/// Storage for individual keybindings
pub struct Keybinding {
	pub origin		: Origin,
	pub controller	: i32,
	pub code		: i32,
}


//= Procedures

impl Settings {
	/// Load settings from file.
	pub fn load(&mut self) {
		//* Attempt to load file */
		let mut fileResult = read_to_string("settings.json");
		if fileResult.is_err() {
			debug::log("[ERROR] - Failed to find settings file. Creating new file.\n");
			self.generate();
			fileResult = read_to_string("settings.json");
		}

		//* Convert to Json */
		let str = fileResult.unwrap();
		let jsonFile: serde_json::Value = serde_json::from_str(&str).unwrap();

		//* Read Json */
		self.screenWidth	= jsonFile["screen_width"].as_i64().unwrap() as i32;
		self.screenHeight	= jsonFile["screen_height"].as_i64().unwrap() as i32;
		self.screenFps		= jsonFile["screen_fps"].as_i64().unwrap() as i32;
		self.language		= Language::from_str(jsonFile["language"].as_str().unwrap()).unwrap();

		self.keybindings = Some(HashMap::new());
		for val in jsonFile["keybindings"].as_array().unwrap() {
			let name = val.as_array().unwrap()[0].as_str().unwrap();
			let info = val.as_array().unwrap()[1].as_array().unwrap();
			let kb = Keybinding{
				origin		: Origin::from_str(info[0].as_str().unwrap()).unwrap(),
				controller	: info[1].as_i64().unwrap() as i32,
				code		: info[2].as_i64().unwrap() as i32, 
			};
			self.keybindings.as_mut().unwrap().insert(name.to_string(), kb);
		}
	}

	/// Set settings to default values.
	fn generate(&mut self) {
		self.screenWidth	= 1280;
		self.screenHeight 	=  720;
		self.screenFps 		=   80;
		self.keybindings 	= Some(HashMap::new());
		self.language 		= Language::English;

		self.keybindings.as_mut().unwrap().insert("up".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 87 });
		self.keybindings.as_mut().unwrap().insert("down".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 83 });
		self.keybindings.as_mut().unwrap().insert("left".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 65 });
		self.keybindings.as_mut().unwrap().insert("right".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 68 });
		self.keybindings.as_mut().unwrap().insert("rotate_left".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 81 });
		self.keybindings.as_mut().unwrap().insert("rotate_right".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 69 });
		self.keybindings.as_mut().unwrap().insert("confirm".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 32 });

		self.save();
	}

	/// Save settings to file.
	fn save(&self) {
		let mut newSettingsFile : String = String::from("");
		let mut counter = 0;

		newSettingsFile.push_str("{\n");
		newSettingsFile.push_str(format!("\t\"screen_width\": {},\n", self.screenWidth).as_str());
		newSettingsFile.push_str(format!("\t\"screen_height\": {},\n", self.screenHeight).as_str());
		newSettingsFile.push_str(format!("\t\"screen_fps\": {},\n", self.screenFps).as_str());
		newSettingsFile.push_str(format!("\t\"language\": \"{}\",\n", self.language).as_str());
		newSettingsFile.push_str("\t\"keybindings\": [\n");
		for (str, key) in self.keybindings.as_ref().unwrap() {
			if counter == self.keybindings.as_ref().unwrap().len()-1 {
				newSettingsFile.push_str(format!("\t\t[\"{}\", [\"{}\",{},{}]]\n", str, key.origin, key.controller, key.code).as_str());
			} else {
				newSettingsFile.push_str(format!("\t\t[\"{}\", [\"{}\",{},{}]],\n", str, key.origin, key.controller, key.code).as_str());
			}

			counter += 1;
		}
		newSettingsFile.push_str("\t]\n");
		newSettingsFile.push_str("}");

		let newFile = File::create("settings.json");
		let _ = newFile.unwrap().write_all(newSettingsFile.as_bytes());
	}

	/// True if input ``k`` is valid and was pressed once.
	pub fn key_pressed(&self, k: &str) -> bool {
		//* Check for key */
		if !self.keybindings.is_some() && self.keybindings.as_ref().unwrap().contains_key(k) {
			debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.\n");
			print!("{}\n",k);
			return false;
		}

		//* Read key and test */
		let key = &self.keybindings.as_ref().unwrap()[k];
		match key.origin {
			Origin::Keyboard => return raylib::button_pressed(key.code),
			Origin::Mouse => return raylib::mouse_button_pressed(key.code),
			Origin::Controller => {
				if raylib::gamepad_available(key.controller) {
					return raylib::gamepad_button_pressed(key.code, key.controller);
				}
			},
		}
		return false;
	}
	/// True if input ``k`` is valid and was held down.
	pub fn key_down(&self, k: &str) -> bool {
		//* Check for key */
		if !self.keybindings.is_some() && self.keybindings.as_ref().unwrap().contains_key(k) {
			debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.\n");
			print!("{}\n",k);
			return false;
		}

		//* Read key and test */
		let key = &self.keybindings.as_ref().unwrap()[k];
		match key.origin {
			Origin::Keyboard => return raylib::button_down(key.code),
			Origin::Mouse => return raylib::mouse_button_down(key.code),
			Origin::Controller => {
				if raylib::gamepad_available(key.controller) {
					return raylib::gamepad_button_down(key.code, key.controller);
				}
			},
		}
		return false;
	}
	/// True if input ``k`` is valid and was let go.
	pub fn key_released(&self, k: &str) -> bool {
		//* Check for key */
		if !self.keybindings.is_some() && self.keybindings.as_ref().unwrap().contains_key(k) {
			debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.\n");
			print!("{}\n",k);
			return false;
		}

		//* Read key and test */
		let key = &self.keybindings.as_ref().unwrap()[k];
		match key.origin {
			Origin::Keyboard => return raylib::button_released(key.code),
			Origin::Mouse => return raylib::mouse_button_released(key.code),
			Origin::Controller => {
				if raylib::gamepad_available(key.controller) {
					return raylib::gamepad_button_released(key.code, key.controller);
				}
			},
		}
		return false;
	}
	/// True if input ``k`` is valid and hasn't been pressed.
	pub fn key_up(&self, k: &str) -> bool {
		//* Check for key */
		if !self.keybindings.is_some() && self.keybindings.as_ref().unwrap().contains_key(k) {
			debug::log("[WARNING] - Attempted to use a keybinding that wasn't mapped.\n");
			print!("{}\n",k);
			return false;
		}

		//* Read key and test */
		let key = &self.keybindings.as_ref().unwrap()[k];
		match key.origin {
			Origin::Keyboard => return raylib::button_up(key.code),
			Origin::Mouse => return raylib::mouse_button_up(key.code),
			Origin::Controller => {
				if raylib::gamepad_available(key.controller) {
					return raylib::gamepad_button_up(key.code, key.controller);
				}
			},
		}
		return false;
	}
}