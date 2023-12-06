

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


use std::collections::HashMap;

//= Imports
use crate::{monsters::{self, MonsterSpecies}, world::Tile, raylib::{vectors::Vector3, textures::Texture}, data};


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
	PlayerMonster{num: i32, species: MonsterSpecies},
	EnemyMonster{num: i32, species: MonsterSpecies},
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

impl BattleObject {
	
	//
	pub fn new(objType: BattleObjectType, position: Vector3) -> Self {
		Self {
			objType,
			position,
		}
	}

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

				self.objects = HashMap::new();
				self.objects.insert(
					"player".to_string(), 
					BattleObject::new(
						BattleObjectType::PlayerMonster{num: 0, species: MonsterSpecies::Mon152},
						Vector3{x:4.0,y:0.0,z:4.0},
					),
				);
				self.objects.insert(
					"enemy".to_string(), 
					BattleObject::new(
						BattleObjectType::EnemyMonster{num: 0, species: MonsterSpecies::Mon158},
						Vector3{x:12.0,y:0.0,z:4.0},
					),
				);

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
				//let grass = Tile::create("grass_1", false, false);
				let battle1 = Tile::create("battle_1", false, false);
				let battle2 = Tile::create("battle_2", false, false);
				let battle3 = Tile::create("battle_3", false, false);

				// TEMP
				//for z in -13..10 {
				//	for x in -16..32 {
				for z in 0..8 {
					for x in 0..16 {
						if x == 7 { result.insert([x,0,z], battle2.clone()); }
						else if x == 8 { result.insert([x,0,z], battle3.clone()); }
						else { result.insert([x,0,z], battle1.clone()); }
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
	pub fn update(&mut self) {
		let obj = self.objects.get_mut("player").unwrap();
		if data::key_pressed("up")		{ obj.position = obj.position - Vector3{x:0.0,y:0.0,z:1.0}; }
		if data::key_pressed("down")	{ obj.position = obj.position + Vector3{x:0.0,y:0.0,z:1.0}; }
		if data::key_pressed("left")	{ obj.position = obj.position - Vector3{x:1.0,y:0.0,z:0.0}; }
		if data::key_pressed("right")	{ obj.position = obj.position + Vector3{x:1.0,y:0.0,z:0.0}; }

		if obj.position.x < 0.0 { obj.position.x = 0.0; }
		if obj.position.x > 7.0 { obj.position.x = 7.0; }
		if obj.position.z < 0.0 { obj.position.z = 0.0; }
		if obj.position.z > 7.0 { obj.position.z = 7.0; }
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
			let mut obj: Option<BattleObject> = None;
			for (_, object) in gamestate.battleData.objects.iter() {
				let objPos: [i32;3] = object.position.into();
				if [x-1,0,z] == objPos { obj = Some(object.clone()); }
			}
			// DRAW
			if obj.is_some() {
				//* Update material */
				let texture: Texture;
				match &obj.as_ref().unwrap().objType {
					BattleObjectType::EnemyMonster { species, .. } => {
						texture = gamestate.graphics.textures[&(species.to_string() + "_6")];
					}
					BattleObjectType::PlayerMonster { species, .. } => {
						texture = gamestate.graphics.textures[&(species.to_string() + "_8")];
					}
					_ => { return }
				}
				let mut model = gamestate.graphics.models["unit"].clone();
				model.set_material_texture(texture);

				//* Draw */
				let position = obj.as_ref().unwrap().position + gamestate.camera.position - Vector3{x:7.5,y:0.0,z:4.0};
				model.draw_ex(
					position,
					Vector3{x:1.0,y:0.0,z:0.0},
					-45.0,
					Vector3{x:1.5,y:1.5,z:1.5},
					raylib_ffi::colors::WHITE,
				);
			}
		}
	}
}