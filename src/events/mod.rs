

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod conditionals;
pub mod event_handler;
pub mod textbox;
pub mod animation;

use crate::{overworld::{Direction, self}, data, utilities::math};


//= Enumerations

/// Event types
pub enum EventChain{

	//= Text controls
	/// Textbox display
	Text{ text: String },
	/// Textbox choice display
	Choice{
		text: String,
		choices: [textbox::Choice;4],
	},
	Input{
		text: String,
		variable: String,
	},

	//= Unit controls
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

	//= Camera controls
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
	/// Rotate camera to position
	RotateCamera{
		rotation: f32,
		wait: bool,
	},

	//= Audio controls
	/// Change Music
	Music{ music: String },
	/// Stop music
	PauseMusic,
	/// Play Sound
	Sound{ sound: String },

	//= Variables
	/// Set variable
	SetVariable{
		variable: String,
		value: conditionals::Condition,
	},
	/// Test variable
	TestVariable{
		variable: String,
		value: conditionals::Condition,

		event: String,
		position: i32,
	},

	//= Animation
	/// Play animation
	PlayAnimation{
		animation: String,
		order: Vec<i32>,
		ticks: i32,
		hold: bool,
	},

	//= DEBUG
	/// Default value
	Test{ text: String },
	/// Print all variables and their values
	DEBUGPrintVariables,
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
			}
		
		//= Text events
		EventChain::Text { text } => {
				if textbox::run(gamestate, text.to_string()) { gamestate.worldData.eventHandler.currentChain += 1; }
			}
		EventChain::Choice { text, choices } => {
				if !gamestate.worldData.eventHandler.textbox.hasChoice {
					gamestate.worldData.eventHandler.textbox.hasChoice = true;
					for i in choices {
						let copy = textbox::Choice{text: i.text.to_string(), event: i.event.to_string(), position: i.position};
						gamestate.worldData.eventHandler.textbox.choiceList.push(copy);
					}
				}
				if textbox::run(gamestate, text.to_string()) { gamestate.worldData.eventHandler.currentChain += 1; }
			}
		EventChain::Input { text, variable } => {
				let variableStr = variable.to_string();
				let inputStr = conditionals::Condition::String(gamestate.worldData.eventHandler.textbox.input.to_string());
				
				if !gamestate.worldData.eventHandler.textbox.isInput {
					gamestate.worldData.eventHandler.textbox.isInput = true;
				}
				if textbox::run(gamestate, text.to_string()) {
					gamestate.worldData.eventHandler.currentChain += 1;
					gamestate.worldData.eventHandler.eventVariables.insert(variableStr, inputStr);
				}
			}

		//= Movement events
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
			}
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
			}
		EventChain::Turn { entityID, direction } => {
				let unit: &mut overworld::Unit;
				if entityID == "player" { unit = &mut gamestate.player.unit; }
				else if !gamestate.worldData.unitMap.contains_key(entityID) { gamestate.worldData.eventHandler.currentChain += 1; return false; }
				else { unit = gamestate.worldData.unitMap.get_mut(entityID).unwrap(); }

				unit.direction = *direction;
				gamestate.worldData.eventHandler.currentChain += 1;
			}

		//= Wait
		EventChain::Wait { time } => {
				gamestate.worldData.eventHandler.internal += 1;
				if gamestate.worldData.eventHandler.internal >= *time {
					gamestate.worldData.eventHandler.internal = 0;
					gamestate.worldData.eventHandler.currentChain += 1;
				}
			}

		//= Camera events
		EventChain::ResetCamera => {
				gamestate.camera.onPlayer = true;
				gamestate.worldData.eventHandler.currentChain += 1;
			}
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
		
		//= Audio events
		EventChain::Music { music } => {
				gamestate.audio.play_music(music.to_string());
				gamestate.worldData.eventHandler.currentChain += 1;
			}
		EventChain::PauseMusic => {
				gamestate.audio.pause_music();
				gamestate.worldData.eventHandler.currentChain += 1;
			}
		EventChain::Sound { sound } => {
				gamestate.audio.play_sound(sound.to_string());
				gamestate.worldData.eventHandler.currentChain += 1;
			}

		//= Variable events
		EventChain::SetVariable { variable, value } => {
			gamestate.worldData.eventHandler.eventVariables.insert(variable.to_string(), value.clone());
			gamestate.worldData.eventHandler.currentChain += 1;
		}
		EventChain::TestVariable { variable, value, event, position } => {
			if gamestate.worldData.eventHandler.eventVariables.get(variable) == Some(value) {
				if event != "" { gamestate.worldData.eventHandler.currentEvent = event.to_string(); }
				gamestate.worldData.eventHandler.currentChain = *position;
			} else {
				gamestate.worldData.eventHandler.currentChain += 1;
			}
		}

		//= Animation events
		EventChain::PlayAnimation { animation, order, ticks, hold } => {
			let animation = animation::Animation{
				currentAnimation: animation.to_string(),
				order: order.clone(),
				ticksPerFrame: *ticks,
				hold: *hold,
				frame: 0,
				ticks: 0,
			};
			if animation::run(gamestate, animation) { gamestate.worldData.eventHandler.currentChain += 1; }
		}

		//= Debug events
		EventChain::DEBUGPrintVariables => {
				print!("Playername: {}\n",gamestate.worldData.eventHandler.playerName);
				print!("Playerpronoun_Subject: {}\n",gamestate.worldData.eventHandler.playerPronouns[0]);
				print!("Playerpronoun_Object: {}\n",gamestate.worldData.eventHandler.playerPronouns[1]);
				print!("Playerpronoun_Possesive: {}\n",gamestate.worldData.eventHandler.playerPronouns[2]);
				print!("Rivalname: {}\n",gamestate.worldData.eventHandler.rivalName);
				for (variable, value) in gamestate.worldData.eventHandler.eventVariables.iter_mut() {
					print!("{}: {}\n",variable, value.to_string());
				}
				gamestate.worldData.eventHandler.currentChain += 1;
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