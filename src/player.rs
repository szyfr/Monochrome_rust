

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld::{self, Direction}, data, raylib::{self, vectors::{Vector2, Vector3}, rectangles::Rectangle}, events, monsters};


//= Constants
/// Player movement speed
const MVSPEED : f32 = 3.0;


//= Enumerations

/// The options on the menu screen
#[derive(Copy, Clone, PartialEq)]
pub enum MenuOptions {
	None	= -2,
	Base	= -1,
	Dex		=  0,
	Mon		=  1,
	Bag		=  2,
	Player	=  3,
	Gear	=  4,
	Save	=  5,
	Options	=  6,
}

/// The options on the gear screen
#[derive(Copy, Clone, PartialEq)]
pub enum GearOptions {
	None	= -2,
	Base	= -1,
	Status	=  0,
	Map		=  1,
	Phone	=  2,
	Radio	=  3,
}


//= Structures
/// Storage structure for Player data
pub struct Player {
	pub unit:		overworld::Unit,

	pub monsters:	monsters::MonsterTeam,

	pub canMove:	bool,
	pub menu:		Menu,
}

/// Variables that handle the menus
pub struct Menu {
	pub open:		MenuOptions,
	pub options:	[bool;8],
	pub selection:	i32,

	pub gearOpen:		GearOptions,
	pub gearOptions:	[bool;4],
	pub gearSelection:	i32,

	pub optionSelection: i32,
}


//= Procedures

impl Player {
	
	/// Initialization
	pub fn init() -> Self {
		let mut player = Player{
			unit:		overworld::Unit::new(),
			monsters:	monsters::MonsterTeam([None, None, None, None]),
			canMove:	true,
			menu:		Menu::init(),
		};
		
		player.unit.position = Vector3{x: 1.0,y: 0.0,z: 2.0};
		player.unit.posTarget = Vector3{x: 1.0,y: 0.0,z: 2.0};
		player.unit.animator.texture = "player_1".to_string();
	
		return player;
	}

}

impl Menu {

	/// Initial value of menu
	pub fn init() -> Self {
		Self {
			open: MenuOptions::None,
			options: [
				false,
				false,
				false,
				true,
				false,
				false,
				true,
				true,
			],
			selection: 0,
	
			gearOpen: GearOptions::None,
			gearOptions: [
				true,
				false,
				false,
				false,
			],
			gearSelection: 0,
	
			optionSelection: 0,
		}
	}

	/// Update the menu
	pub fn poll(gamestate : &data::Gamestate) -> Self {
		Self {
			open: gamestate.player.menu.open,
			options: [
				gamestate.eventHandler.eventVariables.contains_key("dex") && gamestate.eventHandler.eventVariables.get("dex").unwrap().as_bool() == true,
				gamestate.player.monsters.number_of_monsters() >= 1,
				gamestate.eventHandler.eventVariables.contains_key("bag") && gamestate.eventHandler.eventVariables.get("bag").unwrap().as_bool() == true,
				true,
				gamestate.eventHandler.eventVariables.contains_key("gear") && gamestate.eventHandler.eventVariables.get("gear").unwrap().as_bool() == true,
				false, // TODO saving
				true,
				true,
			],
			selection: gamestate.player.menu.selection,
	
			gearOpen: gamestate.player.menu.gearOpen,
			gearOptions: [
				true,
				gamestate.eventHandler.eventVariables.contains_key("gear_map") && gamestate.eventHandler.eventVariables.get("gear_map").unwrap().as_bool() == true,
				gamestate.eventHandler.eventVariables.contains_key("gear_phone") && gamestate.eventHandler.eventVariables.get("gear_phone").unwrap().as_bool() == true,
				gamestate.eventHandler.eventVariables.contains_key("gear_radio") && gamestate.eventHandler.eventVariables.get("gear_radio").unwrap().as_bool() == true,
			],
			gearSelection: gamestate.player.menu.gearSelection,
	
			optionSelection: gamestate.player.menu.optionSelection,
		}
	}

	/// Get the current number of available option
	pub fn get_number_of_options(&self) -> i32 {
		let mut count = 0;
		for i in self.options {
			if i { count += 1; }
		}
		return count;
	}

	/// Get the current selection from menu
	pub fn get_current_option(&self) -> i32 {
		let mut valid = 0;
		let mut count = 0;

		for i in self.options {
			if i {
				if valid == self.selection { return count; }
				valid += 1;
			}
			count += 1;
		}
		return -1;
	}

}

