

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, fs::read_to_string, str::FromStr, mem::ManuallyDrop};
use crate::{utilities::{debug, math}, data::Gamestate, overworld, raylib, events};


//= Constants
const OFFSET : f32 = 10.0;
const DEPTH  : f32 = 20.0;
const WIDTH  : f32 = 20.0;
const HEIGHT : f32 = 10.0;


//= Structures
pub struct Tile {
	pub model : String,

	pub solid : [bool;4],
	pub water : bool,
	pub trnsp : bool,
}


//= Procedures
pub fn load_world( mapName : String ) -> HashMap<[i32;3], Tile> {
	let mut output = HashMap::new();

	//* Attempt to load map file */
	let fileResult_map = read_to_string("data/world/".to_string() + &mapName + "/map.json" );
	if fileResult_map.is_err() {
		debug::log("[ERROR] - Failed to load map file.\n");
		return output;
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
		output.insert(position, tile);
	}

	return output;
}

pub fn load_entities( mapName : String ) -> [Option<overworld::Unit>;20] {
	let mut output = create_empty_unitmap();

	//* Attempt to load entities file */
	let fileResult_ent = read_to_string("data/world/".to_string() + &mapName + "/entities.json" );
	if fileResult_ent.is_err() {
		debug::log("[ERROR] - Failed to load map file.\n");
		return output;
	}

	//* Convert to JSON and read */
	let jsonFile_ent: serde_json::Value = serde_json::from_str(&fileResult_ent.unwrap()).unwrap();
	//for i in jsonFile_ent["entities"].as_array().unwrap() {
	let arr = jsonFile_ent["entities"].as_array().unwrap();
	for i in 0..arr.len() {
		let mut unit = overworld::create_unit(arr[i]["sprite"].as_str().unwrap());

		unit.direction = overworld::Direction::from_str(arr[i]["direction"].as_str().unwrap()).unwrap();
		unit.position = raylib_ffi::Vector3{
			x: arr[i]["location"].as_array().unwrap()[0].as_i64().unwrap() as f32,
			y: arr[i]["location"].as_array().unwrap()[1].as_i64().unwrap() as f32,
			z: arr[i]["location"].as_array().unwrap()[2].as_i64().unwrap() as f32,
		};
		unit.posTarget = unit.position;

		for o in arr[i]["events"].as_array().unwrap() {
			let event = events::EntityEvent{
				val: events::create_empty_conditions(),
				key: o["id"].as_str().unwrap().to_string(),
			};
		//	for _e in o["conditions"].as_array().unwrap() {
		//		//TODO Figure out conditions
		//	}
			unit.events[0] = Some(event);
		}

		for _o in arr[i]["conditions"].as_array().unwrap() {
			//TODO Figure out conditions
		}

		output[i] = Some(unit);
		//output.insert(i["id"].as_str().unwrap().to_string(), unit);
	}

	return output;
}

pub fn load_events( mapName : String ) -> HashMap<String, events::Event> {
	let mut output = HashMap::new();

	//* Attempt to load entities file */
	let fileResult_evt = read_to_string("data/world/".to_string() + &mapName + "/events.json" );
	if fileResult_evt.is_err() {
		debug::log("[ERROR] - Failed to load map file.\n");
		return output;
	}

	//* Convert to JSON and read */
	let jsonFile_evt: serde_json::Value = serde_json::from_str(&fileResult_evt.unwrap()).unwrap();
	for i in jsonFile_evt["events"].as_array().unwrap() {
		let mut event: events::Event = events::Event{ chain : Vec::new() };
		for o in i["chain"].as_array().unwrap() {
			let chain: events::EventChain;
			match o.as_array().unwrap()[0].as_str().unwrap() {
				"warp" => chain = events::EventChain{
					warp: ManuallyDrop::new(events::WarpEvent{
						entityID: o.as_array().unwrap()[1].as_str().unwrap().to_string(),
						position: [
							o.as_array().unwrap()[2].as_array().unwrap()[0].as_i64().unwrap() as i32,
							o.as_array().unwrap()[2].as_array().unwrap()[1].as_i64().unwrap() as i32,
							o.as_array().unwrap()[2].as_array().unwrap()[2].as_i64().unwrap() as i32,
							],
						direction: overworld::Direction::from_str(o.as_array().unwrap()[4].as_str().unwrap()).unwrap(),
						doMove: o.as_array().unwrap()[3].as_bool().unwrap(),
					})
				},
				"text" => chain = events::EventChain{
					text: ManuallyDrop::new(events::TextEvent{text: o.as_array().unwrap()[4].as_str().unwrap().to_string()})
				},
				_ => continue,
			}
			event.chain.push(chain);
		}
		output.insert(i["id"].as_str().unwrap().to_string(), event);
	}

	return output;
}

