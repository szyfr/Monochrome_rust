

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::collections::LinkedList;

//= Imports
use raylib_ffi::Vector3;
use crate::{data, raylib};


//= Enumerations
pub enum Direction {
	Null,
	North,
	South,
	East,
	West,
}


//= Structures
pub struct Unit {
	position	: Vector3,
	posTarget	: Vector3,

	direction	: Direction,

	animator	: Animator,
}

pub struct Animator {
	mesh		: raylib_ffi::Mesh,
	material	: raylib_ffi::Material,
	textures	: LinkedList<raylib_ffi::Texture>,

	currentAnimation : String,

	frame	: i32,
	counter : i32,
}
pub struct Animation {
	frames	: LinkedList<i32>,
	delay	: i32,
}


//= Procedures
pub fn create_unit( gamestate : &data::Gamestate ) -> Unit {
	unsafe {
		return Unit {
			position:	Vector3{x:0.0,y:0.0,z:0.0},
			posTarget:	Vector3{x:0.0,y:0.0,z:0.0},
			direction:	Direction::North,
			animator:	Animator{
				mesh:		*gamestate.models["unit"].meshes,
				material:	raylib::load_default_material(),
				textures:	LinkedList::new(),
				currentAnimation: "idle_down".to_string(),
				frame: 		0,
				counter: 	0,
			},
		};
	}
}