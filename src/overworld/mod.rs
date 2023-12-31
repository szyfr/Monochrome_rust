

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, str::FromStr, fmt::Display};
use crate::{raylib::{self, vectors::Vector3}, utilities::{debug, math}, events::{self, conditionals::Condition}, data, graphics};


//= Enumerations

/// Unit facing direction.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
	North,
	South,
	East,
	West,
}
impl FromStr for Direction {
	type Err = ();
	fn from_str( input : &str ) -> Result<Direction, Self::Err> {
		match input {
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
			Direction::North	=> write!(f, "north"),
			Direction::South	=> write!(f, "south"),
			Direction::East		=> write!(f, "east"),
			Direction::West		=> write!(f, "west"),
		}
	}
}

/// Result of an attemted move
#[derive(Copy, Clone, PartialEq)]
pub enum MovementResult {
	Worked,
	Blocked,
	DoesntExist,
	Moving,
}
impl Display for MovementResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			MovementResult::Worked		=> write!(f, "worked"),
			MovementResult::Blocked		=> write!(f, "blocked"),
			MovementResult::DoesntExist	=> write!(f, "doesn't exist"),
			MovementResult::Moving		=> write!(f, "moving"),
		}
	}
}

//= Structures

/// Represents an overworld Character/Item. Also used with the Player.
#[derive(Clone)]
pub struct Unit {
	pub position	: Vector3,
	pub posTarget	: Vector3,

	pub direction	: Direction,

	pub id			: String,
	pub events		: HashMap<String, HashMap<String, events::conditionals::Condition>>,
	pub conditions	: HashMap<String, events::conditionals::Condition>,

	pub animator	: Animator,
}

/// The animation controller for Units.
#[derive(Clone)]
pub struct Animator {
	pub texture: String,

	pub currentAnimation: String,

	pub frame:		i32,
	pub counter:	i32,
}

/// Storage format for animations loaded from file.
pub struct Animation {
	pub frames	: Vec<i32>,
	pub delay	: i32,
}


//= Procedures

impl Direction {

	/// Reverse the direction
	pub fn reverse(&self) -> Direction {
		match self {
			Direction::North => return Direction::South,
			Direction::South => return Direction::North,
			Direction::East => return Direction::West,
			Direction::West => return Direction::East,
		}
	}

}

impl Animator {

	/// Create new blank animator
	pub fn new() -> Self {
		return Animator {
			texture:	"".to_string(),
			currentAnimation:	"idle_south".to_string(),
			frame: 		0,
			counter:	0,
		}
	}

	/// Sets the Unit's current animation
	pub fn set_animation(&mut self, animation: String) {
		//* Check if new animation is the not the one currently playing */
		if self.currentAnimation != animation {
			//* Reset all variables */
			self.counter = 0;
			self.frame = 0;
			self.currentAnimation = animation;
		}
	}

}

impl Unit {

	/// Create new blank unit
	pub fn new() -> Self {
		return Unit {
			position:	Vector3::zero(),
			posTarget:	Vector3::zero(),
			direction:	Direction::South,
			id:			"".to_string(),
			events:		HashMap::new(),
			conditions:	HashMap::new(),
			animator:	Animator::new(),
		}
	}

