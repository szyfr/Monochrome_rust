

//= Imports
use std::{fs::File, io::copy, io::stdout};
use serde_json;
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
	let file_result = File::open("settings.json");
	assert!(!file_result.is_err());
	
	let mut stdout = stdout();
	let str = &copy(&mut file_result.unwrap(), &mut stdout).unwrap().to_string();
	let json: serde_json::Value = serde_json::from_str(str).unwrap();
	print!("{0}\n", json);
}