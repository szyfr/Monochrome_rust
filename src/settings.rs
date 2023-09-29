

//= Allows
#![allow(non_snake_case)]


//= Imports
use std::{fs::File, io::copy, io::stdout, io::Write};
use serde_json;

use crate::utilities::debug;
//use std::collections::HashMap;
//use crate::data;


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
pub enum Language {
	English,
	French,
	German,
}


//= Procedures
pub fn init() {
	let mut file_result = File::open("settings.json");
	if file_result.is_err() {
		debug::log("[ERROR] - Failed to find settings file.\n");
		create_new_settings();
		file_result = File::open("settings.json");
	}
	
	let mut stdout = stdout();
	let str = &copy(&mut file_result.unwrap(), &mut stdout).unwrap().to_string();
	let _: serde_json::Value = serde_json::from_str(str).unwrap();
	
	//* Read JSON */
	//print!("{0}", json["language"]);
}

fn create_new_settings() {
	let mut newSettingsFile : String = String::from("");

	newSettingsFile.push_str("{\n");
	newSettingsFile.push_str("\t\"screen_width\": 1280,\n");
	newSettingsFile.push_str("\t\"screen_height\": 720,\n");
	newSettingsFile.push_str("\t\"language\": \"english\",\n");
	newSettingsFile.push_str("\t\"keybindings\": {\n");
	newSettingsFile.push_str("\t\t\"up\": [0,0,87],\n");
	newSettingsFile.push_str("\t\t\"down\": [0,0,83],\n");
	newSettingsFile.push_str("\t\t\"left\": [0,0,67],\n");
	newSettingsFile.push_str("\t\t\"right\": [0,0,68],\n");
	newSettingsFile.push_str("\t\t\"rotate_left\": [0,0,81],\n");
	newSettingsFile.push_str("\t\t\"rotate_right\": [0,0,69],\n");
	newSettingsFile.push_str("\t\t\"confirm\": [0,0,65],\n");
	newSettingsFile.push_str("\t},\n");
	newSettingsFile.push_str("}");

	let newFile = File::create("settings.json");
	let _ = newFile.unwrap().write_all(newSettingsFile.as_bytes());
}