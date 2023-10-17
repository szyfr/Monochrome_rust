

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld::{self, Direction}, data, raylib, utilities::math, settings, events::EventChain};


//= Constants
/// Player movement speed
const MVSPEED : f32 = 3.0;


//= Structures
/// Storage structure for Player data
pub struct Player {
	pub unit : overworld::Unit,

	pub canMove : bool,
}


//= Procedures

/// Initialize player data
pub fn init() -> Player {
	let mut player = Player{
		unit:		overworld::create_unit("player_1"),
		canMove:	true,
	};
	player.unit.position = raylib_ffi::Vector3{x: 1.0,y: 0.0,z: 2.0};
	player.unit.posTarget = raylib_ffi::Vector3{x: 1.0,y: 0.0,z: 2.0};

	return player;
}

/// Poll controls and move player or open menus if necessary.
pub fn controls( gamestate : &mut data::Gamestate ) {
	//* Movement */
	let ft = raylib::get_frame_time();

	if !math::close_enough_v3(gamestate.player.unit.position, gamestate.player.unit.posTarget, 0.05) {
		let dir = math::get_direction_v3(gamestate.player.unit.position, gamestate.player.unit.posTarget);
		gamestate.player.unit.position = math::add_v3(gamestate.player.unit.position, math::mul_v3(dir, MVSPEED * ft));
	} else if gamestate.player.canMove {
		gamestate.player.unit.position = gamestate.player.unit.posTarget;
		let mut newpos = gamestate.player.unit.position;

		//* Check for trigger */
		let pos = [
			gamestate.player.unit.position.x as i32,
			gamestate.player.unit.position.y as i32,
			gamestate.player.unit.position.z as i32,
		];
		if gamestate.worldData.triggerMap.contains_key(&pos) {
			// TODO Migrate this to events
			gamestate.worldData.eventHandler.currentEvent = gamestate.worldData.triggerMap[&pos].to_string();
		}

		//* Check for interaction */
		let mut position = [gamestate.player.unit.position.x as i32,gamestate.player.unit.position.y as i32,gamestate.player.unit.position.z as i32];
		if settings::button_down("confirm", &gamestate.settings) {
			match gamestate.player.unit.direction {
				Direction::North => position[2] = position[2] - 1,
				Direction::South => position[2] = position[2] + 1,
				Direction::East  => position[0] = position[0] - 1,
				Direction::West  => position[0] = position[0] + 1,
				_ => return,
			}
			//if gamestate.worldData.unitMap.contains_key(&pos) {
			let unitCheck = overworld::check_for_unit(&gamestate.worldData.unitMap, &position);
			if unitCheck.0 && overworld::exists(&gamestate.worldData.eventHandler, &gamestate.worldData.unitMap[&unitCheck.1]) {
				gamestate.worldData.eventHandler.currentEvent = unitCheck.1;
			}
		}

		//* Event handling */
		if gamestate.worldData.eventHandler.currentEvent != "".to_string() {
			print!("{}\n",gamestate.worldData.eventHandler.currentEvent);
			let event = &gamestate.worldData.eventList[&gamestate.worldData.eventHandler.currentEvent];
			if gamestate.worldData.eventHandler.currentChain >= event.chain.len() as i32 {
				gamestate.worldData.eventHandler.currentEvent = "".to_string();
				gamestate.worldData.eventHandler.currentChain = 0;
			} else {
				let cond = &event.chain[gamestate.worldData.eventHandler.currentChain as usize];
				match cond {
					EventChain::Test{ text } => {
						print!("TEST: {}\n",text);
						gamestate.worldData.eventHandler.currentChain += 1;
					},
					EventChain::Text { text } => {
						print!("TEXT: {}\n",text);
						gamestate.worldData.eventHandler.currentChain += 1;
					},
					EventChain::Warp { entityID, position, .. } => {
						print!("WARP: {}->[{},{},{}]\n", entityID, position[0], position[1], position[2]);
						gamestate.worldData.eventHandler.currentChain += 1;
					},
					//_ => return,
				}
				let _ = std::io::Write::flush(&mut std::io::stdout());
				return;
			}
		}

		//* Gather inputs */
		let up	= settings::button_down("up", &gamestate.settings);
		let down	= settings::button_down("down", &gamestate.settings);
		let left	= settings::button_down("left", &gamestate.settings);
		let right	= settings::button_down("right", &gamestate.settings);

		let curRot = gamestate.camera.rotation;
		let mut dir = gamestate.player.unit.direction;

		if (curRot > -45.0 && curRot <=  45.0) || (curRot > 315.0 && curRot <= 405.0) {
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
		if (curRot >  45.0 && curRot <= 135.0) || (curRot > 405.0 && curRot <= 495.0) {
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
		if curRot > 135.0 && curRot <= 225.0 {
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
		if (curRot > 225.0 && curRot <= 315.0) || (curRot > -135.0 && curRot <= -45.0) {
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

		//* If the player is moving */
		gamestate.player.unit.direction = dir;
		if !math::equal_v3(gamestate.player.unit.posTarget, newpos) {
			overworld::set_animation( &mut gamestate.player.unit, "walk_".to_string() + &math::get_relative_direction_dir(gamestate.camera.rotation, dir).to_string() );
			overworld::move_unit(&gamestate.worldData, &mut gamestate.player.unit, dir);
		} else {
			if gamestate.player.unit.direction != Direction::Null { overworld::set_animation( &mut gamestate.player.unit, "idle_".to_string() + &math::get_relative_direction_dir(gamestate.camera.rotation, dir).to_string() ); }
		}
	}

	//* Menus */
	// TODO

}
