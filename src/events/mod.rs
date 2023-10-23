

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod conditionals;
pub mod event_handler;
pub mod textbox;
use crate::{overworld::{Direction, self}, data, utilities::math};


//= Enumerations

/// Event types
pub enum EventChain{
	/// Test / Debug
	Test{ text: String },
	
	/// Textbox display
	Text{ text: String },
	/// Textbox choice display
	Choice{
		text: String,
		choices: [textbox::Choice;4],
	},

	/// Teleport unit
	Warp{
		entityID:	String,
		position:	[i32;3],
		direction:	Direction,
		doMove:		bool,
	
	},
	/// Move unit
	Move{
		entityID: String,
		direction: Direction,
		times: i32,
	},
	/// Turn unit
	Turn{
		entityID: String,
		direction: Direction,
	},

	/// Wait
	Wait{
		time: i32,
	},

	/// Reset camera to player
	ResetCamera,
	/// Move camera to position
	SetCamera{
		position: [i32;3],
	},
	MoveCamera{
		position: [i32;3],
		wait: bool,
	},
	RotateCamera{
		rotation: f32,
		wait: bool,
	},
	// TODO Rotate Camera / etc.
}


//= Structures

/// Basic structure for all events
pub struct Event{
	pub chain : Vec<EventChain>,
}


//= Procedures

/// Parses the current event.
pub fn parse_event( gamestate : &mut data::Gamestate ) -> bool {

	//* If there isn't an event currently, leave */
	if gamestate.worldData.eventHandler.currentEvent == "".to_string() { return false; }

	//* If the current event doesn't exist, clear it */
	if !gamestate.worldData.eventList.contains_key(&gamestate.worldData.eventHandler.currentEvent) { clear_event(gamestate); return false; }
	
	let event = &gamestate.worldData.eventList[&gamestate.worldData.eventHandler.currentEvent];
	//* Check if event chain position is at the end of the event */
	if gamestate.worldData.eventHandler.currentChain >= event.chain.len() as i32 { clear_event(gamestate); return false; }

	//* Parse the the current chain event */
	let chPos = gamestate.worldData.eventHandler.currentChain as usize;
	let chain = &event.chain[chPos];
	match chain {
		EventChain::Test{ text } => {
				print!("TEST: {}\n",text);
				gamestate.worldData.eventHandler.currentChain += 1;
			},
		EventChain::Text { text } => {
				if textbox::run(gamestate, text.to_string()) { gamestate.worldData.eventHandler.currentChain += 1; }
			},
		EventChain::Choice { text, choices } => {
				if !gamestate.worldData.eventHandler.textbox.hasChoice {
					gamestate.worldData.eventHandler.textbox.hasChoice = true;
					for i in choices {
						let copy = textbox::Choice{text: i.text.to_string(), event: i.event.to_string(), position: i.position};
						gamestate.worldData.eventHandler.textbox.choiceList.push(copy);
					}
				}
				if textbox::run(gamestate, text.to_string()) { gamestate.worldData.eventHandler.currentChain += 1; }
			},
		EventChain::Warp { entityID, position, direction, doMove } => {
				let unit: &mut overworld::Unit;
				let unitMap = gamestate.worldData.unitMap.clone();
				//* Check if target is player */
				if entityID == "player" { unit = &mut gamestate.player.unit; }
				else if !gamestate.worldData.unitMap.contains_key(entityID) { gamestate.worldData.eventHandler.currentChain += 1; return false; }
				else { unit = gamestate.worldData.unitMap.get_mut(entityID).unwrap(); }

				//* Move unit */
				unit.position  = raylib_ffi::Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				unit.posTarget = raylib_ffi::Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				unit.direction = *direction;

				//* If doMove is true, move */
				if *doMove { overworld::move_unit(&gamestate.worldData.currentMap, &unitMap, &gamestate.worldData.eventHandler, unit, *direction); }
				else { unit.posTarget = raylib_ffi::Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32}; }
				
				gamestate.worldData.eventHandler.currentChain += 1;
			},
		EventChain::Move { entityID, direction, times } => {
				let unit: &mut overworld::Unit;
				let unitMap = gamestate.worldData.unitMap.clone();
				//* Check if target is player */
				if entityID == "player" { unit = &mut gamestate.player.unit; }
				else if !gamestate.worldData.unitMap.contains_key(entityID) { gamestate.worldData.eventHandler.currentChain += 1; return false; }
				else { unit = gamestate.worldData.unitMap.get_mut(entityID).unwrap(); }

				overworld::move_unit(&gamestate.worldData.currentMap, &unitMap, &gamestate.worldData.eventHandler, unit, *direction);
				
				gamestate.worldData.eventHandler.internal += 1;
				if gamestate.worldData.eventHandler.internal >= *times {
					gamestate.worldData.eventHandler.internal = 0;
					gamestate.worldData.eventHandler.currentChain += 1;
				}
			},
		EventChain::Turn { entityID, direction } => {
				let unit: &mut overworld::Unit;
				if entityID == "player" { unit = &mut gamestate.player.unit; }
				else if !gamestate.worldData.unitMap.contains_key(entityID) { gamestate.worldData.eventHandler.currentChain += 1; return false; }
				else { unit = gamestate.worldData.unitMap.get_mut(entityID).unwrap(); }

				unit.direction = *direction;
				gamestate.worldData.eventHandler.currentChain += 1;
			},

		EventChain::Wait { time } => {
				gamestate.worldData.eventHandler.internal += 1;
				if gamestate.worldData.eventHandler.internal >= *time {
					gamestate.worldData.eventHandler.internal = 0;
					gamestate.worldData.eventHandler.currentChain += 1;
				}
			},

		EventChain::ResetCamera => {
				gamestate.camera.onPlayer = true;
				gamestate.worldData.eventHandler.currentChain += 1;
			},
		EventChain::SetCamera { position } => {
				gamestate.camera.onPlayer = false;
				gamestate.camera.position = raylib_ffi::Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				gamestate.camera.posTarget = raylib_ffi::Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				gamestate.worldData.eventHandler.currentChain += 1;
			}
		EventChain::MoveCamera { position, wait } => {
				gamestate.camera.onPlayer = false;
				gamestate.camera.posTarget = raylib_ffi::Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				if !*wait || math::equal_v3(gamestate.camera.posTarget, gamestate.camera.position) { gamestate.worldData.eventHandler.currentChain += 1; }
			}
		EventChain::RotateCamera { rotation, wait } => {
				gamestate.camera.onPlayer = false;
				gamestate.camera.rotTarget = *rotation;
				if !*wait || gamestate.camera.rotTarget == gamestate.camera.rotation { gamestate.worldData.eventHandler.currentChain += 1; }
			}
			//_ => return,
	}
	return true;
}

/// Clears the current event and sets all event data to 0.
pub fn clear_event( gamestate : &mut data::Gamestate ) {
	gamestate.worldData.eventHandler.currentChain = 0;
	gamestate.worldData.eventHandler.currentEvent = "".to_string();
	gamestate.worldData.eventHandler.internal = 0;
	gamestate.player.canMove = true;
}