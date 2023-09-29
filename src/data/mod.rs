

//= Allows
#![allow(non_upper_case_globals)]


//= Imports
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

use crate::settings;


//= Globals
pub static mut screenWidth	: i32 = 200;
pub static mut screenHeight	: i32 = 100;
pub static mut screenFps	: i32 = 80;

pub static mut keybindings : Lazy<Mutex<HashMap<&str, settings::Keybinding>>> = Lazy::new(|| {
	let mut m = HashMap::new();
	m.insert("test", settings::Keybinding{ origin: settings::Origin::Keyboard, controller: 10, code: 10 });
	Mutex::new(m)
});

pub static mut language : settings::Language = settings::Language::English;