

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, str::FromStr, fmt::Display};
use crate::{raylib, utilities::{debug, math::{close_enough_v3, self}}, world, events::{self, conditionals::Condition}};


//= Enumerations

/// Unit facing direction.
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

/// Represents an overworld Character/Item. Also used with the Player.
#[derive(Clone)]
pub struct Unit {
	pub position	: raylib_ffi::Vector3,
	pub posTarget	: raylib_ffi::Vector3,

	pub direction	: Direction,

	pub id			: String,
	pub events		: HashMap<String, HashMap<String, events::conditionals::Condition>>,
	pub conditions	: HashMap<String, events::conditionals::Condition>,

	pub animator	: Animator,
}

/// The animation controller for Units.
#[derive(Clone)]
pub struct Animator {
	pub textures	: Vec<raylib_ffi::Texture>,

	pub currentAnimation : String,

	pub frame	: i32,
	pub counter : i32,
}

/// Storage format for animations loaded from file.
pub struct Animation {
	pub frames	: Vec<i32>,
	pub delay	: i32,
}


//= Procedures

/// Creates a new Unit.<br>
/// If the Raylib window is ready, input filename will be loaded as the texture. It will be dropped otherwise.
pub fn create_unit( filename : &str ) -> Unit {
	let mut textures: Vec<raylib_ffi::Texture> = Vec::new();

	if raylib::is_window_ready() { textures = load_unit_textures(filename); }
	return Unit {
		position:	raylib_ffi::Vector3{x:0.0,y:0.0,z:0.0},
		posTarget:	raylib_ffi::Vector3{x:0.0,y:0.0,z:0.0},
		direction:	Direction::South,
		id:			"".to_string(),
		events:		HashMap::new(),
		conditions:	HashMap::new(),
		animator:	Animator{
			textures:	textures,
			currentAnimation: "walk_south".to_string(),
			frame: 		0,
			counter: 	0,
		},
	};
}

/// Concatenates the full path to the input and loads the respective Image into a ``Vec<Texture>``.
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

/// Draws the input Unit in the world as well as updating the Unit's animations.
pub fn draw_unit( animations : &HashMap<String, Animation>, model : raylib_ffi::Model, unit : &mut Unit, rotation : f32 ) {
	//* Check if animation exists */
	if !animations.contains_key(&unit.animator.currentAnimation) {
		debug::log("[ERROR] - Attempting to use animation that doesn't exist.\n");
		return;
	}
	let animation = &animations[&unit.animator.currentAnimation];

	//* Check animations */
	if !math::equal_v3(unit.position, unit.posTarget) {
		let dir = math::get_relative_direction_dir(rotation, unit.direction);
		set_animation(unit, format!("walk_{}",dir))
	} else {
		let dir = math::get_relative_direction_dir(rotation, unit.direction);
		set_animation(unit, format!("idle_{}",dir))
	}

	//* Update animation */
	unit.animator.counter += 1;
	if animation.delay != 0 && unit.animator.counter >= animation.delay {
		unit.animator.counter = 0;
		unit.animator.frame += 1;
		if unit.animator.frame >= animation.frames.len() as i32 { unit.animator.frame = 0; }
	}

	//* Update material */
	let index = unit.animator.frame as usize;
	let frame = animation.frames[index] as usize;
	unsafe { (*(*model.materials).maps).texture = unit.animator.textures[frame]; }

	//* Draw */
	raylib::draw_model_ex(
		model,
		raylib_ffi::Vector3{x: unit.position.x, y: unit.position.y/2.0, z: unit.position.z},
		raylib_ffi::Vector3{x:0.0,y:1.0,z:0.0},
		-rotation,
		raylib_ffi::Vector3{x:1.0,y:1.0,z:1.0},
		raylib_ffi::colors::WHITE,
	);
}

/// Sets the Unit's current animation
pub fn set_animation( unit : &mut Unit, animation : String ) {
	//* Check if new animation is the not the one currently playing */
	if unit.animator.currentAnimation != animation {
		//* Reset all variables */
		unit.animator.counter = 0;
		unit.animator.frame = 0;
		unit.animator.currentAnimation = animation;
	}
}

