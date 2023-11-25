

//= Allows


//= Imports
use crate::overworld::Direction;


//= Constants


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