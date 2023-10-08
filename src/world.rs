

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, fs::read_to_string};
use crate::{utilities::{debug, math}, data::Gamestate, overworld, raylib};


//= Constants
const OFFSET : f32 = 10.0;
const DEPTH  : f32 = 20.0;
const WIDTH  : f32 = 20.0;
const HEIGHT : f32 = 10.0;


//= Structures
pub struct Tile {
	model : String,

	solid : [bool;4],
	water : bool,
	trnsp : bool,
}


//= Procedures
pub fn load_world( mapName : String ) -> HashMap<[i32;3], Tile> {
	let mut output = HashMap::new();

	//* Attempt to load file */
	let fileResult = read_to_string(mapName);
	if fileResult.is_err() {
		debug::log("[ERROR] - Failed to load map file.\n");
		return output;
	}

	//* Convert to JSON and read */
	let jsonFile: serde_json::Value = serde_json::from_str(&fileResult.unwrap()).unwrap();
	for i in jsonFile["tiles"].as_array().unwrap() {
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

pub fn solid_tag_to_bool( array : &Vec<serde_json::Value> ) -> [bool; 4] {
	let mut output = [false, false, false, false];

	for i in array {
		match i.to_string().as_str() {
			"all"	=> output = [true, true, true, true],
			"none"	=> output = [false, false, false, false],
			"north"	=> output[0] = true,
			"south" => output[2] = true,
			"east"	=> output[1] = true,
			"west"	=> output[3] = true,
			_		=> output = [false, false, false, false],
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
				// TODO
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