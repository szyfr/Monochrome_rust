

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::overworld;


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