

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]



use std::str::FromStr;

//= Imports
use serde_json::Value;
use crate::monsters;
use crate::overworld;
use crate::utilities::debug;

use super::EventChain;
use super::conditionals;
use super::textbox;


//= Enumerations


//= Procedures

/// Parses JSON object into an EventChain
pub fn parse_value( value: &Value ) -> EventChain {
	match value.as_array().unwrap()[0].as_str().unwrap() {
		//= Text events
		"text" => {
			return EventChain::Text { text: value.as_array().unwrap()[1].as_str().unwrap().to_string() }
		}
		"choice" => {
			let mut choices = [
				textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
				textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
				textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
				textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
			];

			let mut val = 0;
			for i in value.as_array().unwrap()[2].as_array().unwrap() {
				choices[val] = textbox::Choice{
					text: i.as_array().unwrap()[0].as_str().unwrap().to_string(),
					event: i.as_array().unwrap()[1].as_str().unwrap().to_string(),
					position: i.as_array().unwrap()[2].is_i64() as i32,
				};
				if i.as_array().unwrap().get(2) != None { choices[val].position = i.as_array().unwrap()[2].as_i64().unwrap() as i32; }
				val += 1;
			}

			return EventChain::Choice {
				text: value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				choices,
			};
		}
		"input" => {
			return EventChain::Input {
				text:		value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				variable:	value.as_array().unwrap()[2].as_str().unwrap().to_string(),
			}
		}

		//= Movement events
		"warp" => {
			return EventChain::Warp{
				entityID:	value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				position:	[
					value.as_array().unwrap()[2].as_array().unwrap()[0].as_i64().unwrap() as i32,
					value.as_array().unwrap()[2].as_array().unwrap()[1].as_i64().unwrap() as i32,
					value.as_array().unwrap()[2].as_array().unwrap()[2].as_i64().unwrap() as i32,
				],
				direction:	overworld::Direction::from_str(value.as_array().unwrap()[4].as_str().unwrap()).unwrap(),
				doMove:		value.as_array().unwrap()[3].as_bool().unwrap(),
			}
		}
		"turn" => {
			return EventChain::Turn {
				entityID:	value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				direction:	overworld::Direction::from_str(value.as_array().unwrap()[2].as_str().unwrap()).unwrap(),
			}
		}
		"move" => {
			return EventChain::Move {
				entityID:	value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				direction:	overworld::Direction::from_str(value.as_array().unwrap()[2].as_str().unwrap()).unwrap(),
				times:		value.as_array().unwrap()[3].as_i64().unwrap() as i32,
			}
		}
		
		//= Wait
		"wait" => {
			return EventChain::Wait { time: value.as_array().unwrap()[1].as_i64().unwrap() as i32 }
		}

		//= Monster events
		"give_monster" => {
			match value.as_array().unwrap()[1].as_array().unwrap()[0].as_i64().unwrap() {
				_ => {
					return EventChain::GiveMonster {
						monster: monsters::Monster::new(
							monsters::MonsterSpecies::from_str(value.as_array().unwrap()[1].as_array().unwrap()[1].as_str().unwrap()).unwrap(),
							value.as_array().unwrap()[1].as_array().unwrap()[2].as_i64().unwrap() as i32,
						),
					}
				}
			}
		}
		"give_experience" => {
			return EventChain::GiveExperience {
				monsterPosition:	value.as_array().unwrap()[1].as_i64().unwrap() as usize,
				amount: 			value.as_array().unwrap()[2].as_i64().unwrap() as i32,
			}
		}

		//= Camera events
		"reset_camera" => {
			return EventChain::ResetCamera;
		}
		"set_camera" => {
			return EventChain::SetCamera {
				position: [
					value.as_array().unwrap()[1].as_array().unwrap()[0].as_i64().unwrap() as i32,
					value.as_array().unwrap()[1].as_array().unwrap()[1].as_i64().unwrap() as i32,
					value.as_array().unwrap()[1].as_array().unwrap()[2].as_i64().unwrap() as i32,
				],
			}
		}
		"move_camera" => {
			return EventChain::MoveCamera {
				position: [
					value.as_array().unwrap()[1].as_array().unwrap()[0].as_i64().unwrap() as i32,
					value.as_array().unwrap()[1].as_array().unwrap()[1].as_i64().unwrap() as i32,
					value.as_array().unwrap()[1].as_array().unwrap()[2].as_i64().unwrap() as i32,
				],
				wait: value.as_array().unwrap()[2].as_bool().unwrap(),
			}
		}
		"rotate_camera" => {
			return EventChain::RotateCamera {
				rotation:	value.as_array().unwrap()[1].as_i64().unwrap() as f32,
				wait:		value.as_array().unwrap()[2].as_bool().unwrap(),
			}
		}

		//= Audio events
		"music" => {
			return EventChain::Music { music: value.as_array().unwrap()[1].as_str().unwrap().to_string() };
		}
		"pause_music" => {
			return EventChain::PauseMusic;
		}
		"sound" => {
			return EventChain::Sound { sound: value.as_array().unwrap()[1].as_str().unwrap().to_string() };
		}

		//= Variable events
		"set_variable" => {
			let mut condition: conditionals::Condition = conditionals::Condition::Boolean(false);
			let mut real = true;
			match &value.as_array().unwrap()[2] {
				serde_json::value::Value::String {..} => {
					condition = conditionals::Condition::String(value.as_array().unwrap()[2].as_str().unwrap().to_string())
				}
				serde_json::value::Value::Bool {..} => {
					condition = conditionals::Condition::Boolean(value.as_array().unwrap()[2].as_bool().unwrap())
				}
				serde_json::value::Value::Number {..} => {
					condition = conditionals::Condition::Integer(value.as_array().unwrap()[2].as_i64().unwrap() as i32)
				}
				_ => {
					debug::log("[ERROR] - Attempted to put illegal value into a variable.");
					real = false;
				}
			}
			if real {
				return EventChain::SetVariable {
					variable:	value.as_array().unwrap()[1].as_str().unwrap().to_string(),
					value: 		condition,
				}
			} else {
				return EventChain::Test { text: value.as_array().unwrap()[0].as_str().unwrap().to_string() };
			}
		}
		"test_variable" => {
			let mut condition: conditionals::Condition = conditionals::Condition::Boolean(false);
			let mut real = true;
			match value.as_array().unwrap()[1].as_array().unwrap()[1] {
				serde_json::value::Value::String {..} => {
					condition = conditionals::Condition::String(value.as_array().unwrap()[1].as_array().unwrap()[1].as_str().unwrap().to_string())
				}
				serde_json::value::Value::Bool {..} => {
					condition = conditionals::Condition::Boolean(value.as_array().unwrap()[1].as_array().unwrap()[1].as_bool().unwrap())
				}
				serde_json::value::Value::Number {..} => {
					condition = conditionals::Condition::Integer(value.as_array().unwrap()[1].as_array().unwrap()[1].as_i64().unwrap() as i32)
				}
				_ => {
					debug::log("[ERROR] - Attempted to put illegal value into a variable.");
					real = false;
				}
			}

			if real {
				return EventChain::TestVariable {
					variable: value.as_array().unwrap()[1].as_array().unwrap()[0].as_str().unwrap().to_string(),
					value: condition,
					event: value.as_array().unwrap()[2].as_array().unwrap()[0].as_str().unwrap().to_string(),
					position: value.as_array().unwrap()[2].as_array().unwrap()[1].as_i64().unwrap() as i32,
				}
			} else {
				return EventChain::Test { text: value.as_array().unwrap()[0].as_str().unwrap().to_string() };
			}
		}

		//= Animation events
		"animation" => {
			let mut animOrder: Vec<i32> = Vec::new();
			for i in value.as_array().unwrap()[4].as_array().unwrap() {
				animOrder.push(i.as_i64().unwrap() as i32);
			}

			return EventChain::PlayAnimation {
				animation:	value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				order:		animOrder,
				ticks:		value.as_array().unwrap()[3].as_i64().unwrap() as i32,
				hold:		value.as_array().unwrap()[5].as_bool().unwrap(),
			}
		}
		"emote" => {
			return EventChain::PlayEmote {
				emote:	value.as_array().unwrap()[1].as_str().unwrap().to_string(),
				unit:	value.as_array().unwrap()[2].as_str().unwrap().to_string(),
				wait:	value.as_array().unwrap()[3].as_bool().unwrap(),
			}
		}

		//= DEBUG
		"DEBUG_print_variables" => {
			return EventChain::DEBUGPrintVariables;
		}
		_ => {
			return EventChain::Test { text: value.as_array().unwrap()[0].as_str().unwrap().to_string() }
		}
	}
}

