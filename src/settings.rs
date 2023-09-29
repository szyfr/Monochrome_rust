

//= Allows
#![allow(non_snake_case)]


use std::clone;
use std::sync::Mutex;
//= Imports
use std::{fs::File, fs::read_to_string, io::copy, io::stdout, io::Write, str::FromStr};
use serde_json;

use crate::utilities::debug;
//use std::collections::HashMap;
use crate::data;


//= Structures / Enumeration
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


//= Procedures
pub fn init() {
	let mut file_result = read_to_string("settings.json");
	if file_result.is_err() {
		debug::log("[ERROR] - Failed to find settings file.\n");
		create_new_settings();
		file_result = read_to_string("settings.json");
	}
	
	let str = file_result.unwrap();
	let jsonFile: serde_json::Value = serde_json::from_str(&str).unwrap();
	
	//* Read JSON */
	unsafe {
		data::screenWidth	= jsonFile["screen_width"].as_i64().unwrap() as i32;
		data::screenHeight	= jsonFile["screen_height"].as_i64().unwrap() as i32;
		data::screenFps		= jsonFile["screen_fps"].as_i64().unwrap() as i32;
		data::language		= Language::from_str(jsonFile["language"].as_str().unwrap()).unwrap();
		
		let arr = jsonFile["keybindings"].as_array().unwrap().clone();
		for x in 0..arr.len() {
			
			let name = arr[x].as_array().unwrap()[0].as_str().unwrap();
			let info = arr[x].as_array().unwrap()[1].as_array().unwrap();
			let kb = Keybinding{
				origin		: Origin::from_str(info[0].as_str().unwrap()).unwrap(),
				controller	: info[1].as_i64().unwrap() as i32,
				code		: info[2].as_i64().unwrap() as i32,
			};
			let m = data::keybindings.get_mut().unwrap();
			m.insert( name, kb);
			//print!("{}\n",name)
			//m.insert(name, kb);
			//Mutex::new(m);
		}
	}
}

fn create_new_settings() {
	let mut newSettingsFile : String = String::from("");

	newSettingsFile.push_str("{\n");
	newSettingsFile.push_str("\t\"screen_width\": 1280,\n");
	newSettingsFile.push_str("\t\"screen_height\": 720,\n");
	newSettingsFile.push_str("\t\"screen_fps\": 80,\n");
	newSettingsFile.push_str("\t\"language\": \"english\",\n");
	newSettingsFile.push_str("\t\"keybindings\": [\n");
	newSettingsFile.push_str("\t\t\"up\": [0,0,87],\n");
	newSettingsFile.push_str("\t\t\"down\": [0,0,83],\n");
	newSettingsFile.push_str("\t\t\"left\": [0,0,67],\n");
	newSettingsFile.push_str("\t\t\"right\": [0,0,68],\n");
	newSettingsFile.push_str("\t\t\"rotate_left\": [0,0,81],\n");
	newSettingsFile.push_str("\t\t\"rotate_right\": [0,0,69],\n");
	newSettingsFile.push_str("\t\t\"confirm\": [0,0,65]\n");
	newSettingsFile.push_str("\t]\n");
	newSettingsFile.push_str("}");

	let newFile = File::create("settings.json");
	let _ = newFile.unwrap().write_all(newSettingsFile.as_bytes());
}