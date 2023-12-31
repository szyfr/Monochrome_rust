

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

/// Difficulty
// TODO Improve
#[derive(Clone, Copy)]
pub enum Difficulty {
	Easy,
	Medium,
	Hard,
}
impl FromStr for Difficulty {
	type Err = ();
	fn from_str( input : &str ) -> Result<Difficulty, Self::Err> {
		match input {
			"easy"		=> Ok(Difficulty::Easy),
			"medium"	=> Ok(Difficulty::Medium),
			"hard"		=> Ok(Difficulty::Hard),
			_			=> Err(()),
		}
	}
}
impl Display for Difficulty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Difficulty::Easy => write!(f, "easy"),
			Difficulty::Medium => write!(f, "medium"),
			Difficulty::Hard => write!(f, "hard"),
		}
	}
}


//= Structures

/// Storage for all settings
pub struct Settings {

	//* Screen */
	pub screenWidth: i32,
	pub screenHeight: i32,
	pub screenFps: i32,
	pub screenRatio: f32,

	//* Game text */
	pub text_speed: i32,
	pub language: Language,

	//* Keybindings */
	pub keybindings: Option<HashMap<String, Keybinding>>,

	//* Audio */
	pub masterVolume: f32,
	pub musicVolume: f32,
	pub sfxVolume: f32,

	//* Game config */
	// TODO Seperate this into seperate structure?
	pub difficulty: Difficulty,
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
		self.screenRatio	= self.screenHeight as f32 / 720.0;
		self.language		= Language::from_str(jsonFile["language"].as_str().unwrap()).unwrap();

		self.text_speed		= jsonFile["text_speed"].as_i64().unwrap() as i32;

		self.masterVolume	= jsonFile["master"].as_f64().unwrap() as f32;
		self.musicVolume	= jsonFile["music"].as_f64().unwrap() as f32;
		self.sfxVolume		= jsonFile["sound"].as_f64().unwrap() as f32;

		self.difficulty		= Difficulty::from_str(jsonFile["difficulty"].as_str().unwrap()).unwrap();

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

		self.text_speed		=    4;
		self.language 		= Language::English;

		self.keybindings 	= Some(HashMap::new());
		self.difficulty		= Difficulty::Medium;

		self.keybindings.as_mut().unwrap().insert("up".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 87 });
		self.keybindings.as_mut().unwrap().insert("down".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 83 });
		self.keybindings.as_mut().unwrap().insert("left".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 65 });
		self.keybindings.as_mut().unwrap().insert("right".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 68 });
		self.keybindings.as_mut().unwrap().insert("rotate_left".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 81 });
		self.keybindings.as_mut().unwrap().insert("rotate_right".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 69 });
		self.keybindings.as_mut().unwrap().insert("confirm".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 32 });
		self.keybindings.as_mut().unwrap().insert("cancel".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 341 });
		self.keybindings.as_mut().unwrap().insert("enter".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 257 });
		self.keybindings.as_mut().unwrap().insert("attack_1".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 49 });
		self.keybindings.as_mut().unwrap().insert("attack_2".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 50 });
		self.keybindings.as_mut().unwrap().insert("attack_3".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 51 });
		self.keybindings.as_mut().unwrap().insert("attack_4".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 52 });
		self.keybindings.as_mut().unwrap().insert("swap_modes".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 69 });
		self.keybindings.as_mut().unwrap().insert("shift".to_string(), Keybinding { origin: Origin::Keyboard, controller: 0, code: 340 });

		self.masterVolume = 0.1;
		self.musicVolume = 0.2;
		self.sfxVolume = 0.2;

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
		newSettingsFile.push_str(format!("\t\"text_speed\": {},\n", self.text_speed).as_str());
		newSettingsFile.push_str(format!("\t\"language\": \"{}\",\n", self.language).as_str());
		newSettingsFile.push_str(format!("\t\"master\": {},\n", self.masterVolume).as_str());
		newSettingsFile.push_str(format!("\t\"music\": {},\n", self.musicVolume).as_str());
		newSettingsFile.push_str(format!("\t\"sound\": {},\n", self.sfxVolume).as_str());
		newSettingsFile.push_str(format!("\t\"difficulty\": \"{}\",\n", self.difficulty).as_str());
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

	/// Change screen resolution
	pub fn change_resolution(&mut self, width: i32, height: i32) {
		unsafe {
			self.screenWidth = width;
			self.screenHeight = height;
			self.screenRatio = height as f32 / 720.0;
			raylib_ffi::SetWindowSize(self.screenWidth, self.screenHeight);
			raylib_ffi::SetTextLineSpacing((self.screenRatio * 56.0) as i32);
		}
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