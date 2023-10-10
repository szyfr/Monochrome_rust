

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, str::FromStr, fmt::Display};
use raylib_ffi::Vector3;
use crate::{raylib, utilities::{debug, math::close_enough_v3}, data, events};


//= Enumerations
#[derive(Copy, Clone, PartialEq)]
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
#[derive(Clone)]
pub struct Unit {
	pub position	: Vector3,
	pub posTarget	: Vector3,

	pub direction	: Direction,

	pub id			: String,
	pub events		: [Option<events::EntityEvent>; 5],
	pub conditions	: [Option<events::Condition>; 5],

	pub animator	: Animator,
}
impl Unit {
	pub fn copy_unit( &self ) -> Unit {
		let mut output = Unit{
			position: self.position,
			posTarget: self.posTarget,
			direction: self.direction,
			id: self.id.to_string(),
			events: events::create_empty_entityevents(),
			conditions: events::create_empty_conditions(),
			animator: Animator{
				textures: Vec::new(),
				currentAnimation: self.animator.currentAnimation.to_string(),
				frame: self.animator.frame,
				counter: self.animator.counter,
			}
		};
		for i in 0..5 {
			if self.events[i].is_none() { break; }
			output.events[i] = Some(events::EntityEvent{
				val: self.events[i].clone().expect("").val,
				key: self.events[i].clone().expect("").key,
			});
		}
		for i in 0..5 {
			if self.conditions[i].is_none() { break; }
			output.conditions[i] = Some(events::Condition{
				val: self.conditions[i].clone().expect("").val,
				key: self.conditions[i].clone().expect("").key,
			});
		}
		for i in 0..self.animator.textures.len() {
			output.animator.textures.push(self.animator.textures[i]);
		}
		return output;
	}
}
#[derive(Clone)]
pub struct Animator {
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
		id:			"".to_string(),
		events:		events::create_empty_entityevents(),
		conditions:	events::create_empty_conditions(),
		animator:	Animator{
			textures:	textures,
			currentAnimation: "walk_south".to_string(),
			frame: 		0,
			counter: 	0,
		},
	};
}

pub fn load_unit_textures( filename : &str ) -> Vec<raylib_ffi::Texture> {
	//* Create full path */
	let fullPath = "data/sprites/overworld/".to_string() + filename + ".png";

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
		raylib_ffi::Vector3{x:newUnit.position.x, y: newUnit.position.y/2.0, z: newUnit.position.z},
		raylib_ffi::Vector3{x:0.0,y:1.0,z:0.0},
		-rotation,
		raylib_ffi::Vector3{x:1.0,y:1.0,z:1.0},
		raylib_ffi::colors::WHITE,
	);

	return newUnit;
}

pub fn empty() -> Unit {
	return Unit {
		position:	Vector3{x:0.0,y:0.0,z:0.0},
		posTarget:	Vector3{x:0.0,y:0.0,z:0.0},
		direction:	Direction::South,
		id:			"".to_string(),
		events:		events::create_empty_entityevents(),
		conditions:	events::create_empty_conditions(),
		animator:	Animator{
			textures:	Vec::new(),
			currentAnimation: "walk_south".to_string(),
			frame: 		0,
			counter: 	0,
		},
	};
}

pub fn set_animation( unit : Unit, animation : String ) -> Unit {
	let mut newUnit = unit;

	if newUnit.animator.currentAnimation != animation {
		newUnit.animator.counter = 0;
		newUnit.animator.frame = 0;
		newUnit.animator.currentAnimation = animation;
	}

	return newUnit;
}

pub fn move_unit( gamestate : &data::Gamestate, unit : Unit, direction : Direction ) -> Unit {
	let mut newUnit = unit;

	if !close_enough_v3(newUnit.position, newUnit.posTarget, 0.05) { return newUnit; }
	if newUnit.direction == Direction::Null { return newUnit; }

	let mut newPos = newUnit.position;
	
	match direction {
		Direction::North => newPos.z += -1.0,
		Direction::South => newPos.z +=  1.0,
		Direction::East  => newPos.x += -1.0,
		Direction::West  => newPos.x +=  1.0,
		_ => newPos = newUnit.position,
	}

	//* Check Tiles existance */
	let tileExists = gamestate.currentMap.contains_key(&[newPos.x as i32, newPos.y as i32, newPos.z as i32]);
	if !tileExists {
		//TODO If the reverse movement would not be allowed, jump
		//* Checking for tile up */
		let tileExistsUp = gamestate.currentMap.contains_key(&[newPos.x as i32, (newPos.y as i32)+1, newPos.z as i32]);
		let mut tileUpColli = false;
		if tileExistsUp {
			let tileUp = &gamestate.currentMap[&[newPos.x as i32, (newPos.y as i32)+1, newPos.z as i32]];
			tileUpColli = !check_collision(direction, tileUp.solid);
			if tileUpColli { newPos.y += 1.0; }
		}
		//* Checking for tile down */
		let tileExistsDw = gamestate.currentMap.contains_key(&[newPos.x as i32, (newPos.y as i32)-1, newPos.z as i32]);
		let mut tileDwColli = false;
		if tileExistsDw {
			let tileDw = &gamestate.currentMap[&[newPos.x as i32, (newPos.y as i32)-1, newPos.z as i32]];
			tileDwColli = !check_collision(direction, tileDw.solid);
			if tileDwColli { newPos.y -= 1.0; }
		}
		if !(tileExistsUp && tileUpColli) && !(tileDwColli && tileDwColli) { return newUnit; }
	}
	let tile = &gamestate.currentMap[&[newPos.x as i32, newPos.y as i32, newPos.z as i32]];

	//* Check if Solid */
	if check_collision(direction, tile.solid) { return newUnit; }

	//* Check for entities
	// TODO

	newUnit.posTarget = newPos;
	return newUnit;
}

fn check_collision( direction : Direction, collisionInfo : [bool; 4] ) -> bool {
	match direction {
		Direction::North => return collisionInfo[0],
		Direction::South => return collisionInfo[2],
		Direction::East  => return collisionInfo[3],
		Direction::West  => return collisionInfo[1],
		_ => return true,
	}
}