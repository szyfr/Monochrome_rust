

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::fmt::Display;
use crate::{data, raylib};
use super::EventChain;


//= Enumerations
#[derive(PartialEq)]
pub enum TextboxState{
	Inactive,
	Active,
	Finished,
	Reset,
}
impl Display for TextboxState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			TextboxState::Inactive	=> write!(f, "inactive"),
			TextboxState::Active	=> write!(f, "active"),
			TextboxState::Finished	=> write!(f, "finished"),
			TextboxState::Reset		=> write!(f, "reset"),
		}
	}
}


//= Structures

/// Contains all info for Textbox
pub struct Textbox{
	pub state: TextboxState,
	
	pub currentText: String,
	pub targetText:	 String,

	pub timer: i32,
	pub pause: i32,
	pub position: i32,

	pub hasChoice: bool,
	pub choiceList: Vec<Choice>,
	pub curPosition: i32,
}
impl Textbox {
	/// Resets the textbox to it's default state.
	pub fn reset(&mut self) {
		self.state = TextboxState::Inactive;

		self.currentText = "".to_string();
		self.targetText = "".to_string();

		self.timer = 0;
		self.pause = 0;
		self.position = 0;

		self.hasChoice = false;
		self.choiceList = Vec::new();
		self.curPosition = 0;
	}
}

/// Choices structure
pub struct Choice{
	pub text:  String,
	pub event: String,
}


//= Procedures

/// Creates initial Textbox state
pub fn init() -> Textbox {
	return Textbox {
		state: TextboxState::Inactive,

		currentText: "".to_string(),
		targetText: "".to_string(),

		timer: 0,
		pause: 0,
		position: 0,

		hasChoice: false,
		choiceList: Vec::new(),
		curPosition: 0,
	}
}

/// Sets the textbox to start.
pub fn run( gamestate : &mut data::Gamestate, text : String ) -> bool {
	match gamestate.worldData.eventHandler.textbox.state {
		TextboxState::Inactive => {
			//* If the Textbox is currently inactive, start it up */
			gamestate.worldData.eventHandler.textbox.state = TextboxState::Active;
			gamestate.worldData.eventHandler.textbox.currentText = "".to_string();
			gamestate.worldData.eventHandler.textbox.targetText = gamestate.localization[&text.to_string()].to_string();
			gamestate.worldData.eventHandler.textbox.timer = 0;
			gamestate.worldData.eventHandler.textbox.pause = 0;
			gamestate.worldData.eventHandler.textbox.position = 1;
			gamestate.worldData.eventHandler.textbox.hasChoice = false;
			gamestate.worldData.eventHandler.textbox.choiceList = Vec::new();
			gamestate.worldData.eventHandler.textbox.curPosition = 0;

			return false;
		},
		TextboxState::Active => {
			//* Increase timer */
			// TODO timer amount
			gamestate.worldData.eventHandler.textbox.timer += 1;
			print!("fuck1\n");
			if gamestate.worldData.eventHandler.textbox.timer >= 5 {
				print!("fuck2\n");
				gamestate.worldData.eventHandler.textbox.timer = 0;
				gamestate.worldData.eventHandler.textbox.position += 1;

				let str = &mut gamestate.localization[&text.to_string()].to_string();

				if gamestate.worldData.eventHandler.textbox.position < str.len() as i32 {
					let _ = str.split_off(gamestate.worldData.eventHandler.textbox.position as usize);
				}
				gamestate.worldData.eventHandler.textbox.currentText = str.to_string();
			}

			if data::key_pressed("confirm") {
				let str = &mut gamestate.localization[&text.to_string()].to_string();
				if gamestate.worldData.eventHandler.textbox.position < str.len() as i32 {
					gamestate.worldData.eventHandler.textbox.position = str.len() as i32;
				} else {
					let chPos = gamestate.worldData.eventHandler.currentChain as usize + 1;
					if chPos >= gamestate.worldData.eventList[&gamestate.worldData.eventHandler.currentEvent].chain.len() { gamestate.worldData.eventHandler.textbox.state = TextboxState::Inactive; return true; }
					let chain = &gamestate.worldData.eventList[&gamestate.worldData.eventHandler.currentEvent].chain[chPos];
					match chain {
						EventChain::Text {..} => {

						},
						_ => {
							gamestate.worldData.eventHandler.textbox.reset();
						},
					}
					return true;
				}
			}
		},
		TextboxState::Finished => {},
		TextboxState::Reset => {},
	}
	return false;
}

/// Draw textbox
//TODO Apply screen scaling
pub fn draw( gamestate : &mut data::Gamestate ) {
	let widthOffset = data::get_screenwidth() as f32 / 8.0;
	let heightOffset = data::get_screenheight() as f32 / 1.5;

	if gamestate.worldData.eventHandler.textbox.state != TextboxState::Inactive {
		raylib::draw_texture_npatch(
			gamestate.textures["ui_textbox_general"],
			raylib_ffi::Rectangle {
				x: widthOffset,
				y: heightOffset,
				width: data::get_screenwidth() as f32 - (widthOffset * 2.0),
				height: data::get_screenheight() as f32 - heightOffset,
			},
			raylib_ffi::Vector2 {x: 0.0, y: 0.0},
			0.0,
			raylib_ffi::colors::WHITE,
		);
	
		raylib::draw_text_pro(
			gamestate.fonts["default"],
			&gamestate.worldData.eventHandler.textbox.currentText,
			raylib_ffi::Vector2 {x: widthOffset + (widthOffset / 3.0), y: heightOffset + (widthOffset / 2.75)},
			raylib_ffi::Vector2 {x: 0.0, y: 0.0},
			0.0,
			24.0,
			5.0,
			raylib_ffi::Color{r:57,g:57,b:57,a:255},
		);
	}
}