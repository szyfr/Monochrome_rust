

//= Allows


//= Imports
use raylib_ffi::Vector3;

use crate::{overworld::Direction, camera};


//= Constants
const XDIST : f32 = 0.0;
const YDIST : f32 = 5.0;
const ZDIST : f32 = 5.0;


//= Procedures
pub fn rotate( pos : Vector3, rot : f32 ) -> Vector3 {
	let mut position = Vector3{x:0.0,y:0.0,z:0.0};

	position.x = XDIST * (rot / 57.3).cos() - ZDIST * (rot / 57.3).sin();
	position.z = XDIST * (rot / 57.3).sin() + ZDIST * (rot / 57.3).cos();

	position.x += pos.x;
	position.y  = pos.y + YDIST;
	position.z += pos.z;

	return position;
}

pub fn close_enough_v3( v1 : Vector3, v2 : Vector3, offset : f32 ) -> bool {
	let mut output = true;

	if v1.x > v2.x + offset || v1.x < v2.x - offset { output = false; }
	if v1.y > v2.y + offset || v1.y < v2.y - offset { output = false; }
	if v1.z > v2.z + offset || v1.z < v2.z - offset { output = false; }

	return output;
}
pub fn close_enough_f32( f1 : f32, f2 : f32, offset : f32 ) -> bool {
	if f1 < f2 - offset || f1 > f2 + offset { return false; }

	return true
}

pub fn get_direction_v3( v1 : Vector3, v2 : Vector3 ) -> Vector3 {
	let difference = sub_v3(v2, v1);
	let mut output = Vector3{x:0.0,y:0.0,z:0.0};

	if difference.x  > 0.0 { output.x =  1.0 }
	if difference.x == 0.0 { output.x =  0.0 }
	if difference.x  < 0.0 { output.x = -1.0 }

	if difference.y  > 0.0 { output.y =  1.0 }
	if difference.y == 0.0 { output.y =  0.0 }
	if difference.y  < 0.0 { output.y = -1.0 }

	if difference.z  > 0.0 { output.z =  1.0 }
	if difference.z == 0.0 { output.z =  0.0 }
	if difference.z  < 0.0 { output.z = -1.0 }
	
	return output;
}
pub fn get_direction_f32( f1 : f32, f2 : f32 ) -> f32 {
	let difference = f2 - f1;

	if difference > 0.0 { return  1.0; }
	if difference < 0.0 { return -1.0; }

	return 0.0;
}

pub fn mul_v3( vec : Vector3, value : f32 ) -> Vector3 {
	return Vector3 { x: vec.x * value, y: vec.y * value, z: vec.z * value };	
}

pub fn add_v3( v1 : Vector3, v2 : Vector3 ) -> Vector3 {
	return Vector3 { x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z };
}
pub fn sub_v3( v1 : Vector3, v2 : Vector3 ) -> Vector3 {
	return Vector3 { x: v1.x - v2.x, y: v1.y - v2.y, z: v1.z - v2.z };
}

pub fn equal_v3( v1 : Vector3, v2 : Vector3 ) -> bool {
	if v1.x != v2.x { return false; }
	if v1.y != v2.y { return false; }
	if v1.z != v2.z { return false; }

	return true;
}

pub fn get_relative_direction_dir( camera : &camera::Camera, direction : Direction ) -> Direction {
	if (camera.rotation > -45.0 && camera.rotation <=  45.0) || (camera.rotation > 315.0 && camera.rotation <= 405.0) {
		match direction {
			Direction::North => return Direction::North,
			Direction::South => return Direction::South,
			Direction::East  => return Direction::East,
			Direction::West  => return Direction::West,
			_ => return Direction::Null,
		}
	}
	if (camera.rotation >  45.0 && camera.rotation <= 135.0) || (camera.rotation > 405.0 && camera.rotation <= 495.0) {
		match direction {
			Direction::North => return Direction::East,
			Direction::South => return Direction::West,
			Direction::East  => return Direction::South,
			Direction::West  => return Direction::North,
			_ => return Direction::Null,
		}
	}
	if camera.rotation > 135.0 && camera.rotation <= 225.0 {
		match direction {
			Direction::North => return Direction::South,
			Direction::South => return Direction::North,
			Direction::East  => return Direction::West,
			Direction::West  => return Direction::East,
			_ => return Direction::Null,
		}
	}
	if (camera.rotation > 225.0 && camera.rotation <= 315.0) || (camera.rotation > -135.0 && camera.rotation <= -45.0) {
		match direction {
			Direction::North => return Direction::West,
			Direction::South => return Direction::East,
			Direction::East  => return Direction::North,
			Direction::West  => return Direction::South,
			_ => return Direction::Null,
		}
	}

	return Direction::Null;
}