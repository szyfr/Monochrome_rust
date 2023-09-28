
//= Imports
use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;

use crate::settings;


//= Globals
pub static SCREEN_WIDTH		: i32 = 1280;
pub static SCREEN_HEIGHT	: i32 = 720;
pub static SCREEN_FPS		: i32 = 80;

pub static KEYBINDINGS : Lazy<Mutex<HashMap<&str, settings::Keybinding>>> = Lazy::new(|| {
	let mut m = HashMap::new();
	m.insert("test", settings::Keybinding{ origin: settings::Origin::Keyboard, controller: 0, code: 0 });
	Mutex::new(m)
});

pub static LANGUAGE : settings::Language = settings::Language::English;