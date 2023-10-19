

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{raylib, utilities::math, settings, data};
use raylib_ffi::Vector3;


//= Constants
/// Character movement speed.
const MVSPEED : f32 =   5.0;
/// Camera rotation speed.
const CMSPEED : f32 = 500.0;


//= Structures

/// Camera storage structure.
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
impl Camera {
	fn update_rotation(&mut self) {
		let ft = raylib::get_frame_time();

		//* Update rotation */
		if !math::close_enough_f32(self.rotation, self.rotTarget, 5.0) {
			let dir = math::get_direction_f32(self.rotation, self.rotTarget);
			self.rotation += dir * (CMSPEED * ft);
		} else {
			self.rotation = self.rotTarget;
	
			//* Bounds checking */
			if self.rotation < 0.0 {
				self.rotation += 360.0;
				self.rotTarget += 360.0;
			}
			if self.rotation > 360.0 {
				self.rotation -= 360.0;
				self.rotTarget -= 360.0;
			}
	
			//* Controls */
			unsafe {
				if data::SETTINGS.key_down("rotate_right") { self.rotTarget -= 90.0; }
				if data::SETTINGS.key_down("rotate_left")  { self.rotTarget += 90.0; }
			}
			//if settings::button_down("rotate_right", &gamestate.settings) { self.rotTarget -= 90.0; }
			//if settings::button_down("rotate_left",  &gamestate.settings) { self.rotTarget += 90.0; }
		}
	
		//* Calculate rotation */
		self.camPosition = math::rotate(self.position, self.rotation);
	}
}


//= Procedures

/// Initializes a new Camera structure.
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

/// Updates Camera position and rotation.
pub fn update( gamestate : &mut data::Gamestate ) {
	let ft = raylib::get_frame_time();

	//* Check if targetting a unit */
	if gamestate.camera.onPlayer {
		//* Have camera's movements match player */
		gamestate.camera.position = math::add_v3(gamestate.player.unit.position, Vector3{x:0.0,y:1.0,z:0.0});
		gamestate.camera.position.y = gamestate.camera.position.y / 2.0;
	} else {
		//* Update Position */
		if !math::close_enough_v3(gamestate.camera.position, gamestate.camera.posTarget, 0.5) {
			let dir = math::get_direction_v3(gamestate.camera.position, gamestate.camera.posTarget);
			gamestate.camera.position = math::add_v3(gamestate.camera.position, math::mul_v3(dir, MVSPEED * ft));
		} else { gamestate.camera.position = gamestate.camera.posTarget; }
	}
	gamestate.camera.update_rotation();
	////* Update rotation */
	//if !math::close_enough_f32(gamestate.camera.rotation, gamestate.camera.rotTarget, 5.0) {
	//	let dir = math::get_direction_f32(gamestate.camera.rotation, gamestate.camera.rotTarget);
	//	gamestate.camera.rotation += dir * (CMSPEED * ft);
	//} else {
	//	gamestate.camera.rotation = gamestate.camera.rotTarget;
//
	//	//* Bounds checking */
	//	if gamestate.camera.rotation < 0.0 {
	//		gamestate.camera.rotation += 360.0;
	//		gamestate.camera.rotTarget += 360.0;
	//	}
	//	if gamestate.camera.rotation > 360.0 {
	//		gamestate.camera.rotation -= 360.0;
	//		gamestate.camera.rotTarget -= 360.0;
	//	}
//
	//	//* Controls */
	//	if settings::button_down("rotate_right", &gamestate.settings) { gamestate.camera.rotTarget -= 90.0; }
	//	if settings::button_down("rotate_left",  &gamestate.settings) { gamestate.camera.rotTarget += 90.0; }
	//}
//
	////* Calculate rotation */
	//gamestate.camera.camPosition = math::rotate(gamestate.camera.position, gamestate.camera.rotation);
}