/// Poll controls and move player or open menus if necessary.
pub fn controls( gamestate : &mut data::Gamestate ) {
	//* Get deltatime */
	let ft = raylib::get_frame_time();

	//* Check if player is moving */
	if !gamestate.player.unit.position.close(gamestate.player.unit.posTarget, 0.05) {
		//* Move towards target */
		let dir = gamestate.player.unit.position.direction_to(gamestate.player.unit.posTarget);
		gamestate.player.unit.position = gamestate.player.unit.position + (dir * (MVSPEED * ft));
	} else {
		//* Set position perfectly */
		gamestate.player.unit.position = gamestate.player.unit.posTarget;

		//* Event handling */
		if events::parse_event(gamestate) { return; }

		if gamestate.player.canMove && gamestate.player.menu.open == MenuOptions::None && !gamestate.battleData.started {

			let mut newpos = gamestate.player.unit.position;

			//* Check for trigger */
			let pos: [i32;3] = gamestate.player.unit.posTarget.into();
			if gamestate.worldData.triggerMap.contains_key(&pos) {
				gamestate.eventHandler.currentEvent = gamestate.worldData.triggerMap[&pos].to_string();
				return;
			}

			//* Check for interaction */
			let mut position = gamestate.player.unit.position;
			if data::key_pressed("confirm") {
				match gamestate.player.unit.direction {
					Direction::North => { position.z -= 1.0; }
					Direction::South => { position.z += 1.0; }
					Direction::East  => { position.x -= 1.0; }
					Direction::West  => { position.x += 1.0; }
				}

				//* The last event in the loop that the conditions are met for is done. */
				let (result, unitId) = overworld::check_for_unit(&gamestate.worldData.unitMap, position);
				if result && gamestate.worldData.unitMap[&unitId].exists(&gamestate.eventHandler) {
					let unit = gamestate.worldData.unitMap.get_mut(&unitId).unwrap();
					unit.direction = gamestate.player.unit.direction.reverse();
					if gamestate.worldData.unitMap.contains_key(&unitId) {
						for (str, event) in &gamestate.worldData.unitMap[&unitId].events {
							if overworld::check_conditions(&gamestate.eventHandler, &event) {
								gamestate.eventHandler.currentEvent = str.to_string();
							}
						}
					}
				}
			}

			//* Gather inputs */
			let up	= data::key_down("up");
			let down	= data::key_down("down");
			let left	= data::key_down("left");
			let right	= data::key_down("right");

			let curRot = gamestate.camera.rotation as i32;
			let mut dir = gamestate.player.unit.direction;

			match curRot {
				-46..= 45 |  316..=405 => {
					if up {
						dir = Direction::North;
						newpos.z -= 1.0;
					}
					if down {
						dir = Direction::South;
						newpos.z += 1.0;
					}
					if left {
						dir = Direction::East;
						newpos.x -= 1.0;
					}
					if right {
						dir = Direction::West;
						newpos.x += 1.0;
					}
				}
				 46..=135 |  406..=495 => {
					if up {
						dir = Direction::West;
						newpos.x += 1.0;
					}
					if down {
						dir = Direction::East;
						newpos.x -= 1.0;
					}
					if left {
						dir = Direction::North;
						newpos.z -= 1.0;
					}
					if right {
						dir = Direction::South;
						newpos.z += 1.0;
					}
				}
				136..=225 => {
					if up {
						dir = Direction::South;
						newpos.z += 1.0;
					}
					if down {
						dir = Direction::North;
						newpos.z -= 1.0;
					}
					if left {
						dir = Direction::West;
						newpos.x += 1.0;
					}
					if right {
						dir = Direction::East;
						newpos.x -= 1.0;
					}
				}
				226..=315 | -134..=-45 => {
					if up {
						dir = Direction::East;
						newpos.x -= 1.0;
					}
					if down {
						dir = Direction::West;
						newpos.x += 1.0;
					}
					if left {
						dir = Direction::South;
						newpos.z += 1.0;
					}
					if right {
						dir = Direction::North;
						newpos.z -= 1.0;
					}
				}
				_ => {
					if up {
						dir = Direction::North;
						newpos.z -= 1.0;
					}
					if down {
						dir = Direction::South;
						newpos.z += 1.0;
					}
					if left {
						dir = Direction::East;
						newpos.x -= 1.0;
					}
					if right {
						dir = Direction::West;
						newpos.x += 1.0;
					}
				}
			}

			//* If the player is moving */
			gamestate.player.unit.direction = dir;
			if gamestate.player.unit.posTarget != newpos {
				overworld::Unit::walk(gamestate, "player", dir);
			}
		}

		//* Menus */
		if gamestate.eventHandler.currentEvent == "" {
			//* Openning menu */
			if data::key_pressed("enter") {
				gamestate.audio.play_sound("menu".to_string());
				gamestate.player.menu = Menu::poll(gamestate);
				if gamestate.player.menu.open == MenuOptions::None { gamestate.player.menu.open = MenuOptions::Base; return; }
				if gamestate.player.menu.open != MenuOptions::None { gamestate.player.menu.open = MenuOptions::None; return; }
			}

			//* Selecting options in different menus */
			match gamestate.player.menu.open {
				MenuOptions::Base => {
					let count = gamestate.player.menu.get_number_of_options();

					//* Moving cursor */
					if data::key_pressed("down") {
						if gamestate.player.menu.selection < count-1 {
							gamestate.player.menu.selection += 1;
						} else {
							gamestate.player.menu.selection = 0;
						}
					}
					if data::key_pressed("up") {
						if gamestate.player.menu.selection > 0 {
							gamestate.player.menu.selection -= 1;
						} else {
							gamestate.player.menu.selection = count - 1;
						}
					}

					//* Confirming */
					if data::key_pressed("confirm") {
						if gamestate.player.menu.open == MenuOptions::Base {
							let selection = gamestate.player.menu.get_current_option();
							gamestate.audio.play_sound("menu".to_string());

							match selection {
								0 => { gamestate.player.menu.open = MenuOptions::Dex; }
								1 => { gamestate.player.menu.open = MenuOptions::Mon; }
								2 => { gamestate.player.menu.open = MenuOptions::Bag; }
								3 => { gamestate.player.menu.open = MenuOptions::Player; }
								4 => { gamestate.player.menu.open = MenuOptions::Gear; }
								5 => { gamestate.player.menu.open = MenuOptions::Save; }
								6 => { gamestate.player.menu.open = MenuOptions::Options; }
								7 => { gamestate.running = false; }
								_ => {}
							}
						}
					}
					//* Canceling */
					if data::key_pressed("cancel") {
						gamestate.audio.play_sound("menu".to_string());
						gamestate.player.menu.open = MenuOptions::None;
					}
				}
				MenuOptions::Options => {
					if data::key_pressed("down") {
						if gamestate.player.menu.optionSelection < 6 {
							gamestate.player.menu.optionSelection += 1;
						} else {
							gamestate.player.menu.optionSelection = 0;
						}
					}
					if data::key_pressed("up") {
						if gamestate.player.menu.optionSelection > 0 {
							gamestate.player.menu.optionSelection -= 1;
						} else {
							gamestate.player.menu.optionSelection = 6;
						}
					}

					//* Changing settings */
					match gamestate.player.menu.optionSelection {
						0 => { // Master
							unsafe {
								if data::key_pressed("right") {
									if data::SETTINGS.masterVolume < 1.0 {
										gamestate.audio.pause_music();
										data::SETTINGS.masterVolume += 0.1;
										gamestate.audio.pause_music();
									}
								}
								if data::key_pressed("left") {
									if data::SETTINGS.masterVolume > 0.0 {
										gamestate.audio.pause_music();
										data::SETTINGS.masterVolume -= 0.1;
										gamestate.audio.pause_music();
									}
								}
							}
						}
						1 => { // Music
							unsafe {
								if data::key_pressed("right") {
									if data::SETTINGS.musicVolume < 1.0 {
										gamestate.audio.pause_music();
										data::SETTINGS.musicVolume += 0.1;
										gamestate.audio.pause_music();
									}
								}
								if data::key_pressed("left") {
									if data::SETTINGS.musicVolume > 0.0 {
										gamestate.audio.pause_music();
										data::SETTINGS.musicVolume -= 0.1;
										gamestate.audio.pause_music();
									}
								}
							}
						}
						2 => { // Effects
							unsafe {
								if data::key_pressed("right") {
									if data::SETTINGS.sfxVolume < 1.0 { data::SETTINGS.sfxVolume += 0.1; }
								}
								if data::key_pressed("left") {
									if data::SETTINGS.sfxVolume > 0.0 { data::SETTINGS.sfxVolume -= 0.1; }
								}
							}
						}
						3 => { // Resolution
							unsafe {
								if data::key_pressed("left") {
									if data::SETTINGS.screenHeight == 1080 {
										data::SETTINGS.change_resolution(1280, 720);
									}
								}
								if data::key_pressed("right") {
									if data::SETTINGS.screenHeight == 720 {
										data::SETTINGS.change_resolution(1920, 1080);
									}
								}
							}
						}
						4 => { // Text Speed
							unsafe {
								if data::key_pressed("left") {
									if data::SETTINGS.text_speed < 4 { data::SETTINGS.text_speed += 1; }
								}
								if data::key_pressed("right") {
									if data::SETTINGS.text_speed > 0 { data::SETTINGS.text_speed -= 1; }
								}
							}
						}
						5 => { // Language

						}
						_ => {}
					}

					//* Canceling */
					if data::key_pressed("cancel") {
						gamestate.audio.play_sound("menu".to_string());
						gamestate.player.menu.open = MenuOptions::Base;
					}
				}
				_ => {
					//* Canceling */
					if data::key_pressed("cancel") {
						gamestate.audio.play_sound("menu".to_string());
						gamestate.player.menu.open = MenuOptions::Base;
					}
				}
			}
		}
	}

	

}

