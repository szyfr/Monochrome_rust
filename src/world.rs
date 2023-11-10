

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use crate::{utilities::{debug, math}, data::Gamestate, overworld, raylib, events::{self, conditionals::{Condition, self}, textbox}, monsters};


//= Constants
/// Render width (x)
const WIDTH  : f32 = 20.0;
/// Render height (y)
const HEIGHT : f32 = 10.0;
/// Render depth (z)
const DEPTH  : f32 = 18.0;


//= Structures

/// World data storage
pub struct World{
	pub currentMap	: HashMap<[i32;3], Tile>,

	pub unitMap		: HashMap<String, overworld::Unit>,

	pub triggerMap	: HashMap<[i32;3], String>,
	pub eventList	: HashMap<String, events::Event>,

	pub eventHandler: events::event_handler::EventHandler,
}

/// Tile storage structure
pub struct Tile{
	pub model : String,

	pub solid : [bool;4],
	pub water : bool,
	pub trnsp : bool,
}


//= Procedures

impl World {
	/// Load all
	pub fn load_all( &mut self, mapName : &str ) {
		self.load_world(mapName);
		self.load_entities(mapName);
		self.load_events(mapName);
		self.load_triggers(mapName);
	}

	/// Load tile data from input file to Hashmap indexed by their position.
	pub fn load_world( &mut self, mapName : &str ) {
		//* Attempt to load map file */
		let fileResult_map = read_to_string("data/world/".to_string() + mapName + "/map.json" );
		if fileResult_map.is_err() {
			debug::log("[ERROR] - Failed to load map file.\n");
			return;
		}

		//* Convert to JSON and read */
		let jsonFile_map: serde_json::Value = serde_json::from_str(&fileResult_map.unwrap()).unwrap();
		for i in jsonFile_map["tiles"].as_array().unwrap() {
			let tile = Tile {
				model: i["tile"].as_str().unwrap().to_string(),
				solid: solid_tag_to_bool(i["tags"].as_array().unwrap()[0].as_array().unwrap()),
				water: i["tags"].as_array().unwrap()[1].as_bool().unwrap(),
				trnsp: i["tags"].as_array().unwrap()[2].as_bool().unwrap(),
			};
			let position = [
				i["position"].as_array().unwrap()[0].as_f64().unwrap() as i32,
				i["position"].as_array().unwrap()[1].as_f64().unwrap() as i32,
				i["position"].as_array().unwrap()[2].as_f64().unwrap() as i32,
			];
			self.currentMap.insert(position, tile);
		}
	}

	/// Loads entity data from input file to Hashmap indexed by their ID.
	pub fn load_entities( &mut self, mapName : &str ) {
		//* Attempt to load entities file */
		let fileResult_ent = read_to_string("data/world/".to_string() + mapName + "/entities.json" );
		if fileResult_ent.is_err() {
			debug::log("[ERROR] - Failed to load map file.\n");
			return;
		}

		//* Convert to JSON and read */
		let jsonFile_ent: serde_json::Value = serde_json::from_str(&fileResult_ent.unwrap()).unwrap();
		let arr = jsonFile_ent["entities"].as_array().unwrap();
		for i in 0..arr.len() {
			let mut unit = overworld::create_unit(arr[i]["sprite"].as_str().unwrap());

			//* Set entity direction and position */
			unit.direction = overworld::Direction::from_str(arr[i]["direction"].as_str().unwrap()).unwrap();
			unit.position = raylib_ffi::Vector3{
				x: arr[i]["location"].as_array().unwrap()[0].as_i64().unwrap() as f32,
				y: arr[i]["location"].as_array().unwrap()[1].as_i64().unwrap() as f32,
				z: arr[i]["location"].as_array().unwrap()[2].as_i64().unwrap() as f32,
			};
			unit.posTarget = unit.position;

			//* Set entity events */
			for o in arr[i]["events"].as_array().unwrap() {
				let mut conds: HashMap<String, events::conditionals::Condition> = HashMap::new();
				for e in o["conditions"].as_array().unwrap() {
					let str = e.as_array().unwrap()[0].as_str().unwrap().to_string();
					let arr2: &serde_json::Value = &e.as_array().unwrap()[1];
					match arr2 {
						serde_json::Value::Bool(_)		=> _ = conds.insert(str, Condition::Boolean( arr2.as_bool().unwrap()) ),
						serde_json::Value::Number(_)	=> _ = conds.insert(str, Condition::Integer( arr2.as_i64().unwrap() as i32) ),
						serde_json::Value::String(_)	=> _ = conds.insert(str, Condition::String(  arr2.as_str().unwrap().to_string()) ),
						_ => continue,
					}
				}
				unit.events.insert(o["id"].as_str().unwrap().to_string(), conds);
			}

			//* Set entity appearance conditions */
			for o in arr[i]["conditions"].as_array().unwrap() {
				let str = o.as_array().unwrap()[0].as_str().unwrap().to_string();
				let arr2: &serde_json::Value = &o.as_array().unwrap()[1];
				match arr2 {
					serde_json::Value::Bool(_)		=> _ = unit.conditions.insert(str, Condition::Boolean( arr2.as_bool().unwrap()) ),
					serde_json::Value::Number(_)	=> _ = unit.conditions.insert(str, Condition::Integer( arr2.as_i64().unwrap() as i32) ),
					serde_json::Value::String(_)	=> _ = unit.conditions.insert(str, Condition::String(  arr2.as_str().unwrap().to_string()) ),
					_ => continue,
				}
			}

			self.unitMap.insert(arr[i]["id"].as_str().unwrap().to_string(), unit);
		}
	}

