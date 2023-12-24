

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld::Direction, raylib::vectors::Vector3};


//= Constants
const LOOKUP: [Vector3;4] = [
	Vector3{x: 1.0, y:0.0, z: 0.0},
	Vector3{x: 0.0, y:0.0, z: 1.0},
	Vector3{x:-1.0, y:0.0, z: 0.0},
	Vector3{x: 0.0, y:0.0, z:-1.0},
];


//= Procedures

/// Returns ``true`` if inputted ``float``s are within ``offset`` of each other.
pub fn close_enough_f32( f1 : f32, f2 : f32, offset : f32 ) -> bool {
	if f1 < f2 - offset || f1 > f2 + offset { return false; }

	return true
}

/// Returns whether the difference between input floats is positive, negative, or zero.
pub fn get_direction_f32( f1 : f32, f2 : f32 ) -> f32 {
	let difference = f2 - f1;

	if difference > 0.0 { return  1.0; }
	if difference < 0.0 { return -1.0; }

	return 0.0;
}

/// Returns the Direction a unit would be facing relative to the camera given their current true direction.
pub fn get_relative_direction_dir( rotation : f32, direction : Direction ) -> Direction {
	if (rotation > -45.0 && rotation <=  45.0) || (rotation > 315.0 && rotation <= 405.0) {
		match direction {
			Direction::North => return Direction::North,
			Direction::South => return Direction::South,
			Direction::East  => return Direction::East,
			Direction::West  => return Direction::West,
		}
	}
	if (rotation >  45.0 && rotation <= 135.0) || (rotation > 405.0 && rotation <= 495.0) {
		match direction {
			Direction::North => return Direction::East,
			Direction::South => return Direction::West,
			Direction::East  => return Direction::South,
			Direction::West  => return Direction::North,
		}
	}
	if rotation > 135.0 && rotation <= 225.0 {
		match direction {
			Direction::North => return Direction::South,
			Direction::South => return Direction::North,
			Direction::East  => return Direction::West,
			Direction::West  => return Direction::East,
		}
	}
	if (rotation > 225.0 && rotation <= 315.0) || (rotation > -135.0 && rotation <= -45.0) {
		match direction {
			Direction::North => return Direction::West,
			Direction::South => return Direction::East,
			Direction::East  => return Direction::North,
			Direction::West  => return Direction::South,
		}
	}

	return Direction::South;
}

/// Check whether position is range of another
pub fn is_within_range(p1: Vector3, p2: Vector3, range: i32) -> bool {
	//let distance = distance(p1, p2) as i32;

	let oldDistance = distance(p1, p2);
	let mut newDistance = oldDistance as f32;
	let mut alteredPosition = p1;
	let mut position = 0;
	let mut steps = 0;

	while alteredPosition != p2 {
		for i in 0..4 {
			let tempDist = distance(alteredPosition + Vector3::from(LOOKUP[i]),p2);
			if newDistance > tempDist {
				newDistance = tempDist;
				position = i;
			}
		}
		alteredPosition = alteredPosition + Vector3::from(LOOKUP[position]);
		steps += 1;
	}

	return steps <= range;
}

pub fn distance(p1: Vector3, p2: Vector3) -> f32 {
	return ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt();
}