

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
	return Player {
		unit:		overworld::create_unit("player_1.png"),
		canMove:	true,
	}
}

pub fn controls( gamestate : &data::Gamestate ) -> Player {
	//! Temporary until I implemant copy
	let mut newPlayer = Player {
		unit: Unit {
			position: gamestate.player.unit.position,
			posTarget: gamestate.player.unit.posTarget,
			direction: gamestate.player.unit.direction,
			animator: Animator {
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

	if math::close_enough_v3(newPlayer.unit.position, newPlayer.unit.posTarget, 0.05) {
		let dir = math::get_direction_v3(newPlayer.unit.position, newPlayer.unit.posTarget);
		newPlayer.unit.position = math::add_v3(newPlayer.unit.position, math::mul_v3(dir, MVSPEED * ft));
	} else if newPlayer.canMove {
		newPlayer.unit.position = newPlayer.unit.posTarget;
		let mut newpos = newPlayer.unit.position;

		//* Gather inputs */
		let up	= settings::button_down("up", &gamestate.settings);
		let down	= settings::button_down("down", &gamestate.settings);
		let left	= settings::button_down("left", &gamestate.settings);
		let right = settings::button_down("right", &gamestate.settings);

		let curRot = gamestate.camera.rotation;

		// TODO Figure this out again
		//if curRot < 45.0 && curRot > -45.0 {
		//	newPlayer.unit.direction = Direction::North;
		//	newpos.z -= 1.0;
		//}
	}

	return newPlayer;
}