	/// Loads event data from input file to Hashmap indexed by their ID.
	pub fn load_events( &mut self, mapName : &str ) {
		//* Attempt to load entities file */
		let fileResult_evt = read_to_string("data/world/".to_string() + mapName + "/events.json" );
		if fileResult_evt.is_err() {
			debug::log("[ERROR] - Failed to load map file.\n");
			return;
		}

		//* Convert to JSON and read */
		let jsonFile_evt: serde_json::Value = serde_json::from_str(&fileResult_evt.unwrap()).unwrap();
		for i in jsonFile_evt["events"].as_array().unwrap() {
			let mut event: events::Event = events::Event{ chain : Vec::new() };
			for o in i["chain"].as_array().unwrap() {
				let chain: events::EventChain;
				match o.as_array().unwrap()[0].as_str().unwrap() {
					//= Text events
					"turn" => {
						chain = events::EventChain::Turn { entityID: o.as_array().unwrap()[1].as_str().unwrap().to_string(), direction: overworld::Direction::from_str(o.as_array().unwrap()[2].as_str().unwrap()).unwrap() }
					}
					"text" => {
						chain = events::EventChain::Text { text: o.as_array().unwrap()[1].as_str().unwrap().to_string() }
					}
					"choice" => {
						let mut choices = [
							textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
							textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
							textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
							textbox::Choice{text: "".to_string(), event: "".to_string(), position: 0},
						];

						let mut val = 0;
						for i in o.as_array().unwrap()[2].as_array().unwrap() {
							choices[val] = textbox::Choice{
								text: i.as_array().unwrap()[0].as_str().unwrap().to_string(),
								event: i.as_array().unwrap()[1].as_str().unwrap().to_string(),
								position: i.as_array().unwrap()[2].is_i64() as i32,
							};
							if i.as_array().unwrap().get(2) != None { choices[val].position = i.as_array().unwrap()[2].as_i64().unwrap() as i32; }
							val += 1;
						}

						chain = events::EventChain::Choice {
							text: o.as_array().unwrap()[1].as_str().unwrap().to_string(),
							choices,
						};
					}
					"input" => {
						chain = events::EventChain::Input {
							text: o.as_array().unwrap()[1].as_str().unwrap().to_string(),
							variable: o.as_array().unwrap()[2].as_str().unwrap().to_string(),
						}
					}

					//= Movement events
					"warp" => {
						chain = events::EventChain::Warp{
							entityID:	o.as_array().unwrap()[1].as_str().unwrap().to_string(),
							position:	[
								o.as_array().unwrap()[2].as_array().unwrap()[0].as_i64().unwrap() as i32,
								o.as_array().unwrap()[2].as_array().unwrap()[1].as_i64().unwrap() as i32,
								o.as_array().unwrap()[2].as_array().unwrap()[2].as_i64().unwrap() as i32,
								],
							direction:	overworld::Direction::from_str(o.as_array().unwrap()[4].as_str().unwrap()).unwrap(),
							doMove: o.as_array().unwrap()[3].as_bool().unwrap(),
						}
					}
					"move" => {
						chain = events::EventChain::Move {
							entityID:	o.as_array().unwrap()[1].as_str().unwrap().to_string(),
							direction:	overworld::Direction::from_str(o.as_array().unwrap()[2].as_str().unwrap()).unwrap(),
							times:		o.as_array().unwrap()[3].as_i64().unwrap() as i32,
						}
					}
					
					//= Wait
					"wait" => {
						chain = events::EventChain::Wait { time: o.as_array().unwrap()[1].as_i64().unwrap() as i32 }
					}

					//= Monster events
					"give_monster" => {
						match o.as_array().unwrap()[1].as_array().unwrap()[0].as_i64().unwrap() {
							_ => {
								chain = events::EventChain::GiveMonster {
									monster: monsters::new(
										monsters::MonsterSpecies::from_str(o.as_array().unwrap()[1].as_array().unwrap()[1].as_str().unwrap()).unwrap(),
										o.as_array().unwrap()[1].as_array().unwrap()[2].as_i64().unwrap() as i32,
									),
								}
							}
						}
					}
					"give_experience" => {
						chain = events::EventChain::GiveExperience {
							monsterPosition: o.as_array().unwrap()[1].as_i64().unwrap() as usize,
							amount: o.as_array().unwrap()[2].as_i64().unwrap() as i32,
						}
					}
					
					//= Camera events
					"reset_camera" => {
						chain = events::EventChain::ResetCamera
					}
					"set_camera" => {
						chain = events::EventChain::SetCamera {
							position: [
								o.as_array().unwrap()[1].as_array().unwrap()[0].as_i64().unwrap() as i32,
								o.as_array().unwrap()[1].as_array().unwrap()[1].as_i64().unwrap() as i32,
								o.as_array().unwrap()[1].as_array().unwrap()[2].as_i64().unwrap() as i32,
							],
						}
					}
					"move_camera" => {
						chain = events::EventChain::MoveCamera {
							position: [
								o.as_array().unwrap()[1].as_array().unwrap()[0].as_i64().unwrap() as i32,
								o.as_array().unwrap()[1].as_array().unwrap()[1].as_i64().unwrap() as i32,
								o.as_array().unwrap()[1].as_array().unwrap()[2].as_i64().unwrap() as i32,
							],
							wait: o.as_array().unwrap()[2].as_bool().unwrap(),
						}
					}
					"rotate_camera" => {
						chain = events::EventChain::RotateCamera {
							rotation: o.as_array().unwrap()[1].as_i64().unwrap() as f32,
							wait: o.as_array().unwrap()[2].as_bool().unwrap(),
						}
					}

					//= Audio events
					"music" => {
						chain = events::EventChain::Music { music: o.as_array().unwrap()[1].as_str().unwrap().to_string() };
					}
					"pause_music" => {
						chain = events::EventChain::PauseMusic;
					}
					"sound" => {
						chain = events::EventChain::Sound { sound: o.as_array().unwrap()[1].as_str().unwrap().to_string() };
					}

					//= Variable events
					"set_variable" => {
						let mut condition: conditionals::Condition = conditionals::Condition::Boolean(false);
						let mut real = true;
						match &o.as_array().unwrap()[2] {
							serde_json::value::Value::String {..} => {
								condition = conditionals::Condition::String(o.as_array().unwrap()[2].as_str().unwrap().to_string())
							}
							serde_json::value::Value::Bool {..} => {
								condition = conditionals::Condition::Boolean(o.as_array().unwrap()[2].as_bool().unwrap())
							}
							serde_json::value::Value::Number {..} => {
								condition = conditionals::Condition::Integer(o.as_array().unwrap()[2].as_i64().unwrap() as i32)
							}
							_ => {
								debug::log("[ERROR] - Attempted to put illegal value into a variable.");
								real = false;
							}
						}
						if real {
							chain = events::EventChain::SetVariable {
								variable: o.as_array().unwrap()[1].as_str().unwrap().to_string(),
								value: condition,
							}
						} else {
							chain = events::EventChain::Test { text: o.as_array().unwrap()[0].as_str().unwrap().to_string() };
						}
					}
					"test_variable" => {
						let mut condition: conditionals::Condition = conditionals::Condition::Boolean(false);
						let mut real = true;
						match o.as_array().unwrap()[1].as_array().unwrap()[1] {
							serde_json::value::Value::String {..} => {
								condition = conditionals::Condition::String(o.as_array().unwrap()[1].as_array().unwrap()[1].as_str().unwrap().to_string())
							}
							serde_json::value::Value::Bool {..} => {
								condition = conditionals::Condition::Boolean(o.as_array().unwrap()[1].as_array().unwrap()[1].as_bool().unwrap())
							}
							serde_json::value::Value::Number {..} => {
								condition = conditionals::Condition::Integer(o.as_array().unwrap()[1].as_array().unwrap()[1].as_i64().unwrap() as i32)
							}
							_ => {
								debug::log("[ERROR] - Attempted to put illegal value into a variable.");
								real = false;
							}
						}

						if real {
							chain = events::EventChain::TestVariable {
								variable: o.as_array().unwrap()[1].as_array().unwrap()[0].as_str().unwrap().to_string(),
								value: condition,
								event: o.as_array().unwrap()[2].as_array().unwrap()[0].as_str().unwrap().to_string(),
								position: o.as_array().unwrap()[2].as_array().unwrap()[1].as_i64().unwrap() as i32,
							}
						} else {
							chain = events::EventChain::Test { text: o.as_array().unwrap()[0].as_str().unwrap().to_string() };
						}
					}

					//= Animation events
					"animation" => {
						let mut animOrder: Vec<i32> = Vec::new();
						for i in o.as_array().unwrap()[4].as_array().unwrap() {
							animOrder.push(i.as_i64().unwrap() as i32);
						}

						chain = events::EventChain::PlayAnimation {
							animation: o.as_array().unwrap()[1].as_str().unwrap().to_string(),
							order: animOrder,
							ticks: o.as_array().unwrap()[3].as_i64().unwrap() as i32,
							hold: o.as_array().unwrap()[5].as_bool().unwrap(),
						}
					}
					"emote" => {
						chain = events::EventChain::PlayEmote {
							emote: o.as_array().unwrap()[1].as_str().unwrap().to_string(),
							unit: o.as_array().unwrap()[2].as_str().unwrap().to_string(),
							wait: o.as_array().unwrap()[3].as_bool().unwrap(),
						}
					}

					//= DEBUG
					"DEBUG_print_variables" => {
						chain = events::EventChain::DEBUGPrintVariables;
					}
					_ => {
						chain = events::EventChain::Test { text: o.as_array().unwrap()[0].as_str().unwrap().to_string() };
					}
				}
				event.chain.push(chain);
			}
			self.eventList.insert(i["id"].as_str().unwrap().to_string(), event);
		}
	}

