

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::collections::HashMap;

//= Imports
use crate::{monsters, world::Tile, raylib::vectors::Vector3, data};


//= Enumerations

/// The type of battle
#[derive(Clone)]
pub enum BattleType {
	Empty,
	/// Data for a Single Battle
	Single{
		trainerName: String,
	
		easyTeam:	monsters::MonsterTeam,
		mediumTeam:	monsters::MonsterTeam,
		hardTeam:	monsters::MonsterTeam,

		arena: ArenaType,
	},
	/// Data for a Double Battle
	Double{
		trainerName: [String;2],
		singleTrainer: bool,
	
		easyTeam:	[monsters::MonsterTeam;2],
		mediumTeam:	[monsters::MonsterTeam;2],
		hardTeam:	[monsters::MonsterTeam;2],

		arena: ArenaType,
	},
	/// Data for a Wild Battle
	Wild{
		monster: monsters::Monster,

		arena: ArenaType,
	},
}

/// The type of arena
#[derive(Clone, Copy, PartialEq)]
pub enum ArenaType {
	Field,
	Forest,
	City,
}
impl From<&str> for ArenaType {
	fn from(value: &str) -> Self {
		match value {
			"field" => ArenaType::Field,
			"forest" => ArenaType::Forest,
			"city" => ArenaType::City,
			_ => ArenaType::Field,
		}
	}
}

///
#[derive(Clone)]
pub enum BattleObjectType {
	Delete,
	PlayerMonster{num: i32},
	EnemyMonster{num: i32},
}


//= Structures

/// The general battle data struct
pub struct BattleData {
	//* Battle variables */
	pub started: bool,

	//* Data */
	pub battleType: BattleType,

	pub tiles: HashMap<[i32;3], Tile>,
	pub objects: HashMap<String, BattleObject>,
}

///
#[derive(Clone)]
pub struct BattleObject {
	pub objType: BattleObjectType,
	pub position: Vector3,
}


//= Procedures

impl BattleType {
	

}

impl BattleData {
	
	/// Create new battle structure
	pub fn init() -> Self {
		return BattleData{
			started:	false,
    		battleType:	BattleType::Empty,

			tiles: HashMap::new(),
			objects: HashMap::new(),
		}
	}

	/// Start battle
	/// <br>If ``BattleType::Empty input``, it clears the structure.
	pub fn start_battle(&mut self, battle: BattleType) {
		match battle {
			BattleType::Single { arena, .. } => {
				self.started = true;
				self.battleType = battle;

				self.tiles = BattleData::create_arena(arena);
			}
			BattleType::Double { arena, .. } => {
				self.started = true;
				self.battleType = battle;

				self.tiles = BattleData::create_arena(arena);
			}
			BattleType::Wild { arena, .. } => {
				self.started = true;
				self.battleType = battle;

				self.tiles = BattleData::create_arena(arena);
			}
			BattleType::Empty => {
				self.started = false;
				self.battleType = BattleType::Empty;
			}
		}
	}

	pub fn create_arena(arena: ArenaType) -> HashMap<[i32;3], Tile> {
		let mut result: HashMap<[i32;3], Tile> = HashMap::new();

		match arena {
			ArenaType::Field => {
				let grass = Tile::create("grass_1", false, false);

				// TEMP
				for z in -13..10 {
					for x in -16..32 {
						result.insert([x,0,z], grass.clone());
					}
				}

				return result;
			}
			ArenaType::Forest => {
				let grass = Tile::create("grass_1", false, false);

				// TEMP
				for z in 0..8 {
					for x in 0..16 {
						result.insert([x,0,z], grass.clone());
					}
				}

				return result;
			}
			ArenaType::City => {
				let grass = Tile::create("grass_1", false, false);

				// TEMP
				for z in 0..8 {
					for x in 0..16 {
						result.insert([x,0,z], grass.clone());
					}
				}

				return result;
			}
		}
	}

	/// Updates battle state
	pub fn update(&self) {
		
	}

}

/// Draws battle to screen
pub fn draw(gamestate: &mut data::Gamestate) {
	for z in -13..10 {
		for x in -16..32 {
			//* Tiles */
			if gamestate.battleData.tiles.contains_key(&[x,0,z]) {
				let tile = gamestate.battleData.tiles[&[x,0,z]].clone();
				gamestate.graphics.models[&tile.model].draw_ex(
					Vector3::from([x,0,z]) + gamestate.camera.position - Vector3{x:7.5,y:0.0,z:4.5},
					Vector3{x:0.0,y:1.0,z:0.0},
					0.0,
					Vector3{x:1.0,y:1.0,z:1.0},
					raylib_ffi::colors::WHITE,
				);
			}

			//* Objects */
			let mut obj: BattleObject;
			for (_, object) in gamestate.battleData.objects.iter() {
				let objPos: [i32;3] = object.position.into();
				if [x,0,z] == objPos { obj = object.clone(); }
			}
			// DRAW
		}
	}
	//for (pos, tile) in gamestate.battleData.tiles.iter() {
	//	gamestate.graphics.models[&tile.model].draw_ex(
	//		Vector3::from(*pos) + gamestate.camera.position - Vector3{x:7.5,y:0.0,z:4.5},
	//		Vector3{x:0.0,y:1.0,z:0.0},
	//		0.0,
	//		Vector3{x:1.0,y:1.0,z:1.0},
	//		raylib_ffi::colors::WHITE,
	//	);
	//}
}