

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::fmt::Display;
use crate::{data, raylib::{self, structures::Vector2}};
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
impl Clone for Choice {
    fn clone(&self) -> Self {
        Self { text: self.text.clone(), event: self.event.clone(), position: self.position.clone() }
    }
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
	match gamestate.eventHandler.textbox.state {
		TextboxState::Inactive => {
			//* If the Textbox is currently inactive, start it up */
			gamestate.eventHandler.textbox.state = TextboxState::Active;
			gamestate.eventHandler.textbox.currentText = "".to_string();
			gamestate.eventHandler.textbox.timer = 0;
			gamestate.eventHandler.textbox.pause = 0;
			gamestate.eventHandler.textbox.position = 1;
			gamestate.eventHandler.textbox.hasChoice = false;
			gamestate.eventHandler.textbox.choiceList = Vec::new();
			gamestate.eventHandler.textbox.curPosition = 0;
			gamestate.audio.play_sound("button".to_string());

			//* Check text for replacements */
			let mut str = gamestate.localization[&text.to_string()].to_string();

			str = str.replace("{PLAYER_NAME}", &gamestate.eventHandler.playerName);
			str = str.replace("{PLAYER_PRO_SUBJECT}", &gamestate.eventHandler.playerPronouns[0]);
			str = str.replace("{PLAYER_PRO_OBJECT}", &gamestate.eventHandler.playerPronouns[1]);
			str = str.replace("{PLAYER_PRO_POSSESIVE}", &gamestate.eventHandler.playerPronouns[2]);
			str = str.replace("{RIVAL_NAME}", &gamestate.eventHandler.rivalName);
			str = str.replace("{PLAYER_PRO_SUBJECT}", &gamestate.eventHandler.playerPronouns[0]);
			//for (variable, cond) in &gamestate.eventHandler.eventVariables {
			//	let varStr = "{".to_string() + &variable.to_string() + "}";
			//	str = str.replace(&varStr, &cond.to_string());
			//} TODO Figure out why this doesn't work

			gamestate.eventHandler.textbox.targetText = str;

			return false;
		},
		TextboxState::Active => {
			//* Increase timer */
			gamestate.eventHandler.textbox.timer += 1;
			if gamestate.eventHandler.textbox.timer >= data::get_textspeed() {
				gamestate.eventHandler.textbox.timer = 0;
				gamestate.eventHandler.textbox.position += 1;

				let str = &mut gamestate.eventHandler.textbox.targetText.to_string();

				if gamestate.eventHandler.textbox.position < str.len() as i32 {
					let _ = str.split_off(gamestate.eventHandler.textbox.position as usize);
				}
				gamestate.eventHandler.textbox.currentText = str.to_string();
			}

			//* If it's a choice box, move cursor on button press */
			if gamestate.eventHandler.textbox.hasChoice {
				if data::key_pressed("up") {
					if gamestate.eventHandler.textbox.curPosition == 0 {
						gamestate.eventHandler.textbox.curPosition = 3;
						for i in 0..4 { if gamestate.eventHandler.textbox.choiceList[i as usize].text == "" { gamestate.eventHandler.textbox.curPosition -= 1; } }
					} else { gamestate.eventHandler.textbox.curPosition -= 1; }
				}
				if data::key_pressed("down") {
					if gamestate.eventHandler.textbox.curPosition >= gamestate.eventHandler.textbox.choiceList.len() as i32 - 1 || gamestate.eventHandler.textbox.choiceList[gamestate.eventHandler.textbox.curPosition as usize + 1].text == "" { gamestate.eventHandler.textbox.curPosition = 0; }
					else { gamestate.eventHandler.textbox.curPosition += 1; }
				}
			}

			//* If it's an input box, accept all keys */
			// TODO Decide if i want string max length to be 16...
			if gamestate.eventHandler.textbox.isInput {
				let input = raylib::get_key_pressed();
				if input == ".".to_string() && gamestate.eventHandler.textbox.input.len() > 0 {
					gamestate.eventHandler.textbox.input.truncate(gamestate.eventHandler.textbox.input.len()-1);
				} else if gamestate.eventHandler.textbox.input.len() < 16 && input != ".".to_string() { gamestate.eventHandler.textbox.input += &input; }
				if data::key_pressed("enter") {
					if gamestate.eventHandler.textbox.input.len() > 0 {
						gamestate.eventHandler.textbox.reset();
						gamestate.audio.play_sound("button".to_string());
						return true;
					}
				}
			}

			if data::key_pressed("confirm") {
				let str = &mut gamestate.localization[&text.to_string()].to_string();
				if gamestate.eventHandler.textbox.position < str.len() as i32 {
					//* Skip text scroll */
					gamestate.audio.play_sound("button".to_string());
					gamestate.eventHandler.textbox.position = str.len() as i32;
				} else {
					//* If its a choice */
					if gamestate.eventHandler.textbox.hasChoice {
						gamestate.audio.play_sound("button".to_string());

						let choice = &gamestate.eventHandler.textbox.choiceList[gamestate.eventHandler.textbox.curPosition as usize];
						if choice.event == "" {
							if choice.position != -1 { gamestate.eventHandler.currentChain = choice.position; }
							gamestate.eventHandler.textbox.reset();
							return true;
						}
						if choice.event == gamestate.eventHandler.currentEvent {
							if choice.position != -1 { gamestate.eventHandler.currentChain = choice.position; }
							gamestate.eventHandler.textbox.reset();
							return false;
						}
						if gamestate.worldData.eventList.contains_key(&choice.event) {
							gamestate.eventHandler.currentEvent = choice.event.to_string();
							if choice.position != -1 { gamestate.eventHandler.currentChain = choice.position; }
							gamestate.eventHandler.textbox.reset();
							return false;
						}
					//* If it's an input */
					} else if gamestate.eventHandler.textbox.isInput {
					
					//* If it's a basic textbox */
					} else {
						gamestate.audio.play_sound("button".to_string());

						let chPos = gamestate.eventHandler.currentChain as usize + 1;
						if chPos >= gamestate.worldData.eventList[&gamestate.eventHandler.currentEvent].chain.len() { gamestate.eventHandler.textbox.state = TextboxState::Inactive; return true; }
						let chain = &gamestate.worldData.eventList[&gamestate.eventHandler.currentEvent].chain[chPos];
						match chain {
							EventChain::Text {..} => {
								gamestate.eventHandler.textbox.timer = 0;
								gamestate.eventHandler.textbox.position = 0;
							},
							EventChain::Choice {..} => {
								gamestate.eventHandler.textbox.timer = 0;
								gamestate.eventHandler.textbox.position = 0;
							},
							_ => {
								gamestate.eventHandler.textbox.reset();
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

	if gamestate.eventHandler.textbox.state != TextboxState::Inactive {
		gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
			raylib::structures::Rectangle {
				x:		widthOffset,
				y:		heightOffset,
				width:	data::get_screenwidth() as f32 - (widthOffset * 2.0),
				height:	data::get_screenheight() as f32 - heightOffset,
			},
			0.0,
		);

		let ratio = data::get_screenratio();
		let mut fontSize = 24.0;
		if ratio > 1.0 { fontSize = (((24.0 * ratio) / 8.0) as i32) as f32 * 8.0 }
		gamestate.graphics.fonts["default"].draw_pro(
			&gamestate.eventHandler.textbox.currentText,
			Vector2 {x: widthOffset + (widthOffset / 3.0), y: heightOffset + (heightOffset / 8.0)},
			0.0,
			fontSize,
			5.0 * ratio,
			raylib_ffi::Color{r:57,g:57,b:57,a:255},
		);

		//* Draw options */
		if gamestate.eventHandler.textbox.hasChoice {
			let choiceWidthOffset = data::get_screenwidth() as f32 - (widthOffset * 2.0);
			let choiceHeightOffset = heightOffset - fontSize;

			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				raylib::structures::Rectangle {
					x:		choiceWidthOffset,
					y:		choiceHeightOffset,
					width:	widthOffset * 1.5,
					height:	data::get_screenheight() as f32 - heightOffset,
				},
				0.0,
			);
			let mut choiceOffset = 0.0;
			for i in &gamestate.eventHandler.textbox.choiceList {
				if i.text != "" {
					gamestate.graphics.fonts["default"].draw_pro(
						&gamestate.localization[&i.text],
						Vector2 {
							x: choiceWidthOffset + (widthOffset / 3.0) + (12.0 * ratio),
							y: choiceHeightOffset + (heightOffset / 8.0) + (choiceOffset * (fontSize + (12.0 * ratio))),
						},
						0.0,
						fontSize,
						5.0 * ratio,
						raylib_ffi::Color{r:57,g:57,b:57,a:255},
					);
				}
				choiceOffset += 1.0;
			}
			let mut height = choiceHeightOffset + (heightOffset / 8.0) - (8.0 * ratio);
			match gamestate.eventHandler.textbox.curPosition {
				1 => height += fontSize + (12.0 * ratio),
				2 => height += 2.0 * (fontSize + (12.0 * ratio)),
				3 => height += 3.0 * (fontSize + (12.0 * ratio)),
				_ => {},
			}
			gamestate.graphics.textures["ui_pointer_general"].draw_pro(
				raylib::structures::Rectangle { x:0.0,y:0.0, width:8.0,height:8.0 },
				raylib::structures::Rectangle {
					x:		choiceWidthOffset + (widthOffset / 3.0) - (24.0 * ratio),
					y:		height,
					width:	32.0 * ratio,
					height:	32.0 * ratio,
				},
				0.0,
			);
		}

		//* Draw input */
		if gamestate.eventHandler.textbox.isInput {
			let inputWidthOffset = 320.0 * data::get_screenratio();
			let inputHeightOffset = heightOffset - (fontSize * 2.0);
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				raylib::structures::Rectangle {
					x:		inputWidthOffset,
					y:		inputHeightOffset,
					width:	data::get_screenwidth() as f32 - (inputWidthOffset * 2.0),
					height:	fontSize * 4.0,
				},
				0.0,
			);

			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.eventHandler.textbox.input,
				Vector2 {
					x: inputWidthOffset + (widthOffset / 5.5) + (12.0 * ratio),
					y: inputHeightOffset + (heightOffset / 10.5),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);

			gamestate.graphics.textures["ui_pointer_general"].draw_pro(
				raylib::structures::Rectangle { x:0.0,y:0.0, width:8.0,height:8.0 },
				raylib::structures::Rectangle {
					x:		inputWidthOffset + (widthOffset / 5.5) + (12.0 * ratio) + (gamestate.eventHandler.textbox.input.len() as f32 * (fontSize + (5.0 * ratio))),
					y:		inputHeightOffset + (heightOffset / 12.0),
					width:	32.0 * ratio,
					height:	32.0 * ratio,
				},
				0.0,
			);
		}
	}
}