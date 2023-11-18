

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld::{self, Direction}, data, raylib, utilities::math, events, monsters};


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
	pub gearSelection:	i32
}


//= Procedures

impl Player {
	
}

impl Menu {
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

/// Initialize player data
pub fn init() -> Player {
	let mut player = Player{
		unit:		overworld::create_unit("player_1"),
		monsters:	monsters::MonsterTeam([None, None, None, None]),
		canMove:	true,
		menu:		init_menu(),
	};
	
	player.unit.position = raylib_ffi::Vector3{x: 1.0,y: 0.0,z: 2.0};
	player.unit.posTarget = raylib_ffi::Vector3{x: 1.0,y: 0.0,z: 2.0};

	return player;
}

/// Initial value of menu
pub fn init_menu() -> Menu {
	return Menu {
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
	};
}

//
pub fn poll_menu( gamestate : &data::Gamestate) -> Menu {
	return Menu {
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
		gearSelection: 0,
	};
}

/// Poll controls and move player or open menus if necessary.
pub fn controls( gamestate : &mut data::Gamestate ) {
	//* Movement */
	let ft = raylib::get_frame_time();

	if !math::close_enough_v3(gamestate.player.unit.position, gamestate.player.unit.posTarget, 0.05) {
		let dir = math::get_direction_v3(gamestate.player.unit.position, gamestate.player.unit.posTarget);
		gamestate.player.unit.position = math::add_v3(gamestate.player.unit.position, math::mul_v3(dir, MVSPEED * ft));
	} else {
		//* Event handling */
		if events::parse_event(gamestate) { return; }

		if gamestate.player.canMove && gamestate.player.menu.open == MenuOptions::None {

			gamestate.player.unit.position = gamestate.player.unit.posTarget;
			let mut newpos = gamestate.player.unit.position;

			//* Check for trigger */
			let pos = [
				gamestate.player.unit.posTarget.x as i32,
				gamestate.player.unit.posTarget.y as i32,
				gamestate.player.unit.posTarget.z as i32,
			];
			if gamestate.worldData.triggerMap.contains_key(&pos) { gamestate.eventHandler.currentEvent = gamestate.worldData.triggerMap[&pos].to_string(); return; }

			//* Check for interaction */
			let mut position = [gamestate.player.unit.position.x as i32,gamestate.player.unit.position.y as i32,gamestate.player.unit.position.z as i32];
			if data::key_pressed("confirm") {
				match gamestate.player.unit.direction {
					Direction::North => position[2] = position[2] - 1,
					Direction::South => position[2] = position[2] + 1,
					Direction::East  => position[0] = position[0] - 1,
					Direction::West  => position[0] = position[0] + 1,
				}

				//* The last event in the loop that the conditions are met for is done. */
				let unitCheck = overworld::check_for_unit(&gamestate.worldData.unitMap, &position);
				if unitCheck.0 && overworld::exists(&gamestate.eventHandler, &gamestate.worldData.unitMap[&unitCheck.1]) {
					let unit = gamestate.worldData.unitMap.get_mut(&unitCheck.1).unwrap();
					unit.direction = gamestate.player.unit.direction.reverse();
					if gamestate.worldData.unitMap.contains_key(&unitCheck.1) {
						for (str, event) in &gamestate.worldData.unitMap[&unitCheck.1].events {
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
			if !math::equal_v3(gamestate.player.unit.posTarget, newpos) {
				overworld::set_animation( &mut gamestate.player.unit, "walk_".to_string() + &math::get_relative_direction_dir(gamestate.camera.rotation, dir).to_string() );
				//overworld::move_unit(&gamestate.worldData.currentMap, &mut gamestate.worldData.unitMap, &gamestate.eventHandler, &mut gamestate.player.unit, dir);
				overworld::move_unit_test(gamestate, "player".to_string(), dir);
			} else {
				overworld::set_animation( &mut gamestate.player.unit, "idle_".to_string() + &math::get_relative_direction_dir(gamestate.camera.rotation, dir).to_string() );
			}
		}

		//* Menus */
		if gamestate.eventHandler.currentEvent == "" {
			//* Openning menu */
			if data::key_pressed("enter") {
				gamestate.audio.play_sound("menu".to_string());
				gamestate.player.menu = poll_menu(gamestate);
				if gamestate.player.menu.open == MenuOptions::None { gamestate.player.menu.open = MenuOptions::Base; return; }
				if gamestate.player.menu.open != MenuOptions::None { gamestate.player.menu.open = MenuOptions::None; return; }
			}

			//* Selecting option */
			if gamestate.player.menu.open != MenuOptions::None {
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

						match selection {
							0 => { // Pokedex
								gamestate.player.menu.open = MenuOptions::Dex;
								gamestate.audio.play_sound("menu".to_string());
							}
							1 => { // Pokemon
								gamestate.player.menu.open = MenuOptions::Mon;
								gamestate.audio.play_sound("menu".to_string());
							}
							2 => { // Bag
								gamestate.player.menu.open = MenuOptions::Bag;
								gamestate.audio.play_sound("menu".to_string());
							}
							3 => { // Player
								gamestate.player.menu.open = MenuOptions::Player;
								gamestate.audio.play_sound("menu".to_string());
							}
							4 => { // Gear
								gamestate.player.menu.open = MenuOptions::Gear;
								gamestate.audio.play_sound("menu".to_string());
							}
							5 => { // Save
								gamestate.player.menu.open = MenuOptions::Save;
								gamestate.audio.play_sound("menu".to_string());
							}
							6 => { // Options
								gamestate.player.menu.open = MenuOptions::Options;
								gamestate.audio.play_sound("menu".to_string());
							}
							7 => { // Quit
								gamestate.running = false;
							}
							_ => {}
						}
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
				raylib::structures::Rectangle {
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
				
					raylib::draw_text_pro(
						gamestate.graphics.fonts["default"],
						&output,
						raylib_ffi::Vector2 {
							x: widthOffset + (fontSize * 3.0),
							y: heightOffset + fontSize + offset,
						},
						raylib_ffi::Vector2 {x: 0.0, y: 0.0},
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
				raylib::structures::Rectangle { x: 0.0, y: 0.0, width: 8.0, height: 8.0 },
				raylib::structures::Rectangle{
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
				raylib::structures::Rectangle {
						x: widthOffset,
						y: heightOffset,
						width,
						height,
					},
				0.0,
			);
		}
		MenuOptions::Save => {}
		MenuOptions::Options => {}
		_ => {}
	}
	
}