	/// Draw unit
	pub fn draw(&mut self, graphics: &graphics::Graphics, rotation: f32) -> &Self {
		//* Check if unit HAS a sprite */
		if self.animator.texture == "".to_string() { return self; }

		//* Check if animation exists */
		if !graphics.animations.contains_key(&self.animator.currentAnimation) {
			debug::log("[ERROR] - Attempting to use animation that doesn't exist.\n");
			return self;
		}
		let animation = &graphics.animations[&self.animator.currentAnimation];

		//* Check animations */
		if self.position == self.posTarget {
			let dir = math::get_relative_direction_dir(rotation, self.direction);
			self.animator.set_animation(format!("idle_{}",dir));
		} else {
			let dir = math::get_relative_direction_dir(rotation, self.direction);
			self.animator.set_animation(format!("walk_{}",dir));
		}

		//* Update animation */
		self.animator.counter += 1;
		if animation.delay != 0 && self.animator.counter >= animation.delay {
			self.animator.counter = 0;
			self.animator.frame += 1;
			if self.animator.frame >= animation.frames.len() as i32 { self.animator.frame = 0; }
		}

		//* Update material */
		let index = self.animator.frame as usize;
		let frame = animation.frames[index] as usize;
		let texture = graphics.textures[&(self.animator.texture.to_string() + "_" + &frame.to_string())];
		let mut model = graphics.models["unit"].clone();
		model.set_material_texture(texture);

		//* Draw */
		model.draw_ex(
			Vector3{
				x: self.position.x,
				y: self.position.y/2.0,
				z: self.position.z,
			},
			Vector3{x:0.0,y:1.0,z:0.0},
			-rotation,
			Vector3{x:1.0,y:1.0,z:1.0},
			raylib_ffi::colors::WHITE,
		);

		return self;
	}

	/// Returns whether the Unit should be drawn / Interracted with.
	pub fn exists(&self, eventHandler : &events::event_handler::EventHandler) -> bool {
		let mut result = true;

		//* Check if there are any conditions */
		if self.conditions.len() == 0 { return true; }

		for (str, cond) in &self.conditions {
			match cond {
				Condition::Integer(_) => {
					if eventHandler.eventVariables.contains_key(str) {
						if eventHandler.eventVariables[str] != *cond {  result = false; }
					} else {
						if *cond != Condition::Integer(0) { result = false; }
					}
				}
				Condition::Boolean(_) => {
					if eventHandler.eventVariables.contains_key(str) {
						if eventHandler.eventVariables[str] != *cond {  result = false; }
					} else {
						if *cond != Condition::Boolean(false) { result = false; }
					}
				}
				Condition::String(_) => {
					if eventHandler.eventVariables.contains_key(str) {
						if eventHandler.eventVariables[str] != *cond {  result = false; }
					} else {
						if *cond != Condition::String("".to_string()) { result = false; }
					}
				}
			}
		}
		return result;
	}

	/// Checks if unit is moving
	pub fn is_moving(gamestate: &data::Gamestate, unitId : &str) -> bool {
		let unit = &gamestate.worldData.unitMap[unitId];
		return unit.position == unit.posTarget;
	}

	/// Update unit
	pub fn update(&mut self) -> &Self {
		let ft = raylib::get_frame_time();
		
		if !self.position.close(self.posTarget, 0.05) {
			let dir = self.position.direction_to(self.posTarget);
			self.position = self.position + (dir * (3.0 * ft));
		} else {
			self.position = self.posTarget;
		}

		return self;
	}

