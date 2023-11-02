

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld::{self, Direction}, data, raylib, utilities::math, events};


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
	} else {
		//* Event handling */
		if events::parse_event(gamestate) { return; }

		if gamestate.player.canMove {

			gamestate.player.unit.position = gamestate.player.unit.posTarget;
			let mut newpos = gamestate.player.unit.position;

			//* Check for trigger */
			let pos = [
				gamestate.player.unit.posTarget.x as i32,
				gamestate.player.unit.posTarget.y as i32,
				gamestate.player.unit.posTarget.z as i32,
			];
			if gamestate.worldData.triggerMap.contains_key(&pos) { gamestate.worldData.eventHandler.currentEvent = gamestate.worldData.triggerMap[&pos].to_string(); return; }

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
				if unitCheck.0 && overworld::exists(&gamestate.worldData.eventHandler, &gamestate.worldData.unitMap[&unitCheck.1]) {
					let unit = gamestate.worldData.unitMap.get_mut(&unitCheck.1).unwrap();
					unit.direction = gamestate.player.unit.direction.reverse();
					if gamestate.worldData.unitMap.contains_key(&unitCheck.1) {
						for (str, event) in &gamestate.worldData.unitMap[&unitCheck.1].events {
							if overworld::check_conditions(&gamestate.worldData.eventHandler, &event) {
								gamestate.worldData.eventHandler.currentEvent = str.to_string();
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
				//overworld::move_unit(&gamestate.worldData.currentMap, &mut gamestate.worldData.unitMap, &gamestate.worldData.eventHandler, &mut gamestate.player.unit, dir);
				overworld::move_unit_test(gamestate, "player".to_string(), dir);
			} else {
				overworld::set_animation( &mut gamestate.player.unit, "idle_".to_string() + &math::get_relative_direction_dir(gamestate.camera.rotation, dir).to_string() );
			}
		}
	}

	//* Menus */
	// TODO

}