/// Draw menu to screen
pub fn draw_menu( gamestate : &data::Gamestate ) {
	let ratio = data::get_screenratio();
	let mut fontSize = 24.0;
	if ratio > 1.0 { fontSize = (((24.0 * ratio) / 8.0)) * 8.0 }

	match gamestate.player.menu.open {
		MenuOptions::Base => {
			let heightOffset = 16.0 * ratio;
			let width = 320.0 * ratio;
			let widthOffset = data::get_screenwidth() as f32 - heightOffset - width;
			let height = (gamestate.player.menu.get_number_of_options() as f32 + 1.5) * (fontSize * 2.0);

			//* Draw BG */
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				Rectangle {
						x: widthOffset,
						y: heightOffset,
						width,
						height,
					},
				0.0,
			);

			//* Draw text */
			let mut offset = fontSize * 1.25;
			let mut count = 0;
			for option in gamestate.player.menu.options {
				if option {
					let str = "menu_".to_string() + &count.to_string();
					let output: String;
					if str == "menu_3" { output = gamestate.eventHandler.playerName.to_string(); }
					else { output = gamestate.localization[&str].to_string(); }
				
					gamestate.graphics.fonts["default"].draw_pro(
						&output,
						Vector2{
							x: widthOffset + (fontSize * 3.0),
							y: heightOffset + fontSize + offset,
						},
						0.0,
						fontSize,
						5.0 * ratio,
						raylib_ffi::Color{r:57,g:57,b:57,a:255},
					);
					offset += fontSize * 2.0;
				}
				count += 1;
			}
		
			//* Draw Pointer */
			gamestate.graphics.textures["ui_pointer_general"].draw_pro(
				Rectangle { x: 0.0, y: 0.0, width: 8.0, height: 8.0 },
				Rectangle{
					x: widthOffset + (fontSize * 1.5),
					y: heightOffset + (fontSize * 2.0) + (gamestate.player.menu.selection as f32 * (fontSize * 2.0)),
					width: 32.0 * ratio,
					height: 32.0 * ratio,
				},
				0.0,
			);
		}
		MenuOptions::Dex => {}
		MenuOptions::Mon => {}
		MenuOptions::Bag => {}
		MenuOptions::Player => {}
		MenuOptions::Gear => {
			let heightOffset = 16.0 * ratio;
			let width = 320.0 * ratio;
			let widthOffset = data::get_screenwidth() as f32 - heightOffset - width;
			let height = (gamestate.player.menu.get_number_of_options() as f32 + 1.5) * (fontSize * 2.0);

			//* Draw BG */
			gamestate.graphics.textures["ui_blackbox_general"].draw_npatch(
				Rectangle {
						x: widthOffset,
						y: heightOffset,
						width,
						height,
					},
				0.0,
			);
		}
		MenuOptions::Save => {}
		MenuOptions::Options => {
			let width = 600.0 * ratio;
			let widthOffset = (data::get_screenwidth() as f32 / 2.0) - (width / 2.0);
			let height = 700.0 * ratio;
			let heightOffset = (data::get_screenheight() as f32 / 2.0) - (height / 2.0);

			//* Draw BG */
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				Rectangle {
						x: widthOffset,
						y: heightOffset,
						width,
						height,
					},
				0.0,
			);

			//* Draw Option 1: Master Volume */
			let mut str: String;
			unsafe { str = ((data::SETTINGS.masterVolume * 10.0) as i32).to_string(); }
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization.get("options_master").unwrap().to_string(),
				Vector2 {
					x: widthOffset + (fontSize * 3.0),
					y: heightOffset + (fontSize * 3.0),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			gamestate.graphics.fonts["default"].draw_pro(
				&str,
				Vector2 {
					x: widthOffset + (fontSize * 5.5),
					y: heightOffset + (fontSize * 4.25),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			//* Draw Option 2: Music Volume */
			unsafe { str = ((data::SETTINGS.musicVolume * 10.0) as i32).to_string(); }
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization.get("options_music").unwrap().to_string(),
				Vector2 {
					x: widthOffset + (fontSize * 3.0),
					y: heightOffset + (fontSize * 5.5),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			gamestate.graphics.fonts["default"].draw_pro(
				&str,
				Vector2 {
					x: widthOffset + (fontSize * 5.5),
					y: heightOffset + (fontSize * 6.75),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			//* Draw Option 3: Effects Volume */
			unsafe { str = ((data::SETTINGS.sfxVolume * 10.0) as i32).to_string(); }
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization.get("options_effects").unwrap().to_string(),
				Vector2 {
					x: widthOffset + (fontSize * 3.0),
					y: heightOffset + (fontSize * 8.0),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			gamestate.graphics.fonts["default"].draw_pro(
				&str,
				Vector2 {
					x: widthOffset + (fontSize * 5.5),
					y: heightOffset + (fontSize * 9.25),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			//* Draw Option 4: Resolution */
			unsafe { str = data::SETTINGS.screenWidth.to_string() + " x " + &data::SETTINGS.screenHeight.to_string(); }
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization.get("options_resolution").unwrap().to_string(),
				Vector2 {
					x: widthOffset + (fontSize * 3.0),
					y: heightOffset + (fontSize * 10.5),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			gamestate.graphics.fonts["default"].draw_pro(
				&str,
				Vector2 {
					x: widthOffset + (fontSize * 5.5),
					y: heightOffset + (fontSize * 11.75),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			//* Draw Option 5: Text Speed */
			unsafe {
				match data::SETTINGS.text_speed {
					1 => { str = gamestate.localization.get("options_speed_1").unwrap().to_string(); }
					2 => { str = gamestate.localization.get("options_speed_2").unwrap().to_string(); }
					3 => { str = gamestate.localization.get("options_speed_3").unwrap().to_string(); }
					4 => { str = gamestate.localization.get("options_speed_4").unwrap().to_string(); }
					_ => { str = gamestate.localization.get("options_speed_0").unwrap().to_string(); }
				}
			}
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization.get("options_speed").unwrap().to_string(),
				Vector2 {
					x: widthOffset + (fontSize * 3.0),
					y: heightOffset + (fontSize * 12.75),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			gamestate.graphics.fonts["default"].draw_pro(
				&str,
				Vector2 {
					x: widthOffset + (fontSize * 5.5),
					y: heightOffset + (fontSize * 14.0),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			//* Draw Option 6: Language */
			unsafe { str = data::SETTINGS.language.to_string(); }
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization.get("options_language").unwrap().to_string(),
				Vector2 {
					x: widthOffset + (fontSize * 3.0),
					y: heightOffset + (fontSize * 15.25),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);
			gamestate.graphics.fonts["default"].draw_pro(
				&str,
				Vector2 {
					x: widthOffset + (fontSize * 5.5),
					y: heightOffset + (fontSize * 16.5),
				},
				0.0,
				fontSize,
				5.0 * ratio,
				raylib_ffi::Color{r:57,g:57,b:57,a:255},
			);

			//* Cursor */
			gamestate.graphics.textures["ui_pointer_general"].draw_pro(
				Rectangle{ x: 0.0, y: 0.0, width: 8.0, height: 8.0 },
				Rectangle{
					x: widthOffset + (fontSize * 1.5),
					y: heightOffset + (fontSize * 3.25) + ((fontSize * gamestate.player.menu.optionSelection as f32) * 2.5),
					width: 32.0,
					height: 32.0,
				},
				0.0,
			);
		}
		_ => {}
	}
	
}