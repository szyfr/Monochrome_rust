

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{overworld, data};


//= Structures
pub struct Player {
	unit : overworld::Unit,

	canMove : bool,
}


//= Procedures
pub fn init( gamestate : &data::Gamestate ) -> Player {
	return Player {
		unit:		overworld::create_unit(gamestate),
		canMove:	true,
	}
}