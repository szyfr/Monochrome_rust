

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

	pub isInput: bool,
	pub input: String,
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

		self.isInput = false;
		self.input = "".to_string();
	}
}

/// Choices structure
pub struct Choice{
	pub text:  String,
	pub event: String,
	pub position: i32,
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

		isInput: false,
		input: "".to_string(),
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
			gamestate.worldData.eventHandler.textbox.timer += 1;
			if gamestate.worldData.eventHandler.textbox.timer >= data::get_textspeed() {
				gamestate.worldData.eventHandler.textbox.timer = 0;
				gamestate.worldData.eventHandler.textbox.position += 1;

				let str = &mut gamestate.localization[&text.to_string()].to_string();

				if gamestate.worldData.eventHandler.textbox.position < str.len() as i32 {
					let _ = str.split_off(gamestate.worldData.eventHandler.textbox.position as usize);
				}
				gamestate.worldData.eventHandler.textbox.currentText = str.to_string();
			}

			//* If it's a choice box, move cursor on button press */
			if gamestate.worldData.eventHandler.textbox.hasChoice {
				if data::key_pressed("up") {
					if gamestate.worldData.eventHandler.textbox.curPosition == 0 {
						gamestate.worldData.eventHandler.textbox.curPosition = 3;
						for i in 0..4 { if gamestate.worldData.eventHandler.textbox.choiceList[i as usize].text == "" { gamestate.worldData.eventHandler.textbox.curPosition -= 1; } }
					} else { gamestate.worldData.eventHandler.textbox.curPosition -= 1; }
				}
				if data::key_pressed("down") {
					if gamestate.worldData.eventHandler.textbox.curPosition >= gamestate.worldData.eventHandler.textbox.choiceList.len() as i32 - 1 || gamestate.worldData.eventHandler.textbox.choiceList[gamestate.worldData.eventHandler.textbox.curPosition as usize + 1].text == "" { gamestate.worldData.eventHandler.textbox.curPosition = 0; }
					else { gamestate.worldData.eventHandler.textbox.curPosition += 1; }
				}
			}

			//* If it's an input box, accept all keys */
			// TODO Decide if i want string max length to be 16...
			// TODO Also decide if i want space to still be confirm for input or do i want to change it to Enter to allow spaces
			if gamestate.worldData.eventHandler.textbox.isInput {
				let input = raylib::get_key_pressed();
				if input == ".".to_string() && gamestate.worldData.eventHandler.textbox.input.len() > 0 {
					gamestate.worldData.eventHandler.textbox.input.truncate(gamestate.worldData.eventHandler.textbox.input.len()-1);
				} else if gamestate.worldData.eventHandler.textbox.input.len() < 16 && input != ".".to_string() { gamestate.worldData.eventHandler.textbox.input += &input; }
			}

			if data::key_pressed("confirm") {
				let str = &mut gamestate.localization[&text.to_string()].to_string();
				if gamestate.worldData.eventHandler.textbox.position < str.len() as i32 {
					gamestate.worldData.eventHandler.textbox.position = str.len() as i32;
				} else {
					if gamestate.worldData.eventHandler.textbox.hasChoice {
						let choice = &gamestate.worldData.eventHandler.textbox.choiceList[gamestate.worldData.eventHandler.textbox.curPosition as usize];
						if choice.event == "" {
							gamestate.worldData.eventHandler.textbox.reset();
							return true;
						}
						if choice.event == gamestate.worldData.eventHandler.currentEvent {
							gamestate.worldData.eventHandler.currentChain = choice.position;
							gamestate.worldData.eventHandler.textbox.reset();
							return false;
						}
						if gamestate.worldData.eventList.contains_key(&choice.event) {
							gamestate.worldData.eventHandler.currentEvent = choice.event.to_string();
							gamestate.worldData.eventHandler.currentChain = choice.position;
							gamestate.worldData.eventHandler.textbox.reset();
							return false;
						}
					} else if gamestate.worldData.eventHandler.textbox.isInput {
						if gamestate.worldData.eventHandler.textbox.input.len() > 0 {
							gamestate.worldData.eventHandler.textbox.reset();
							return true;
						}
					} else {
						let chPos = gamestate.worldData.eventHandler.currentChain as usize + 1;
						if chPos >= gamestate.worldData.eventList[&gamestate.worldData.eventHandler.currentEvent].chain.len() { gamestate.worldData.eventHandler.textbox.state = TextboxState::Inactive; return true; }
						let chain = &gamestate.worldData.eventList[&gamestate.worldData.eventHandler.currentEvent].chain[chPos];
						match chain {
							EventChain::Text {..} => {
								gamestate.worldData.eventHandler.textbox.timer = 0;
								gamestate.worldData.eventHandler.textbox.position = 0;
							},
							EventChain::Choice {..} => {
								gamestate.worldData.eventHandler.textbox.timer = 0;
								gamestate.worldData.eventHandler.textbox.position = 0;
							},
							_ => {
								gamestate.worldData.eventHandler.textbox.reset();
							},
						}
						return true;
					}
				}
			}
		},
		TextboxState::Finished => {},
		TextboxState::Reset => {},
	}
	return false;
}