	/// Loads trigger data from input file to hasmap indexed by position.
	pub fn load_triggers( &mut self, mapName : &str ) {
		//* Attempt to load entities file */
		let fileResult_evt = read_to_string("data/world/".to_string() + mapName + "/events.json" );
		if fileResult_evt.is_err() {
			debug::log("[ERROR] - Failed to load map file.\n");
			return;
		}

		//* Convert to JSON and read */
		let jsonFile_evt: serde_json::Value = serde_json::from_str(&fileResult_evt.unwrap()).unwrap();
		for i in jsonFile_evt["triggers"].as_array().unwrap() {
			let pos = [
				i["location"].as_array().unwrap()[0].as_i64().unwrap() as i32,
				i["location"].as_array().unwrap()[1].as_i64().unwrap() as i32,
				i["location"].as_array().unwrap()[2].as_i64().unwrap() as i32,
			];
			self.triggerMap.insert(pos, i["event"].as_str().unwrap().to_string());
		}
	}
}

/// Creates an empty worlddata structure.
pub fn init_empty() -> World {
	return World{
		currentMap		: HashMap::new(),

		unitMap			: HashMap::new(),

		triggerMap		: HashMap::new(),
		eventList		: HashMap::new(),

		eventHandler	: events::event_handler::create(),
	}
}

