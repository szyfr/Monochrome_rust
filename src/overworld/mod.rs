

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, str::FromStr, fmt::Display};
use crate::{raylib, utilities::{debug, math::{close_enough_v3, self}}, data, events::{self, ConditionType}};


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
	pub events		: Vec<events::EntityEvent>,
	pub conditions	: Vec<events::Condition>,

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
		events:		Vec::new(),
		conditions:	Vec::new(),
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
pub fn draw_unit( animations : &HashMap<String, Animation>, model : raylib_ffi::Model, unit : Unit, rotation : f32 ) -> Unit {
	let mut newUnit = unit;

	//* Check if animation exists */
	if !animations.contains_key(&newUnit.animator.currentAnimation) {
		debug::log("[ERROR] - Attempting to use animation that doesn't exist.\n");
		return newUnit;
	}
	let animation = &animations[&newUnit.animator.currentAnimation];

	//* Check animations */
	if !math::equal_v3(newUnit.position, newUnit.posTarget) {
		let dir = math::get_relative_direction_dir(rotation, newUnit.direction);
		newUnit = set_animation(newUnit, format!("walk_{}",dir))
	} else {
		let dir = math::get_relative_direction_dir(rotation, newUnit.direction);
		newUnit = set_animation(newUnit, format!("idle_{}",dir))
	}

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

/// Sets the Unit's current animation
pub fn set_animation( unit : Unit, animation : String ) -> Unit {
	let mut newUnit = unit;

	//* Check if new animation is the not the one currently playing */
	if newUnit.animator.currentAnimation != animation {
		//* Reset all variables */
		newUnit.animator.counter = 0;
		newUnit.animator.frame = 0;
		newUnit.animator.currentAnimation = animation;
	}

	return newUnit;
}

/// Calculates whether the Unit can move in the input direction and if possible set them to move.
pub fn move_unit( gamestate : &data::Gamestate, unit : Unit, direction : Direction ) -> Unit {
	let mut newUnit = unit;

	//* Leave if still moving or current direction is Null */
	if !close_enough_v3(newUnit.position, newUnit.posTarget, 0.05) { return newUnit; }
	if newUnit.direction == Direction::Null { return newUnit; }

	//* Calculate new position */
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

	//* Check for entities */
	for (_, unit) in gamestate.unitMap.iter() {
		if math::equal_v3(newPos ,unit.position) && exists(&gamestate.eventHandler, unit) { return newUnit; }
	}

	newUnit.posTarget = newPos;
	return newUnit;
}

/// Returns whether the Unit should exist.
// TODO I might want to change this to a result declared at the start that gets changed by the match so that multiple variables can decide it?
pub fn exists( handler : &events::EventHandler, unit : &Unit ) -> bool {
	if unit.conditions.len() == 0 { return true; }
	for i in unit.conditions.iter() {
		unsafe {
			//print!("{}\n",i.value.bl);
			match i.condType {
				ConditionType::Boolean => if (handler.eventVariables.contains_key(&i.key) && handler.eventVariables[&i.key].bl == i.value.bl) || (!handler.eventVariables.contains_key(&i.key) && i.value.bl == false) { return true; },
				ConditionType::Integer => if (handler.eventVariables.contains_key(&i.key) && handler.eventVariables[&i.key].int == i.value.int) || (!handler.eventVariables.contains_key(&i.key) && i.value.int == 0) { return true; },
				_ => return false,
			}
		}
	}
	return false;
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