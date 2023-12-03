

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod conditionals;
pub mod event_handler;
pub mod textbox;
pub mod animation;
pub mod parser;

use crate::{overworld::{Direction, self}, data, monsters, battle, raylib::vectors::Vector3};


//= Enumerations

/// Event types
#[derive(Clone)]
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
		wait: bool,
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

	//= Monsters
	/// Give monster to player
	GiveMonster{ monster: monsters::Monster },
	/// Give experience to player's monster
	GiveExperience{
		monsterPosition: usize,
		amount: i32,
	},
	/// Show level up stat changes for monster
	//ShowStats{},

	//= Battle
	StartBattle{ battle: battle::BattleType },
	EndBattle,

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
	PlayEmote{
		emote: String,
		unit: String,
		wait: bool,
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

impl Event {
	pub fn to_string(&self) -> String {
		let mut str = "".to_string();

		let mut count = 0;
		for i in self.chain.clone() {
			str += &format!("{}: ", count);
			match i {
				EventChain::Test { text } => {
					str += &format!("TEST-{}\n", text);
				}
				EventChain::Text { text } => {
					str += &format!("TEXT-{}\n", text);
				}
				EventChain::Choice { text, choices } => {
					str += &format!(
						"CHOICE-{}-[{}:{}-{},{}:{}-{},{}:{}-{},{}:{}-{}]\n",
						text,
						choices[0].text, choices[0].event, choices[0].position,
						choices[1].text, choices[1].event, choices[1].position,
						choices[2].text, choices[2].event, choices[2].position,
						choices[3].text, choices[3].event, choices[3].position,
					);
				}
				EventChain::Input { text, variable } => {
					str += &format!("INPUT-{}:{}\n", text, variable);
				}
				EventChain::Warp { entityID, position, direction, doMove } => {
					str += &format!(
						"WARP-{}->[[{},{},{}],{},{}]\n",
						entityID,
						position[0], position[1], position[2],
						direction,
						doMove,
					);
				}
				EventChain::Move { entityID, direction, times, wait } => {
					str += &format!(
						"MOVE-{}->[{},{}]-{}\n",
						entityID,
						direction,
						times,
						wait
					);
				}
				EventChain::Turn { entityID, direction } => {
					str += &format!("TURN-{}->{}\n", entityID, direction);
				}
				EventChain::Wait { time } => {
					str += &format!("WAIT-{}\n", time);
				}
				EventChain::GiveMonster { monster } => {
					str += &format!("GIVE_MONSTER-{}\n",monster);
				}
				EventChain::GiveExperience { monsterPosition, amount } => {
					str += &format!("GIVE_EXPERIENCE-[{}->{}]\n",amount,monsterPosition);
				}
				EventChain::StartBattle { .. } => {
					// TODO
					str += &format!("STARTBATTLE-\n");
				}
				EventChain::EndBattle => {
					str += "ENDBATTLE\n";
				}
				EventChain::ResetCamera => {
					str += &format!("CAMERA_RESET\n");
				}
				EventChain::SetCamera { position } => {
					str += &format!("CAMERA_SET->[{},{},{}]\n", position[0], position[1], position[2]);
				}
				EventChain::MoveCamera { position, wait } => {
					str += &format!("CAMERA_MOVE->[{},{},{}]:{}\n", position[0], position[1], position[2], wait);
				}
				EventChain::RotateCamera { rotation, wait } => {
					str += &format!("CAMERA_ROTATE->{}:{}\n", rotation, wait);
				}
				EventChain::Music { music } => {
					str += &format!("MUSIC-{}\n", music);
				}
				EventChain::PauseMusic => {
					str += &format!("MUSIC_PAUSE\n");
				}
				EventChain::Sound { sound } => {
					str += &format!("SOUND-{}\n", sound);
				}
				EventChain::SetVariable { variable, value } => {
					str += &format!("VARIABLE_SET-[{}:{}]\n", variable, value);
				}
				EventChain::TestVariable { variable, value, event, position } => {
					str += &format!("VARIABLE_TEST-[{}:{}]->[{}:{}]\n", variable, value, event, position);
				}
				EventChain::PlayAnimation { animation, order, ticks, hold } => {
					str += &format!("ANIMATION-{}:[{:?}]:{}:{}\n", animation, order, ticks, hold);
				}
				EventChain::PlayEmote { emote, unit, wait } => {
					str += &format!("EMOTE-{}->{}:{}\n", emote, unit, wait);
				}
				EventChain::DEBUGPrintVariables => {
					str += &format!("DEBUG_PRINTVARIABLES\n");
				}
			}
			count += 1;
		}
		return str;
	}
}

/// Parses the current event.
pub fn parse_event( gamestate : &mut data::Gamestate ) -> bool {

	//* If there isn't an event currently, leave */
	if gamestate.eventHandler.currentEvent == "".to_string() { return false; }

	//* If the current event doesn't exist, clear it */
	if !gamestate.worldData.eventList.contains_key(&gamestate.eventHandler.currentEvent) { clear_event(gamestate); return false; }
	
	let event = &gamestate.worldData.eventList[&gamestate.eventHandler.currentEvent];
	//* Check if event chain position is at the end of the event */
	if gamestate.eventHandler.currentChain >= event.chain.len() as i32 { clear_event(gamestate); return false; }

	//* Parse the the current chain event */
	let chPos = gamestate.eventHandler.currentChain as usize;
	let chain = &gamestate.worldData.eventList[&gamestate.eventHandler.currentEvent].chain[chPos].clone();
	match chain {
		EventChain::Test{ text } => {
				print!("TEST: {}\n",text);
				gamestate.eventHandler.currentChain += 1;
			}
		
		//= Text events
		EventChain::Text { text } => {
				if textbox::run(gamestate, text.to_string()) { gamestate.eventHandler.currentChain += 1; }
			}
		EventChain::Choice { text, choices } => {
				if !gamestate.eventHandler.textbox.hasChoice {
					gamestate.eventHandler.textbox.hasChoice = true;
					for i in choices {
						let copy = textbox::Choice{text: i.text.to_string(), event: i.event.to_string(), position: i.position};
						gamestate.eventHandler.textbox.choiceList.push(copy);
					}
				}
				if textbox::run(gamestate, text.to_string()) { gamestate.eventHandler.currentChain += 1; }
			}
		EventChain::Input { text, variable } => {
				let variableStr = variable.to_string();
				let inputStr = conditionals::Condition::String(gamestate.eventHandler.textbox.input.to_string());
				
				if !gamestate.eventHandler.textbox.isInput {
					gamestate.eventHandler.textbox.isInput = true;
				}
				if textbox::run(gamestate, text.to_string()) {
					gamestate.eventHandler.currentChain += 1;
					gamestate.eventHandler.eventVariables.insert(variableStr, inputStr);
				}
			}

		//= Movement events
		EventChain::Warp { entityID, position, direction, doMove } => {
				//* Move unit */
				overworld::Unit::warp(gamestate, entityID, Vector3::from(*position));

				//* If doMove is true, move */
				if *doMove { overworld::Unit::walk(gamestate, entityID, *direction); }
				
				gamestate.eventHandler.currentChain += 1;
			}
		EventChain::Move { entityID, direction, times, .. } => {
				//* Check if target is player */
				let result = overworld::Unit::walk(gamestate, entityID, *direction);
				
				//* Only increment movement if not moving */
				// TODO Make the wait bool work
				if result != overworld::MovementResult::Moving {
					gamestate.eventHandler.internal += 1;
					if gamestate.eventHandler.internal >= *times {
						gamestate.eventHandler.internal = 0;
						gamestate.eventHandler.currentChain += 1;
					}
				}
			}
		EventChain::Turn { entityID, direction } => {
				let unit: &mut overworld::Unit;
				if entityID == "player" { unit = &mut gamestate.player.unit; }
				else if !gamestate.worldData.unitMap.contains_key(entityID) { gamestate.eventHandler.currentChain += 1; return false; }
				else { unit = gamestate.worldData.unitMap.get_mut(entityID).unwrap(); }

				unit.direction = *direction;
				gamestate.eventHandler.currentChain += 1;
			}

		//= Wait
		EventChain::Wait { time } => {
				gamestate.eventHandler.internal += 1;
				if gamestate.eventHandler.internal >= *time {
					gamestate.eventHandler.internal = 0;
					gamestate.eventHandler.currentChain += 1;
				}
			}

		//= Monster events
		EventChain::GiveMonster { monster } => {
				gamestate.player.monsters.add_member(monster.clone());
				gamestate.eventHandler.currentChain += 1;
			}
		EventChain::GiveExperience { monsterPosition, amount } => {
				gamestate.audio.play_sound("experience".to_string());
				gamestate.eventHandler.internal += 1;
				if gamestate.eventHandler.internal < *amount {
					gamestate.player.monsters.0[*monsterPosition].as_mut().unwrap().give_experience(1);
				} else {
					gamestate.eventHandler.currentChain += 1;
					gamestate.eventHandler.internal = 0;
				}
			}

		//= Battle events
		EventChain::StartBattle { battle } => {
			gamestate.player.canMove = false;
			gamestate.battleData.start_trainer_battle(battle.clone());
			gamestate.eventHandler.currentChain += 1;
		}
		EventChain::EndBattle => {
			gamestate.player.canMove = true;
			gamestate.battleData.clear();
			gamestate.eventHandler.currentChain += 1;
		}

		//= Camera events
		EventChain::ResetCamera => {
				gamestate.camera.onPlayer = true;
				gamestate.eventHandler.currentChain += 1;
			}
		EventChain::SetCamera { position } => {
				gamestate.camera.onPlayer = false;
				gamestate.camera.position = Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				gamestate.camera.posTarget = Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				gamestate.eventHandler.currentChain += 1;
			}
		EventChain::MoveCamera { position, wait } => {
				gamestate.camera.onPlayer = false;
				gamestate.camera.posTarget = Vector3{x: position[0] as f32, y: position[1] as f32, z: position[2] as f32};
				if !*wait || gamestate.camera.posTarget == gamestate.camera.position { gamestate.eventHandler.currentChain += 1; }
			}
		EventChain::RotateCamera { rotation, wait } => {
				gamestate.camera.onPlayer = false;
				gamestate.camera.rotTarget = *rotation;
				if !*wait || gamestate.camera.rotTarget == gamestate.camera.rotation { gamestate.eventHandler.currentChain += 1; }
			}
		
		//= Audio events
		EventChain::Music { music } => {
				gamestate.audio.play_music(music.to_string());
				gamestate.eventHandler.currentChain += 1;
			}
		EventChain::PauseMusic => {
				gamestate.audio.pause_music();
				gamestate.eventHandler.currentChain += 1;
			}
		EventChain::Sound { sound } => {
				gamestate.audio.play_sound(sound.to_string());
				gamestate.eventHandler.currentChain += 1;
			}

		//= Variable events
		EventChain::SetVariable { variable, value } => {
			gamestate.eventHandler.eventVariables.insert(variable.to_string(), value.clone());
			gamestate.eventHandler.currentChain += 1;
		}
		EventChain::TestVariable { variable, value, event, position } => {
			if gamestate.eventHandler.eventVariables.get(variable) == Some(value) {
				if event != "" { gamestate.eventHandler.currentEvent = event.to_string(); }
				gamestate.eventHandler.currentChain = *position;
			} else {
				gamestate.eventHandler.currentChain += 1;
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
			if animation::run(gamestate, animation) { gamestate.eventHandler.currentChain += 1; }
		}
		EventChain::PlayEmote { emote, unit, wait } => {
			if gamestate.eventHandler.internal == 0 {
				gamestate.eventHandler.emotes.push(
					animation::EmoteAnimation {
						emote: emote.to_string(),
						unitID: unit.to_string(),
						duration: 100,
						ticks: 0,
					}
				);
			}
			if *wait {
				gamestate.eventHandler.internal += 1;
				if gamestate.eventHandler.internal >= 200 {
					gamestate.eventHandler.currentChain += 1;
					gamestate.eventHandler.internal = 0;
				}
			} else {
				gamestate.eventHandler.currentChain += 1;
				gamestate.eventHandler.internal = 0;
			}
			
		}

		//= Debug events
		EventChain::DEBUGPrintVariables => {
				print!("Playername: {}\n",gamestate.eventHandler.playerName);
				print!("Playerpronoun_Subject: {}\n",gamestate.eventHandler.playerPronouns[0]);
				print!("Playerpronoun_Object: {}\n",gamestate.eventHandler.playerPronouns[1]);
				print!("Playerpronoun_Possesive: {}\n",gamestate.eventHandler.playerPronouns[2]);
				print!("Rivalname: {}\n",gamestate.eventHandler.rivalName);
				for (variable, value) in gamestate.eventHandler.eventVariables.iter_mut() {
					print!("{}: {}\n",variable, value.to_string());
				}
				gamestate.eventHandler.currentChain += 1;
			}
		
	}
	return true;
}

/// Clears the current event and sets all event data to 0.
pub fn clear_event( gamestate : &mut data::Gamestate ) {
	gamestate.eventHandler.currentChain = 0;
	gamestate.eventHandler.currentEvent = "".to_string();
	gamestate.eventHandler.internal = 0;
	gamestate.player.canMove = true;
}