/// Draw textbox
pub fn draw( gamestate : &mut data::Gamestate ) {
	let widthOffset = 160.0 * data::get_screenratio();
	let heightOffset = 480.0 * data::get_screenratio();

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

		let ratio = data::get_screenratio();
		let mut fontSize = 24.0;
		if ratio > 1.0 { fontSize = (((24.0 * ratio) / 8.0) as i32) as f32 * 8.0 }
		raylib::draw_text_pro(
			gamestate.fonts["default"],
			&gamestate.worldData.eventHandler.textbox.currentText,
			raylib_ffi::Vector2 {x: widthOffset + (widthOffset / 3.0), y: heightOffset + (heightOffset / 8.0)},
			raylib_ffi::Vector2 {x: 0.0, y: 0.0},
			0.0,
			fontSize,
			5.0 * ratio,
			raylib_ffi::Color{r:57,g:57,b:57,a:255},
		);

		//* Draw options */
		if gamestate.worldData.eventHandler.textbox.hasChoice {
			let choiceWidthOffset = data::get_screenwidth() as f32 - (widthOffset * 2.0);
			let choiceHeightOffset = heightOffset - fontSize;
			raylib::draw_texture_npatch(
				gamestate.textures["ui_textbox_general"],
				raylib_ffi::Rectangle {
					x: choiceWidthOffset,
					y: choiceHeightOffset,
					width: widthOffset * 1.5,
					height: data::get_screenheight() as f32 - heightOffset,
				},
				raylib_ffi::Vector2 {x: 0.0, y: 0.0},
				0.0,
				raylib_ffi::colors::WHITE,
			);
			let mut choiceOffset = 0.0;
			for i in &gamestate.worldData.eventHandler.textbox.choiceList {
				if i.text != "" {
					raylib::draw_text_pro(
						gamestate.fonts["default"],
						&gamestate.localization[&i.text],
						raylib_ffi::Vector2 {x: choiceWidthOffset + (widthOffset / 3.0) + (12.0 * ratio), y: choiceHeightOffset + (heightOffset / 8.0) + (choiceOffset * (fontSize + (12.0 * ratio)))},
						raylib_ffi::Vector2 {x: 0.0, y: 0.0},
						0.0,
						fontSize,
						5.0 * ratio,
						raylib_ffi::Color{r:57,g:57,b:57,a:255},
					);
				}
				choiceOffset += 1.0;
			}
			let mut height = choiceHeightOffset + (heightOffset / 8.0) - (8.0 * ratio);
			match gamestate.worldData.eventHandler.textbox.curPosition {
				1 => height += fontSize + (12.0 * ratio),
				2 => height += 2.0 * (fontSize + (12.0 * ratio)),
				3 => height += 3.0 * (fontSize + (12.0 * ratio)),
				_ => {},
			}
			raylib::draw_texture_pro(
				gamestate.textures["ui_pointer_general"],
				raylib_ffi::Rectangle{ x:0.0,y:0.0, width:8.0,height:8.0 },
				raylib_ffi::Rectangle{
					x: choiceWidthOffset + (widthOffset / 3.0) - (24.0 * ratio),
					y: height,
					width: 32.0 * ratio,
					height: 32.0 * ratio,
				},
				raylib_ffi::Vector2{x: 0.0, y: 0.0},
				0.0,
				raylib_ffi::colors::WHITE,
			);
		}

		//* Draw input */
		if gamestate.worldData.eventHandler.textbox.isInput {
			let inputWidthOffset = 320.0 * data::get_screenratio();
			let inputHeightOffset = heightOffset - (fontSize * 2.0);
			raylib::draw_texture_npatch(
				gamestate.textures["ui_textbox_general"],
				raylib_ffi::Rectangle {
					x: inputWidthOffset,
					y: inputHeightOffset,
					width: data::get_screenwidth() as f32 - (inputWidthOffset * 2.0),
					height: (fontSize * 4.0),
				},
				raylib_ffi::Vector2 {x: 0.0, y: 0.0},
				0.0,
				raylib_ffi::colors::WHITE,
			);

			raylib::draw_text_pro(
				gamestate.fonts["default"],
				&gamestate.worldData.eventHandler.textbox.input,
				raylib_ffi::Vector2 {x: inputWidthOffset + (widthOffset / 5.5) + (12.0 * ratio), y: inputHeightOffset + (heightOffset / 10.5)},
				raylib_ffi::Vector2 {x: 0.0, y: 0.0},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
		}
	}
}