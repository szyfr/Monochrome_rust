

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]



//= Imports
use std::{collections::HashMap, str::FromStr, fmt::Display};
use raylib_ffi::Vector3;
use crate::{raylib, utilities::debug};


//= Enumerations
#[derive(Copy, Clone)]
pub enum Direction {
	Null,
	North,
	South,
	East,
	West,
}
impl FromStr for Direction {
	type Err = ();
	fn from_str( input : &str ) -> Result<Direction, Self::Err> {
		match input {
			"null"	=> Ok(Direction::Null),
			"north"	=> Ok(Direction::North),
			"south"	=> Ok(Direction::South),
			"east"	=> Ok(Direction::East),
			"west"	=> Ok(Direction::West),
			_				=> Err(()),
		}
	}
}
impl Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Direction::Null		=> write!(f, "null"),
			Direction::North	=> write!(f, "north"),
			Direction::South	=> write!(f, "south"),
			Direction::East		=> write!(f, "east"),
			Direction::West		=> write!(f, "west"),
		}
	}
}

//= Structures
pub struct Unit {
	pub position	: Vector3,
	pub posTarget	: Vector3,

	pub direction	: Direction,

	pub animator	: Animator,
}
pub struct Animator {
	//mesh		: raylib_ffi::Mesh,
	//pub material	: raylib_ffi::Material,
	pub textures	: Vec<raylib_ffi::Texture>,

	pub currentAnimation : String,

	pub frame	: i32,
	pub counter : i32,
}
pub struct Animation {
	pub frames	: Vec<i32>,
	pub delay	: i32,
}


//= Procedures
pub fn create_unit( filename : &str ) -> Unit {
	let mut textures: Vec<raylib_ffi::Texture> = Vec::new();

	if raylib::is_window_ready() { textures = load_unit_textures(filename); }
	return Unit {
		position:	Vector3{x:0.0,y:0.0,z:0.0},
		posTarget:	Vector3{x:0.0,y:0.0,z:0.0},
		direction:	Direction::South,
		animator:	Animator{
			//mesh:		*gamestate.models["unit"].meshes,
			//material:	raylib::load_default_material(),
			textures:	textures,
			currentAnimation: "walk_south".to_string(),
			frame: 		0,
			counter: 	0,
		},
	};
}

pub fn load_unit_textures( filename : &str ) -> Vec<raylib_ffi::Texture> {
	//* Create full path */
	let mut fullPath = "data/sprites/overworld/".to_string();
	fullPath.push_str(filename);

	//* Load image */
	let img = raylib::load_image(&fullPath);
	let mut output: Vec<raylib_ffi::Texture> = Vec::new();

	//* Generate each texture from image */
	for i in 0..img.width/img.height {
		let subImg = raylib::image_from_image(img, raylib_ffi::Rectangle{
			x:(i*img.height) as f32,
			y:0.0,
			width:img.height as f32,
			height:img.height as f32,
		});
		output.push(raylib::load_texture_from_image(subImg));
		raylib::unload_image(subImg);
	}
	raylib::unload_image(img);

	return output;
}

pub fn draw_unit( animations : &HashMap<String, Animation>, model : raylib_ffi::Model, unit : Unit, rotation : f32 ) -> Unit {
	let mut newUnit = unit;
	if !animations.contains_key(&newUnit.animator.currentAnimation) {
		debug::log("[ERROR] - Attempting to use animation that doesn't exist.\n");
		print!("{}\n", newUnit.animator.currentAnimation);
		return newUnit;
	}
	let animation = &animations[&newUnit.animator.currentAnimation];

	//* Update animation */
	newUnit.animator.counter += 1;
	if animation.delay != 0 && newUnit.animator.counter >= animation.delay {
		newUnit.animator.counter = 0;
		newUnit.animator.frame += 1;
		if newUnit.animator.frame >= animation.frames.len() as i32 { newUnit.animator.frame = 0; }
	}

	//* Update material */
	let index = newUnit.animator.frame as usize;
	let frame = animation.frames[index] as usize;
	unsafe { (*(*model.materials).maps).texture = newUnit.animator.textures[frame]; }

	//* Draw */
	raylib::draw_model_ex(
		model,
		newUnit.position,
		raylib_ffi::Vector3{x:0.0,y:1.0,z:0.0},
		-rotation,
		raylib_ffi::Vector3{x:1.0,y:1.0,z:1.0},
		raylib_ffi::colors::WHITE,
	);

	return newUnit;
}