/// Converts input JSON value into an array of 4 bools representing a collision box.
pub fn solid_tag_to_bool( array : &Vec<serde_json::Value> ) -> [bool; 4] {
	let mut output = [false, false, false, false];

	for i in array {
		match i.as_str().unwrap() {
			"all"	=> output = [true, true, true, true],
			"none"	=> output = [false, false, false, false],
			"north"	=> output[0] = true,
			"south" => output[2] = true,
			"east"	=> output[1] = true,
			"west"	=> output[3] = true,
			_		=> return output,
		}
	}

	return output;
}

/// Draws the world.
pub fn draw_world( gamestate : &mut Gamestate ) {
	let rotation = gamestate.camera.rotation;

	if (rotation > -45.0 && rotation <=  45.0) || (rotation > 315.0 && rotation <= 405.0)	{ return draw_rot_000(gamestate); }
	if (rotation >  45.0 && rotation <= 135.0) || (rotation > 405.0 && rotation <= 495.0)	{ return draw_rot_090(gamestate); }
	if  rotation > 135.0 && rotation <= 225.0												{ return draw_rot_180(gamestate); }
	if (rotation > 225.0 && rotation <= 315.0) || (rotation > -135.0 && rotation <= -45.0)	{ return draw_rot_270(gamestate); }
}