pub fn create_empty_unitmap() -> [Option<overworld::Unit>;20] {
	return [
		None, None,
		None, None,
		None, None,
		None, None,
		None, None,
		None, None,
		None, None,
		None, None,
		None, None,
		None, None,
	];
}

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

pub fn draw_world( gamestate : Gamestate ) -> Gamestate {
	let rotation = gamestate.camera.rotation;

	if (rotation > -45.0 && rotation <=  45.0) || (rotation > 315.0 && rotation <= 405.0)	{ return draw_rot_000(gamestate); }
	if (rotation >  45.0 && rotation <= 135.0) || (rotation > 405.0 && rotation <= 495.0)	{ return draw_rot_090(gamestate); }
	if  rotation > 135.0 && rotation <= 225.0												{ return draw_rot_180(gamestate); }
	if (rotation > 225.0 && rotation <= 315.0) || (rotation > -135.0 && rotation <= -45.0)	{ return draw_rot_270(gamestate); }
	
	return gamestate;
}

fn draw_rot_000( gamestate : Gamestate ) -> Gamestate {
	let mut newGamestate = gamestate;

	let playerPosition = math::round_v3(newGamestate.player.unit.position);
	let maxX = (playerPosition.x + WIDTH) as i32;
	let minX = (playerPosition.x - WIDTH) as i32;
	let maxY = (playerPosition.y + HEIGHT) as i32;
	let minY = (playerPosition.y - HEIGHT) as i32;
	let maxZ = (playerPosition.z + DEPTH) as i32;
	let minZ = (playerPosition.z - DEPTH) as i32;

	for z in minZ..maxZ {
		let mut x = minX;
		let mut flip = false;

		//* Draw player unit */
		if playerPosition.z.round() as i32 == z-1 {
			newGamestate.player.unit = overworld::draw_unit(
				&newGamestate.animations,
				newGamestate.models["unit"],
				newGamestate.player.unit,
				newGamestate.camera.rotation,
			);
		}

		for _ in minX..maxX {
			for y in minY..maxY {
				//* Check if tile exists */
				if newGamestate.currentMap.contains_key(&[x, y, z]) {
					let tile = &newGamestate.currentMap[&[x, y, z]];
					raylib::draw_model_ex(
						newGamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						-360.0,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				for i in newGamestate.unitMap.iter_mut() {
					if i.is_none() { break; }
					let clone = i.clone().expect("").copy_unit();
					*i = Some(overworld::draw_unit(
						&newGamestate.animations,
						newGamestate.models["unit"],
						clone,
						newGamestate.camera.rotation,
					),);
				}
				//for i in 0..newGamestate.unitMap.len() {
				//	let clone = newGamestate.unitMap[i].expect("").copy_unit();
				//	newGamestate.unitMap[i] = Some(overworld::draw_unit(
				//		&newGamestate.animations,
				//		newGamestate.models["unit"],
				//		clone,
				//		newGamestate.camera.rotation,
				//	),);
				//}
				//let mut iter: HashMap<String, Unit> = HashMap::new();
				//for i in &newGamestate.unitMap {
				//	
				//}
				//let iter = newGamestate.unitMap.
				//for (string, unit) in gamestate.unitMap.iter() {
				//	let v3 = raylib_ffi::Vector3{x: x as f32,y: y as f32,z: z as f32};
				//	if math::equal_v3(unit.position, v3) {
				//		let key = string.to_string();
				//		newGamestate.unitMap.insert(
				//			key, 
				//			overworld::draw_unit(
				//				&newGamestate.animations,
				//				newGamestate.models["unit"],
				//				overworld::copy_unit(unit),
				//				newGamestate.camera.rotation,
				//			),
				//		);
				//	}
				//}
				//let iter = &mut newGamestate.unitMap;
				//for (key, value) in &*iter {
				//	let v3 = raylib_ffi::Vector3{x: x as f32,y: y as f32,z: z as f32};
				//	if math::equal_v3(value.position, v3) {
				//		let key = key.to_string();
				//		newGamestate.unitMap.insert(
				//			key, 
				//			overworld::draw_unit(
				//				&newGamestate.animations,
				//				newGamestate.models["unit"],
				//				newGamestate.unitMap.get(&key).unwrap().clone(),
				//				newGamestate.camera.rotation,
				//			),
				//		);
				//	}
				//}
			}
			if !flip	{ x += 1; }
			else		{ x -= 1; }

			if x as f32 >= playerPosition.x && !flip {
				flip = true;
				x = maxX-1;
			}
		}
	}

	return newGamestate
}

fn draw_rot_090( gamestate : Gamestate ) -> Gamestate {
	let mut newGamestate = gamestate;

	let playerPosition = math::round_v3(newGamestate.player.unit.position);
	let maxX = (playerPosition.x + WIDTH) as i32;
	let minX = (playerPosition.x - WIDTH) as i32;
	let maxY = (playerPosition.y + HEIGHT) as i32;
	let minY = (playerPosition.y - HEIGHT) as i32;
	let maxZ = (playerPosition.z + DEPTH) as i32;
	let minZ = (playerPosition.z - DEPTH) as i32;

	for x in (minX..maxX).rev() {
		let mut z = maxZ;
		let mut flip = false;

		//* Draw player unit */
		if playerPosition.x.round() as i32 == x+1 {
			newGamestate.player.unit = overworld::draw_unit(
				&newGamestate.animations,
				newGamestate.models["unit"],
				newGamestate.player.unit,
				newGamestate.camera.rotation,
			);
		}

		for _ in minZ..maxZ {
			for y in minY..maxY {
				//* Check if tile exists */
				if newGamestate.currentMap.contains_key(&[x, y, z]) {
					let tile = &newGamestate.currentMap[&[x, y, z]];
					let mut rot = -360.0;
					if tile.trnsp { rot = -90.0; }
					raylib::draw_model_ex(
						newGamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						rot,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				// TODO
			}
			if !flip	{ z -= 1; }
			else		{ z += 1; }

			if z as f32 <= playerPosition.z && !flip {
				flip = true;
				z = minZ+1;
			}
		}
	}

	return newGamestate
}

fn draw_rot_180( gamestate : Gamestate ) -> Gamestate {
	let mut newGamestate = gamestate;

	let playerPosition = math::round_v3(newGamestate.player.unit.position);
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
			newGamestate.player.unit = overworld::draw_unit(
				&newGamestate.animations,
				newGamestate.models["unit"],
				newGamestate.player.unit,
				newGamestate.camera.rotation,
			);
		}

		for _ in minX..maxX {
			for y in minY..maxY {
				//* Check if tile exists */
				if newGamestate.currentMap.contains_key(&[x, y, z]) {
					let tile = &newGamestate.currentMap[&[x, y, z]];
					let mut rot = -360.0;
					if tile.trnsp { rot = -180.0; }
					raylib::draw_model_ex(
						newGamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						rot,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				// TODO
			}
			if !flip	{ x -= 1; }
			else		{ x += 1; }

			if x as f32 <= playerPosition.x && !flip {
				flip = true;
				x = minX+1;
			}
		}
	}

	return newGamestate
}

fn draw_rot_270( gamestate : Gamestate ) -> Gamestate {
	let mut newGamestate = gamestate;

	let playerPosition = math::round_v3(newGamestate.player.unit.position);
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
			newGamestate.player.unit = overworld::draw_unit(
				&newGamestate.animations,
				newGamestate.models["unit"],
				newGamestate.player.unit,
				newGamestate.camera.rotation,
			);
		}

		for _ in minZ..maxZ {
			for y in minY..maxY {
				//* Check if tile exists */
				if newGamestate.currentMap.contains_key(&[x, y, z]) {
					let tile = &newGamestate.currentMap[&[x, y, z]];
					let mut rot = -360.0;
					if tile.trnsp { rot = -270.0; }
					raylib::draw_model_ex(
						newGamestate.models[tile.model.as_str()],
						raylib_ffi::Vector3 {x: x as f32, y: y as f32 / 2.0, z: z as f32},
						raylib_ffi::Vector3 {x: 0.0, y: 1.0, z: 0.0},
						rot,
						raylib_ffi::Vector3 {x: 1.0, y: 1.0, z: 1.0},
						raylib_ffi::colors::WHITE,
					)
				}
				//* Check if unit exists */
				// TODO
			}
			if !flip	{ z += 1; }
			else		{ z -= 1; }

			if z as f32 >= playerPosition.z && !flip {
				flip = true;
				z = maxZ-1;
			}
		}
	}

	return newGamestate
}