/// Calculates whether the Unit can move in the input direction and if possible set them to move.
//pub fn move_unit( worldData : &world::World, unit : &mut Unit, direction : Direction ) {
pub fn move_unit( currentMap : &HashMap<[i32;3], world::Tile>, unitMap : &HashMap<String, Unit>, eventHandler : &events::event_handler::EventHandler, unit : &mut Unit, direction : Direction ) {
	//* Leave if still moving or current direction is Null */
	if !close_enough_v3(unit.position, unit.posTarget, 0.05) { return; }
	if unit.direction == Direction::Null { return; }

	//* Calculate new position */
	let mut newPos = unit.position;
	match direction {
		Direction::North => newPos.z += -1.0,
		Direction::South => newPos.z +=  1.0,
		Direction::East  => newPos.x += -1.0,
		Direction::West  => newPos.x +=  1.0,
		_ => newPos = unit.position,
	}

	//* Check Tiles existance */
	let tileExists = currentMap.contains_key(&[newPos.x as i32, newPos.y as i32, newPos.z as i32]);
	if !tileExists {
		//TODO If the reverse movement would not be allowed, jump
		//* Checking for tile up */
		let tileExistsUp = currentMap.contains_key(&[newPos.x as i32, (newPos.y as i32)+1, newPos.z as i32]);
		let mut tileUpColli = false;
		if tileExistsUp {
			let tileUp = &currentMap[&[newPos.x as i32, (newPos.y as i32)+1, newPos.z as i32]];
			tileUpColli = !check_collision(direction, tileUp.solid);
			if tileUpColli { newPos.y += 1.0; }
		}
		//* Checking for tile down */
		let tileExistsDw = currentMap.contains_key(&[newPos.x as i32, (newPos.y as i32)-1, newPos.z as i32]);
		let mut tileDwColli = false;
		if tileExistsDw {
			let tileDw = &currentMap[&[newPos.x as i32, (newPos.y as i32)-1, newPos.z as i32]];
			tileDwColli = !check_collision(direction, tileDw.solid);
			if tileDwColli { newPos.y -= 1.0; }
		}
		if !(tileExistsUp && tileUpColli) && !(tileDwColli && tileDwColli) { return; }
	}
	let tile = &currentMap[&[newPos.x as i32, newPos.y as i32, newPos.z as i32]];

	//* Check if Solid */
	if check_collision(direction, tile.solid) { return; }

	//* Check for entities */
	for (_, unit) in unitMap.iter() {
		if math::equal_v3(newPos ,unit.position) && exists(&eventHandler, unit) { return; }
	}

	unit.posTarget = newPos;
}

///

/// Returns whether the Unit should exist.
pub fn exists( handler : &events::event_handler::EventHandler, unit : &Unit ) -> bool {
	let mut result = true;

	if unit.conditions.len() == 0 { return true; }

	for (str, cond) in &unit.conditions {
		match cond {
			Condition::Integer(_) =>
				if handler.eventVariables.contains_key(str) {
					if handler.eventVariables[str] != *cond { result = false; }
				} else {
					if *cond != Condition::Integer(0) { result = false; }
				},
			Condition::Boolean(_) =>
				if handler.eventVariables.contains_key(str) {
					if handler.eventVariables[str] != *cond { result = false; }
				} else {
					if *cond != Condition::Boolean(false) { result = false; }
				},
		}
	}
	return result;
}

pub fn check_conditions( handler : &events::event_handler::EventHandler, conditions : &HashMap<String, events::conditionals::Condition> ) -> bool {
	let mut result = true;
	
	for (str, cond) in conditions {
		if handler.eventVariables.contains_key(str) {
			if handler.eventVariables[str] == *cond { result = true; }
		} else if *cond == events::conditionals::Condition::Boolean(false) { result = true; }
	}
	return result;
}

/// Checks if there is a Unit in that position.
pub fn check_for_unit( unitMap : &HashMap<String, Unit>, position : &[i32;3] ) -> (bool, String) {
	for (str, unit) in unitMap {
		if [unit.position.x as i32, unit.position.y as i32, unit.position.z as i32] == *position { return (true, str.to_string()); }
	}
	return (false, "".to_string());
}

/// Calculates collision.
fn check_collision( direction : Direction, collisionInfo : [bool; 4] ) -> bool {
	match direction {
		Direction::North => return collisionInfo[0],
		Direction::South => return collisionInfo[2],
		Direction::East  => return collisionInfo[3],
		Direction::West  => return collisionInfo[1],
		_ => return true,
	}
}

/// Checks if current animation is walking
fn check_walking_animation( unit : &Unit ) -> bool {
	match unit.animator.currentAnimation.as_str() {
		"walk_north" => return true,
		"walk_south" => return true,
		"walk_east"  => return true,
		"walk_west"  => return true,
		_ => return false,
	}
}