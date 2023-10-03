

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{raylib, data, utilities::math::{close_enough, mul_v3, add_v3}};
use raylib_ffi::Vector3;


//= Constants
const MVSPEED : f32 = 5.0;


//= Structures
pub struct Camera {
	// ! TEST
	pub position	: Vector3,
	pub target		: Vector3,
	pub fovy		: f32,

	posTarget	: Vector3,
	rotation	: f32,
	rotTarget	: f32,

	// TODO Figure out a better way to do this
	targetUnit	: i32,
}


//= Procedures
pub fn init() -> Camera {
	return Camera{
		position:	Vector3 {x:0.5,y:7.0,z:5.5},
		target:		Vector3 {x:0.5,y:0.0,z:0.5},
		fovy:		70.0,

		posTarget:	Vector3 {x:0.5,y:0.0,z:0.5},
		rotation:	0.0,
		rotTarget:	0.0,

		targetUnit:	0,
	};
}

pub fn update( mut gamestate : data::Gamestate ) {
	let camera = gamestate.camera;
	let ft = raylib::get_frame_time();

	//* Check if targetting a unit */
	if camera.targetUnit != -1 {
		//camera.position	= gamestate.world.units[camera.targetUnit] + Vector3{0.0,1.0,0.0};
		//camera.target		= gamestate.world.units[camera.targetUnit] + Vector3{0.0,1.0,0.0};
	} else {
		//* Update Position */
		if close_enough(camera.position, camera.posTarget, 0.5) {
			//let dir = Vector3{x:1.0,y:1.0,z:1.0};
			//gamestate.camera.position = add_v3(gamestate.camera.position, mul_v3(dir, (MVSPEED * ft)));
		}
	}
}