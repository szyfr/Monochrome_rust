

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld::{self, Unit, Animator, Direction}, data, raylib, utilities::math, settings};


//= Constants
const MVSPEED : f32 = 3.0;


//= Structures
pub struct Player {
	pub unit : overworld::Unit,

	pub canMove : bool,
}


//= Procedures
pub fn init() -> Player {
	let mut player = Player{
		unit:		overworld::create_unit("player_1"),
		canMove:	true,
	};
	player.unit.position = raylib_ffi::Vector3{x: 1.0,y: 0.0,z: 2.0};
	player.unit.posTarget = raylib_ffi::Vector3{x: 1.0,y: 0.0,z: 2.0};

	return player;
}

pub fn controls( gamestate : &data::Gamestate ) -> Player {
	//! Temporary until I implemant copy
	let mut newPlayer = Player {
		unit: Unit {
			position:	gamestate.player.unit.position,
			posTarget:	gamestate.player.unit.posTarget,
			direction:	gamestate.player.unit.direction,
			events:		Vec::new(),
			conditions:	Vec::new(),
			animator:	Animator {
				textures: Vec::new(),
				currentAnimation: gamestate.player.unit.animator.currentAnimation.to_string(),
				frame: gamestate.player.unit.animator.frame,
				counter: gamestate.player.unit.animator.counter,
			},
		},
		canMove: gamestate.player.canMove,
	};
	for i in &gamestate.player.unit.animator.textures { newPlayer.unit.animator.textures.push(*i); }

	//* Movement */
	let ft = raylib::get_frame_time();

	if !math::close_enough_v3(newPlayer.unit.position, newPlayer.unit.posTarget, 0.05) {
		let dir = math::get_direction_v3(newPlayer.unit.position, newPlayer.unit.posTarget);
		newPlayer.unit.position = math::add_v3(newPlayer.unit.position, math::mul_v3(dir, MVSPEED * ft));
	} else if newPlayer.canMove {
		newPlayer.unit.position = newPlayer.unit.posTarget;
		let mut newpos = newPlayer.unit.position;

		//* Gather inputs */
		let up	= settings::button_down("up", &gamestate.settings);
		let down	= settings::button_down("down", &gamestate.settings);
		let left	= settings::button_down("left", &gamestate.settings);
		let right	= settings::button_down("right", &gamestate.settings);

		let curRot = gamestate.camera.rotation;
		let mut dir = newPlayer.unit.direction;

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
		newPlayer.unit.direction = dir;
		if !math::equal_v3(newPlayer.unit.posTarget, newpos) {
			newPlayer.unit = overworld::set_animation( newPlayer.unit, "walk_".to_string() + &math::get_relative_direction_dir(&gamestate.camera, dir).to_string() );
			newPlayer.unit = overworld::move_unit(gamestate, newPlayer.unit, dir);
		} else {
			if newPlayer.unit.direction != Direction::Null { newPlayer.unit = overworld::set_animation( newPlayer.unit, "idle_".to_string() + &math::get_relative_direction_dir(&gamestate.camera, dir).to_string() ); }
		}
	}

	return newPlayer;
}