

//= Allows


//= Imports
use raylib_ffi::Vector3;


//= Procedures
pub fn close_enough( v1 : Vector3, v2 : Vector3, offset : f32 ) -> bool {
	let mut output = true;

	if v1.x > v2.x + offset || v1.x < v2.x - offset { output = false; }
	if v1.y > v2.y + offset || v1.y < v2.y - offset { output = false; }
	if v1.z > v2.z + offset || v1.z < v2.z - offset { output = false; }

	return output;
}

pub fn mul_v3( vec : Vector3, value : f32 ) -> Vector3 {
	return Vector3 { x: vec.x * value, y: vec.y * value, z: vec.z * value };	
}

pub fn add_v3( v1 : Vector3, v2 : Vector3 ) -> Vector3 {
	return Vector3 { x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z };
}