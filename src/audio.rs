

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{raylib, data};


//= Enumerations


//= Structures

/// Audio structure
pub struct Audio{
	currentMusicName: String,
	currentMusic: Option<raylib_ffi::Music>,
	currentSoundName: String,
	currentSound: Option<raylib_ffi::Sound>,
} 


//= Procedures

impl Audio {

	/// Default init
	pub fn init() -> Self {
		Self {
			currentMusicName: "".to_string(),
			currentMusic: None,
			currentSoundName: "".to_string(),
			currentSound: None,
		}
	}

	/// Play sound
	pub fn play_sound( &mut self, sound: String ) {
		if sound == self.currentSoundName { return }

		if self.currentSound.is_some() {
			raylib::stop_sound(self.currentSound.unwrap());
			raylib::unload_sound(self.currentSound.unwrap());
		}
		self.currentSoundName = sound.to_string();
		self.currentSound = Some(raylib::load_sound(&("data/audio/sfx/".to_string() + &sound + ".wav")));
		raylib::set_sound_volume(self.currentSound.unwrap(), data::get_master_volume() * data::get_sfx_volume());
		raylib::play_sound(self.currentSound.unwrap());
	}

	/// Play music
	pub fn play_music( &mut self, music: String ) {
		if music == self.currentMusicName { return }

		if self.currentMusic.is_some() {
			raylib::stop_music(self.currentMusic.unwrap());
			raylib::unload_music(self.currentMusic.unwrap());
		}
		self.currentMusicName = music.to_string();
		self.currentMusic = Some(raylib::load_music(&("data/audio/music/".to_string() + &music + ".wav")));
		raylib::set_music_volume(self.currentMusic.unwrap(), data::get_master_volume() * data::get_music_volume());
		raylib::play_music(self.currentMusic.unwrap());
	}
	/// Pause / Resume music
	pub fn pause_music( &self ) {
		if self.currentMusic.is_some() {
			raylib::set_music_volume(self.currentMusic.unwrap(), data::get_master_volume() * data::get_music_volume());
			if raylib::is_music_playing(self.currentMusic.unwrap()) { raylib::pause_music(self.currentMusic.unwrap()); }
			else { raylib::play_music(self.currentMusic.unwrap()); }
		}
	}
	/// Update music
	pub fn update( &mut self ) {
		if self.currentMusic.is_some() { raylib::update_music(self.currentMusic.unwrap()); }
		if self.currentSoundName != "" && !raylib::is_sound_playing(self.currentSound.unwrap()) {
			self.currentSoundName = "".to_string();
			raylib::unload_sound(self.currentSound.unwrap());
			self.currentSound = None;
		}
	}

}
