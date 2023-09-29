

//= Allows
#![allow(non_upper_case_globals)]


//= Imports
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

use crate::settings;


//= Globals
pub static screenWidth	: i32 = 1280;
pub static screenHeight	: i32 = 720;
pub static screenFps	: i32 = 80;

pub static keybindings : Lazy<Mutex<HashMap<&str, settings::Keybinding>>> = Lazy::new(|| {
	let mut m = HashMap::new();
	m.insert("test", settings::Keybinding{ origin: settings::Origin::Keyboard, controller: 0, code: 0 });
	Mutex::new(m)
});

pub static language : settings::Language = settings::Language::English;