/// Draws tiles and units from a north-facing persepective.
fn draw_rot_000( gamestate : &mut Gamestate ) {
	let playerPosition = math::round_v3(gamestate.player.unit.position);
	let maxX = (playerPosition.x + WIDTH) as i32;
	let minX = (playerPosition.x - WIDTH) as i32;
	let maxY = (playerPosition.y + HEIGHT) as i32;
	let minY = (playerPosition.y - HEIGHT) as i32;
	let maxZ = (playerPosition.z + (DEPTH / 2.0)) as i32;
	let minZ = (playerPosition.z - (DEPTH + (DEPTH / 2.0))) as i32;

	for z in minZ..maxZ {
		let mut x = minX;
		let mut flip = false;

		//* Draw player unit */
		if playerPosition.z.round() as i32 == z-1 {
			overworld::draw_unit(
				&gamestate.animations,
				gamestate.models["unit"],
				&mut gamestate.player.unit,
				gamestate.camera.rotation,
			);
		}

		for _ in minX..maxX {
			for y in minY..maxY {
				//* Check if tile exists */
				if gamestate.worldData.currentMap.contains_key(&[x, y, z]) {
					let tile = &gamestate.worldData.currentMap[&[x, y, z]];
					raylib::draw_model_ex(
						gamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						-360.0,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				for (_, unit) in &mut gamestate.worldData.unitMap {
					if math::equal_v3(unit.position, raylib_ffi::Vector3{x: x as f32, y: y as f32 / 2.0, z: z as f32}) && overworld::exists(&gamestate.worldData.eventHandler, unit) {
						overworld::draw_unit(
							&gamestate.animations,
							gamestate.models["unit"],
							unit,
							gamestate.camera.rotation,
						);
					}
				}
			}
			if !flip	{ x += 1; }
			else		{ x -= 1; }

			if x as f32 >= playerPosition.x && !flip {
				flip = true;
				x = maxX-1;
			}
		}
	}
}

/// Draws tiles and units from a east-facing persepective.
fn draw_rot_090( gamestate : &mut Gamestate ){
	let playerPosition = math::round_v3(gamestate.player.unit.position);
	let maxX = (playerPosition.x + (DEPTH + (DEPTH / 2.0))) as i32;
	let minX = (playerPosition.x - (DEPTH / 2.0)) as i32;
	let maxY = (playerPosition.y + HEIGHT) as i32;
	let minY = (playerPosition.y - HEIGHT) as i32;
	let maxZ = (playerPosition.z + WIDTH) as i32;
	let minZ = (playerPosition.z - WIDTH) as i32;

	for x in (minX..maxX).rev() {
		let mut z = maxZ;
		let mut flip = false;

		//* Draw player unit */
		if playerPosition.x.round() as i32 == x+1 {
			overworld::draw_unit(
				&gamestate.animations,
				gamestate.models["unit"],
				&mut gamestate.player.unit,
				gamestate.camera.rotation,
			);
		}

		for _ in minZ..maxZ {
			for y in minY..maxY {
				//* Check if tile exists */
				if gamestate.worldData.currentMap.contains_key(&[x, y, z]) {
					let tile = &gamestate.worldData.currentMap[&[x, y, z]];
					let mut rot = -360.0;
					//let offset = 0.0;
					// TODO experiment with this
					//if x as f32 > maxX as f32 - (DEPTH / 2.0) { offset = ((x as f32) / (maxX as f32 - (DEPTH / 2.0))) * 1.0; }
					if tile.trnsp { rot = -90.0; }
					raylib::draw_model_ex(
						gamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: (y as f32 / 2.0), z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						rot,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				for (_, unit) in &mut gamestate.worldData.unitMap {
					if math::equal_v3(unit.position, raylib_ffi::Vector3{x: x as f32, y: y as f32 / 2.0, z: z as f32}) && overworld::exists(&gamestate.worldData.eventHandler, unit) {
						overworld::draw_unit(
							&gamestate.animations,
							gamestate.models["unit"],
							unit,
							gamestate.camera.rotation,
						);
					}
				}
			}
			if !flip	{ z -= 1; }
			else		{ z += 1; }

			if z as f32 <= playerPosition.z && !flip {
				flip = true;
				z = minZ+1;
			}
		}
	}
}

/// Draws tiles and units from a south-facing persepective.
fn draw_rot_180( gamestate : &mut Gamestate ) {
	let playerPosition = math::round_v3(gamestate.player.unit.position);
	let maxX = (playerPosition.x + WIDTH) as i32;
	let minX = (playerPosition.x - WIDTH) as i32;
	let maxY = (playerPosition.y + HEIGHT) as i32;
	let minY = (playerPosition.y - HEIGHT) as i32;
	let maxZ = (playerPosition.z + DEPTH) as i32;
	let minZ = (playerPosition.z - DEPTH) as i32;

	for z in (minZ..maxZ).rev() {
		let mut x = maxX;
		let mut flip = false;

		//* Draw player unit */
		if playerPosition.z.round() as i32 == z+1 {
			overworld::draw_unit(
				&gamestate.animations,
				gamestate.models["unit"],
				&mut gamestate.player.unit,
				gamestate.camera.rotation,
			);
		}

		for _ in minX..maxX {
			for y in minY..maxY {
				//* Check if tile exists */
				if gamestate.worldData.currentMap.contains_key(&[x, y, z]) {
					let tile = &gamestate.worldData.currentMap[&[x, y, z]];
					let mut rot = -360.0;
					if tile.trnsp { rot = -180.0; }
					raylib::draw_model_ex(
						gamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						rot,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				for (_, unit) in &mut gamestate.worldData.unitMap {
					if math::equal_v3(unit.position, raylib_ffi::Vector3{x: x as f32, y: y as f32 / 2.0, z: z as f32}) && overworld::exists(&gamestate.worldData.eventHandler, unit) {
						overworld::draw_unit(
							&gamestate.animations,
							gamestate.models["unit"],
							unit,
							gamestate.camera.rotation,
						);
					}
				}
			}
			if !flip	{ x -= 1; }
			else		{ x += 1; }

			if x as f32 <= playerPosition.x && !flip {
				flip = true;
				x = minX+1;
			}
		}
	}
}

/// Draws tiles and units from a west-facing persepective.
fn draw_rot_270( gamestate : &mut Gamestate ) {
	let playerPosition = math::round_v3(gamestate.player.unit.position);
	let maxX = (playerPosition.x + WIDTH) as i32;
	let minX = (playerPosition.x - WIDTH) as i32;
	let maxY = (playerPosition.y + HEIGHT) as i32;
	let minY = (playerPosition.y - HEIGHT) as i32;
	let maxZ = (playerPosition.z + DEPTH) as i32;
	let minZ = (playerPosition.z - DEPTH) as i32;

	for x in minX..maxX {
		let mut z = minZ;
		let mut flip = false;

		//* Draw player unit */
		if playerPosition.x.round() as i32 == x-1 {
			overworld::draw_unit(
				&gamestate.animations,
				gamestate.models["unit"],
				&mut gamestate.player.unit,
				gamestate.camera.rotation,
			);
		}

		for _ in minZ..maxZ {
			for y in minY..maxY {
				//* Check if tile exists */
				if gamestate.worldData.currentMap.contains_key(&[x, y, z]) {
					let tile = &gamestate.worldData.currentMap[&[x, y, z]];
					let mut rot = -360.0;
					if tile.trnsp { rot = -270.0; }
					raylib::draw_model_ex(
						gamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						rot,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				for (_, unit) in &mut gamestate.worldData.unitMap {
					if math::equal_v3(unit.position, raylib_ffi::Vector3{x: x as f32, y: y as f32 / 2.0, z: z as f32}) && overworld::exists(&gamestate.worldData.eventHandler, unit) {
						overworld::draw_unit(
							&gamestate.animations,
							gamestate.models["unit"],
							unit,
							gamestate.camera.rotation,
						);
					}
				}
			}
			if !flip	{ z += 1; }
			else		{ z -= 1; }

			if z as f32 >= playerPosition.z && !flip {
				flip = true;
				z = maxZ-1;
			}
		}
	}
}