	/// Calculates whether the Unit can move in the input direction and if possible set them to move.
	pub fn walk(gamestate: &mut data::Gamestate, unitId : &str, direction : Direction) -> MovementResult {
		//* Get unit */
		let mut unitMove: Unit;
		if unitId == "player" { unitMove = gamestate.player.unit.clone(); }
		else if gamestate.worldData.unitMap.contains_key(unitId) { unitMove = gamestate.worldData.unitMap.get(unitId).unwrap().clone(); }
		else { return MovementResult::DoesntExist; }

		//* Leave if still moving */
		if !unitMove.position.close(unitMove.posTarget, 0.05) { return MovementResult::Moving; }

		//* Set direction */
		unitMove.direction = direction;

		//* Calculate new position */
		let mut newPos = unitMove.position;
		match direction {
			Direction::North => newPos.z += -1.0,
			Direction::South => newPos.z +=  1.0,
			Direction::East  => newPos.x += -1.0,
			Direction::West  => newPos.x +=  1.0,
		}

		//* Check Tiles existance */
		let tileExists = gamestate.worldData.currentMap.contains_key(&[newPos.x as i32, newPos.y as i32, newPos.z as i32]);
		if !tileExists {
			//TODO If the reverse movement would not be allowed, jump
			//* Checking for tile up */
			let tileExistsUp = gamestate.worldData.currentMap.contains_key(&[newPos.x as i32, (newPos.y as i32)+1, newPos.z as i32]);
			let mut tileUpColli = false;
			if tileExistsUp {
				let tileUp = &gamestate.worldData.currentMap[&[newPos.x as i32, (newPos.y as i32)+1, newPos.z as i32]];
				tileUpColli = !check_collision(direction, tileUp.solid);
				if tileUpColli { newPos.y += 1.0; }
			}
			//* Checking for tile down */
			let tileExistsDw = gamestate.worldData.currentMap.contains_key(&[newPos.x as i32, (newPos.y as i32)-1, newPos.z as i32]);
			let mut tileDwColli = false;
			if tileExistsDw {
				let tileDw = &gamestate.worldData.currentMap[&[newPos.x as i32, (newPos.y as i32)-1, newPos.z as i32]];
				tileDwColli = !check_collision(direction, tileDw.solid);
				if tileDwColli { newPos.y -= 1.0; }
			}
			if !(tileExistsUp && tileUpColli) && !(tileDwColli && tileDwColli) {
				gamestate.audio.play_sound("collision".to_string());
				return MovementResult::Blocked;
			}
		}
		let tile = &gamestate.worldData.currentMap[&[newPos.x as i32, newPos.y as i32, newPos.z as i32]];

		//* Check if Solid */
		if check_collision(direction, tile.solid) {
			gamestate.audio.play_sound("collision".to_string());
			return MovementResult::Blocked;
		}

		//* Check for entities */
		for (_, unit) in gamestate.worldData.unitMap.iter() {
			if newPos == unit.position && unit.exists(&gamestate.eventHandler) {
				gamestate.audio.play_sound("collision".to_string());
				return MovementResult::Blocked;
			}
		}

		//* Set animation */
		unitMove.animator.set_animation("walk_".to_string() + &direction.to_string());

		unitMove.posTarget = newPos;
		if unitId == "player" { gamestate.player.unit = unitMove; }
		else { gamestate.worldData.unitMap.insert(unitId.to_string(), unitMove); }

		return MovementResult::Worked;
	}

	/// Teleports a Unit to target location
	pub fn warp(gamestate: &mut data::Gamestate, unitId : &str, position : Vector3) {
		//* Get unit */
		let unitMove: &mut Unit;
		if unitId == "player" { unitMove = &mut gamestate.player.unit; }
		else { unitMove = gamestate.worldData.unitMap.get_mut(unitId).unwrap(); }

		//* Warp */
		unitMove.position = position;
		unitMove.posTarget = position;
	}

}

/// Check if the conditions are true
pub fn check_conditions(handler: &events::event_handler::EventHandler, conditions: &HashMap<String, events::conditionals::Condition>) -> bool {
	let mut result = true;
	
	for (str, cond) in conditions {
		if handler.eventVariables.contains_key(str) {
			if handler.eventVariables[str] == *cond { result = true; }
		} else if *cond == events::conditionals::Condition::Boolean(false) { result = true; }
	}
	return result;
}

/// Checks if there is a Unit in that position.
pub fn check_for_unit(unitMap: &HashMap<String, Unit>, position: Vector3) -> (bool, String) {
	for (str, unit) in unitMap {
		if unit.position == position { return (true, str.to_string()); }
	}
	return (false, "".to_string());
}

/// Calculates collision.
fn check_collision(direction: Direction, collisionInfo: [bool; 4]) -> bool {
	match direction {
		Direction::North => return collisionInfo[0],
		Direction::South => return collisionInfo[2],
		Direction::East  => return collisionInfo[3],
		Direction::West  => return collisionInfo[1],
	}
}
