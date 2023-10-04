

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{raylib, utilities::math, settings, data};
use raylib_ffi::Vector3;


//= Constants
const MVSPEED : f32 =   5.0;
const CMSPEED : f32 = 500.0;


//= Structures
#[derive(Copy, Clone)]
pub struct Camera {
	pub position	: Vector3,
	pub posTarget	: Vector3,
	
	pub rotation	: f32,
	pub rotTarget	: f32,

	pub camPosition	: Vector3,
	pub fovy		: f32,

	onPlayer		: bool,
}


//= Procedures
pub fn init() -> Camera {
	return Camera{
		position:		Vector3 {x:0.5,y:0.0,z:0.5},
		posTarget:		Vector3 {x:0.5,y:0.0,z:0.5},
		
		rotation:		0.0,
		rotTarget:		0.0,

		camPosition:	Vector3 {x:0.5,y:7.0,z:5.5},
		fovy:			70.0,

		onPlayer:		true,
	};
}

pub fn update( gamestate : &data::Gamestate ) -> Camera {
	let mut newCamera = gamestate.camera;
	let ft = raylib::get_frame_time();

	//* Check if targetting a unit */
	if newCamera.onPlayer {
		newCamera.position = math::add_v3(gamestate.player.unit.position, Vector3{x:0.0,y:1.0,z:0.0});
	} else {
		//* Update Position */
		if !math::close_enough_v3(newCamera.position, newCamera.posTarget, 0.5) {
			let dir = math::get_direction_v3(newCamera.position, newCamera.posTarget);
			newCamera.position = math::add_v3(newCamera.position, math::mul_v3(dir, MVSPEED * ft));
		} else { newCamera.position = newCamera.posTarget; }
	}
	//* Update rotation */
	if !math::close_enough_f32(newCamera.rotation, newCamera.rotTarget, 5.0) {
		let dir = math::get_direction_f32(newCamera.rotation, newCamera.rotTarget);
		newCamera.rotation += dir * (CMSPEED * ft);
	} else {
		newCamera.rotation = newCamera.rotTarget;

		//* Bounds checking */
		if newCamera.rotation < 0.0 {
			newCamera.rotation += 360.0;
			newCamera.rotTarget += 360.0;
		}
		if newCamera.rotation > 360.0 {
			newCamera.rotation -= 360.0;
			newCamera.rotTarget -= 360.0;
		}

		//* Controls */
		if settings::button_down("rotate_right", &gamestate.settings) { newCamera.rotTarget -= 90.0; }
		if settings::button_down("rotate_left",  &gamestate.settings) { newCamera.rotTarget += 90.0; }
	}

	//* Calculate rotation */
	newCamera.camPosition = math::rotate(newCamera.position, newCamera.rotation);

	return newCamera;
}