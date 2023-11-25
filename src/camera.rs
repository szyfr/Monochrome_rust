

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{raylib::{self, structures::Vector3}, utilities::math, data};


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

	pub onPlayer	: bool,
}


//= Procedures

impl Camera {

	/// Initializes a new Camera structure.
	pub fn init() -> Self {
		Self {
			position:		Vector3 {x:0.5,y:0.0,z:0.5},
			posTarget:		Vector3 {x:0.5,y:0.0,z:0.5},

			rotation:		0.0,
			rotTarget:		0.0,

			camPosition:	Vector3 {x:0.5,y:7.0,z:5.5},
			fovy:			70.0,

			onPlayer:		true,
		}
	}

	/// Update camera
	pub fn update(&mut self, playerPos: Vector3, control: bool) {
		let ft = raylib::get_frame_time();

		//* Check if targetting a unit, and update position accordingly */
		if self.onPlayer {
			//* Have camera's movements match player */
			self.position = playerPos + Vector3{x:0.0,y:1.0,z:0.0};
			self.position.y = self.position.y / 2.0;
			self.posTarget = self.position;
		} else {
			//* Update Position */
			if self.position.close(self.posTarget, 0.5) {
				let dir = self.position.direction_to(self.posTarget);
				self.position = self.position + (dir * (MVSPEED * ft));
			} else { self.position = self.posTarget; }
		}

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
			if control {
				if data::key_down("rotate_right") { self.rotTarget -= 90.0; }
				if data::key_down("rotate_left")  { self.rotTarget += 90.0; }
			}
		}
	
		//* Calculate rotation */
		self.camPosition = self.position.rotate(Vector3{x: 0.0, y: 6.0, z: 5.0}, self.rotation);
	}

}
