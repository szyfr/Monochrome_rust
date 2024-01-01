

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{monsters::{self, MonsterSpecies, MonsterAttacks}, world::Tile, raylib::{vectors::{Vector3, Vector2}, textures::Texture, self, rectangles::Rectangle}, data, utilities::math::is_within_range, settings::Difficulty};
use std::collections::HashMap;


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

/// Player Battle State
#[derive(Clone, PartialEq)]
pub enum PlayerBattleState {
	Movement,
	Attack,
}

//= Structures

/// The general battle data struct
pub struct BattleData {
	//* Battle variables */
	pub started: bool,

	pub turnCur: i8,
	pub roundTotal: i32,
	pub turnOrder: [i8;4],

	pub playerState: PlayerBattleState,
	pub cursor: Vector3,

	pub attackChoice: i32,

	//* Data */
	pub battleType: BattleType,
	pub playerTeam: monsters::MonsterTeam,
	pub enemyTeam:  monsters::MonsterTeam,

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

			turnCur: 0,
			roundTotal: 0,
			turnOrder:	[-1,-1,-1,-1],

			playerState: PlayerBattleState::Movement,
			cursor: Vector3{x:4.0,y:0.0,z:4.0},

			attackChoice: 0,

    		battleType:	BattleType::Empty,
			playerTeam: monsters::MonsterTeam::new(),
			enemyTeam: monsters::MonsterTeam::new(),

			tiles: HashMap::new(),
			objects: HashMap::new(),
		}
	}

	/// Start battle
	/// <br>If ``BattleType::Empty input``, it clears the structure.
	pub fn start_battle(&mut self, battle: BattleType, playerTeam: &monsters::MonsterTeam) {
		match battle {
			BattleType::Single { arena, trainerName, easyTeam, mediumTeam, hardTeam } => {
				self.started = true;
				self.battleType = BattleType::Single{
					trainerName: trainerName.clone(),
					easyTeam: easyTeam.clone(),
					mediumTeam: mediumTeam.clone(),
					hardTeam: hardTeam.clone(),
					arena: arena.clone(),
				};
				self.playerTeam = playerTeam.clone();
				match data::get_difficulty() {
					Difficulty::Easy	=> { self.enemyTeam = easyTeam; }
					Difficulty::Medium	=> { self.enemyTeam = mediumTeam; }
					Difficulty::Hard	=> { self.enemyTeam = hardTeam; }
				}

				self.turnOrder = calc_turn_order([playerTeam.0[0].clone(), None, self.enemyTeam.0[0].clone(), None]);
				print!("[{},{},{},{}]\n",self.turnOrder[0],self.turnOrder[1],self.turnOrder[2],self.turnOrder[3]);

				self.objects = HashMap::new();
				self.objects.insert(
					"player_1".to_string(), 
					BattleObject::new(
						BattleObjectType::PlayerMonster{num: 0, species: self.playerTeam.0[0].as_ref().unwrap().species.clone()},
						Vector3{x:4.0,y:0.0,z:4.0},
					),
				);
				self.objects.insert(
					"enemy_1".to_string(), 
					BattleObject::new(
						BattleObjectType::EnemyMonster{num: 0, species: self.enemyTeam.0[0].as_ref().unwrap().species.clone()},
						Vector3{x:11.0,y:0.0,z:4.0},
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
		if data::key_pressed("swap_modes") {
			if self.playerState == PlayerBattleState::Movement { self.playerState = PlayerBattleState::Attack }
			else if self.playerState == PlayerBattleState::Attack { self.playerState = PlayerBattleState::Movement }
		}
		match self.turnOrder[self.turnCur as usize] {
			0 => { // Player mon 1
				match self.playerState {
					PlayerBattleState::Movement => {
						if data::key_pressed("up")		{ self.cursor = self.cursor - Vector3{x:0.0,y:0.0,z:1.0}; }
						if data::key_pressed("down")	{ self.cursor = self.cursor + Vector3{x:0.0,y:0.0,z:1.0}; }
						if data::key_pressed("left")	{ self.cursor = self.cursor - Vector3{x:1.0,y:0.0,z:0.0}; }
						if data::key_pressed("right")	{ self.cursor = self.cursor + Vector3{x:1.0,y:0.0,z:0.0}; }

						if self.cursor.x < 0.0 { self.cursor.x = 0.0; }
						if self.cursor.x > 7.0 { self.cursor.x = 7.0; }
						if self.cursor.z < 0.0 { self.cursor.z = 0.0; }
						if self.cursor.z > 7.0 { self.cursor.z = 7.0; }
					}
					PlayerBattleState::Attack => {
						if data::key_pressed("attack_1") && self.playerTeam.0[0].as_ref().unwrap().attacks[0] != MonsterAttacks::None { self.attackChoice = 0; }
						if data::key_pressed("attack_2") && self.playerTeam.0[0].as_ref().unwrap().attacks[1] != MonsterAttacks::None { self.attackChoice = 1; }
						if data::key_pressed("attack_3") && self.playerTeam.0[0].as_ref().unwrap().attacks[2] != MonsterAttacks::None { self.attackChoice = 2; }
						if data::key_pressed("attack_4") && self.playerTeam.0[0].as_ref().unwrap().attacks[3] != MonsterAttacks::None { self.attackChoice = 3; }
						
						if data::key_pressed("up")		{ self.cursor = self.cursor - Vector3{x:0.0,y:0.0,z:1.0}; }
						if data::key_pressed("down")	{ self.cursor = self.cursor + Vector3{x:0.0,y:0.0,z:1.0}; }
						if data::key_pressed("left")	{ self.cursor = self.cursor - Vector3{x:1.0,y:0.0,z:0.0}; }
						if data::key_pressed("right")	{ self.cursor = self.cursor + Vector3{x:1.0,y:0.0,z:0.0}; }

						if self.cursor.x < 8.0 { self.cursor.x = 8.0; }
						if self.cursor.x > 16.0 { self.cursor.x = 16.0; }
						if self.cursor.z < 0.0 { self.cursor.z = 0.0; }
						if self.cursor.z > 7.0 { self.cursor.z = 7.0; }

						// TODO This doesn't work for some reason...
						if data::key_pressed("shift") && data::key_pressed("left")	{ self.attackChoice = self.attackChoice - 1; }
						if data::key_pressed("shift") && data::key_pressed("right")	{ self.attackChoice = self.attackChoice + 1; }

						if self.attackChoice < 0 { self.attackChoice = 0; }
						if self.attackChoice > 3 { self.attackChoice = 3; }
					}
				}
				if data::key_pressed("confirm")	{
					print!("Player turn ends.\n");
					self.next_turn();
				}
			}
			1 => {} // Player mon 2
			2 => { // Enemy mon 1
				print!("Enemy turn ends.\n");
				self.next_turn();
			}
			3 => { // Enemy mon 2
				print!("Enemy turn ends.\n");
				self.next_turn();
			}
			_ => {} // Null
		}

		
	}

	pub fn next_turn(&mut self) {
		self.turnCur += 1;

		//* If turn is invalid, reset round */
		if self.turnCur >= 4 || self.turnOrder[self.turnCur as usize] == -1 {
			print!("Round ends.\n");
			self.roundTotal += 1;
			self.turnCur = 0;
		}
	}

}

/// Draws battle to screen
pub fn draw(gamestate: &mut data::Gamestate) {
	for z in -13..10 {
		for x in -16..32 {
			//* Tiles */
			if gamestate.battleData.tiles.contains_key(&[x,0,z]) {
				let tile = gamestate.battleData.tiles[&[x,0,z]].clone();
				let mut color: raylib_ffi::Color = raylib_ffi::colors::WHITE;

				let chosenMon = gamestate.battleData.turnOrder[gamestate.battleData.turnCur as usize];
				
				match gamestate.battleData.playerState {
					PlayerBattleState::Movement => {
						if x < 8 && z < 8 {
							// TODO Double battle
							if chosenMon == 0 {
								let objPosition = gamestate.battleData.objects["player_1"].position;
								if is_within_range(objPosition, Vector3::from([x,0,z]), gamestate.battleData.playerTeam.0[0].as_ref().unwrap().get_move_distance()) {
									color = raylib_ffi::colors::ORANGE;
								}
							}
							if Vector3::from([x,0,z]) == gamestate.battleData.cursor { color = raylib_ffi::colors::GREEN; }
						}
					}
					PlayerBattleState::Attack => {
						if chosenMon == 0 {
							let monster = gamestate.battleData.playerTeam.0[0].as_ref().unwrap();
							let monPosi = gamestate.battleData.objects["player_1"].position;
							match monster.attacks[gamestate.battleData.attackChoice as usize] {
								MonsterAttacks::Tackle => {
									//* Player side */
									if x > 3 && x < 8 {
										if monPosi.x > 3.0 { color = raylib_ffi::colors::GREEN; }
										else { color = raylib_ffi::colors::RED; }
									}
									//* Enemy side */
									if x > 7 && x < 12 && z > (monPosi.z - 2.0) as i32 && z < (monPosi.z + 2.0) as i32 { color = raylib_ffi::colors::BLUE; }
								}
								MonsterAttacks::Scratch => {
									//* Player side */
									if x > 3 && x < 8 {
										if monPosi.x > 3.0 && monPosi.x < 8.0 { color = raylib_ffi::colors::GREEN; }
										else { color = raylib_ffi::colors::RED; }
									}
									//* Enemy side */
									if x > 7 && x < 11 && z > (monPosi.z - 3.0) as i32 && z < (monPosi.z + 3.0) as i32 { color = raylib_ffi::colors::BLUE; }
								}
								MonsterAttacks::Growl => {
									if x > 7 { color = raylib_ffi::colors::BLUE; }
								}
								MonsterAttacks::Leer => {
									if x > 7 { color = raylib_ffi::colors::BLUE; }
								}
								MonsterAttacks::Leafage => {
									let mut isFront = false;
									if monPosi.x > 3.0 {
										isFront = true;
										if x > 11 && x < 16 { color = raylib_ffi::colors::ORANGE; }
									}
									if monPosi.x < 4.0 {
										isFront = false;
										if x > 7 && x < 12 { color = raylib_ffi::colors::ORANGE; }
									}
									if x > (gamestate.battleData.cursor.x - 2.0) as i32 && x < (gamestate.battleData.cursor.x + 2.0) as i32 && z > (gamestate.battleData.cursor.z - 2.0) as i32 && z < (gamestate.battleData.cursor.z + 2.0) as i32 {
										if isFront  && 11 < gamestate.battleData.cursor.x as i32 { color = raylib_ffi::colors::BLUE; }
										if isFront  && 12 > gamestate.battleData.cursor.x as i32 { color = raylib_ffi::colors::RED; }
										if !isFront && 11 < gamestate.battleData.cursor.x as i32 { color = raylib_ffi::colors::RED; }
										if !isFront && 12 > gamestate.battleData.cursor.x as i32 { color = raylib_ffi::colors::BLUE; }
									}
								}
								MonsterAttacks::Ember => {
									if monPosi.z as i32 == z && x > monPosi.x as i32 { color = raylib_ffi::colors::BLUE; }
								}
								MonsterAttacks::Aquajet => {
									if monPosi.x as i32 > 3 {
										if x > 3 && x < 8 { color = raylib_ffi::colors::GREEN; }
										if x > 11 && z > (monPosi.z - 2.0) as i32 && z < (monPosi.z + 2.0) as i32 { color = raylib_ffi::colors::BLUE; }
									} else {
										if x < 4 { color = raylib_ffi::colors::GREEN; }
										if x > 7 && x < 12 && z > (monPosi.z - 2.0) as i32 && z < (monPosi.z + 2.0) as i32 { color = raylib_ffi::colors::BLUE; }
									}
								}
								_ => {}
							}
						}
					}
				}
				

				let model = gamestate.graphics.models[&tile.model].clone();
				raylib::set_shader_value(
					gamestate.graphics.tileShader.unwrap(),
					gamestate.graphics.colorLoc,
					[(color.r as f32 / 255.0), (color.g as f32 / 255.0), (color.b as f32 / 255.0), (color.a as f32 / 255.0)].as_ptr().cast(),
					raylib::enums::ShaderUniformDataType::ShaderUniformVec4,
				);

				model.draw_ex(
					Vector3::from([x,0,z]) + gamestate.camera.position - Vector3{x:7.5,y:0.0,z:4.5},
					Vector3{x:0.0,y:1.0,z:0.0},
					0.0,
					Vector3{x:1.0,y:1.0,z:1.0},
					raylib_ffi::colors::BLACK,
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

			//* Cursor */
			//TODO
		}
	}
}

pub fn draw_ui(gamestate: &mut data::Gamestate) {
	//* Current combat state */
	match gamestate.battleData.playerState {
		PlayerBattleState::Movement => {
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				Rectangle{
					x: data::get_screenwidth() as f32 - 200.0,
					y: 100.0 * data::get_screenratio(),
					width: 300.0 * data::get_screenratio(),
					height: 100.0 * data::get_screenratio(),
				},
				0.0,
			);
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				Rectangle{
					x: data::get_screenwidth() as f32 - 50.0,
					y: 200.0 * data::get_screenratio(),
					width: 300.0 * data::get_screenratio(),
					height: 100.0 * data::get_screenratio(),
				},
				0.0,
			);
			gamestate.graphics.fonts["default"].draw(
				&gamestate.localization["battle_text_movement"],
				data::get_screenwidth() - 150,
				(145.0 * data::get_screenratio()) as i32,
				(20.0 * data::get_screenratio()) as i32,
				raylib_ffi::colors::BLACK,
			);
		}
		PlayerBattleState::Attack => {
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				Rectangle{
					x: data::get_screenwidth() as f32 - 50.0,
					y: 100.0 * data::get_screenratio(),
					width: 300.0 * data::get_screenratio(),
					height: 100.0 * data::get_screenratio(),
				},
				0.0,
			);
			gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
				Rectangle{
					x: data::get_screenwidth() as f32 - 200.0,
					y: 200.0 * data::get_screenratio(),
					width: 300.0 * data::get_screenratio(),
					height: 100.0 * data::get_screenratio(),
				},
				0.0,
			);
			gamestate.graphics.fonts["default"].draw(
				&gamestate.localization["battle_text_attack"],
				data::get_screenwidth() - 150,
				(245.0 * data::get_screenratio()) as i32,
				(20.0 * data::get_screenratio()) as i32,
				raylib_ffi::colors::BLACK,
			);
		}
	}

	//* Battle status */
	//TODO Double battle
	// Player side
	gamestate.graphics.textures["ui_battlestatus_general"].draw_ex(
		Vector2{
			x:384.0 * data::get_screenratio() as f32,
			y: 96.0 * data::get_screenratio() as f32,
		},
		180.0,
		1.0,
	);

	//TODO Have a look to see if i can do this better
	let playermon = gamestate.battleData.playerTeam.0[0].as_ref().unwrap();
	let name = playermon.get_name();
	let str: String;
	if name.1 { str = name.0; }
	else { str = gamestate.localization[&name.0].to_string(); }

	gamestate.graphics.fonts["default"].draw_pro(
		&str,
		Vector2 {
			x: 26.0 * data::get_screenratio() as f32,
			y: 26.0 * data::get_screenratio() as f32,
		},
		0.0,
		16.0 * data::get_screenratio() as f32,
		0.0,
		raylib_ffi::colors::BLACK,
	);

	let hpStr = playermon.hpCur.to_string() + " / " + &playermon.hpMax.to_string();
	gamestate.graphics.fonts["default"].draw_pro(
		&hpStr,
		Vector2 {
			x: 26.0 * data::get_screenratio() as f32,
			y: 58.0 * data::get_screenratio() as f32,
		},
		0.0,
		16.0 * data::get_screenratio() as f32,
		0.0,
		raylib_ffi::colors::BLACK,
	);

	// Enemy side
	gamestate.graphics.textures["ui_battlestatus_general"].draw_ex(
		Vector2{
			x: data::get_screenwidth() as f32 - (384.0 * data::get_screenratio() as f32),
			y: 0.0,
		},
		0.0,
		1.0,
	);
	let enemyStr = gamestate.localization[&gamestate.battleData.enemyTeam.0[0].as_ref().unwrap().species.to_string()].to_string();
	let offset = enemyStr.len() as f32 * 16.0;
	gamestate.graphics.fonts["default"].draw_pro(
		&enemyStr,
		Vector2 {
			//x: data::get_screenwidth() as f32 - (384.0 * data::get_screenratio() as f32) + (106.0 * data::get_screenratio() as f32),
			x: data::get_screenwidth() as f32 - (26.0 + offset),
			y: 26.0 * data::get_screenratio() as f32,
		},
		0.0,
		16.0 * data::get_screenratio() as f32,
		0.0,
		raylib_ffi::colors::BLACK,
	);

	//* Attack UI */
	//TODO Edit this
	for i in 0..4 {
		let yOffset: i32;
		if gamestate.battleData.attackChoice == i && gamestate.battleData.playerState == PlayerBattleState::Attack { yOffset = (50.0 * data::get_screenratio()) as i32; }
		else { yOffset = 0; }

		let attacks = gamestate.battleData.playerTeam.0[0].clone().unwrap().attacks;
		gamestate.graphics.textures["ui_textbox_general"].draw_npatch(
			Rectangle{
				x: (25.0 * data::get_screenratio()) + ((200.0 * data::get_screenratio()) * i as f32),
				y: data::get_screenheight() as f32 - (60.0 * data::get_screenratio()) - yOffset as f32,
				width: 200.0 * data::get_screenratio(),
				height: 200.0 * data::get_screenratio(),
			},
			0.0,
		);
		if attacks[i as usize] != MonsterAttacks::None {
			gamestate.graphics.fonts["default"].draw_pro(
				&gamestate.localization[&attacks[i as usize].to_string()],
				Vector2{
					x: (50.0 * data::get_screenratio()) + ((200.0 * data::get_screenratio()) * i as f32),
					y: data::get_screenheight() as f32 - (25.0 * data::get_screenratio()) - yOffset as f32,
				},
				0.0,
				16.0 * data::get_screenratio(),
				0.0,
				raylib_ffi::colors::BLACK,
			);
		}
	}
}

pub fn calc_turn_order(monsters: [Option<monsters::Monster>;4]) -> [i8;4] {
	//* This is the stupid way of doing it, but i am that... so */
	let mut result: [i8;4] = [-1,-1,-1,-1];

	let mut largest = 0;
	let mut member: i32 = -1;
	for x in 0..4 {
		if monsters[x as usize].is_none() { continue; }
		if monsters[x as usize].clone().unwrap().speed > largest { member = x; }
	}
	if member == -1 { return result; }
	result[0] = member as i8;
	largest = 0;
	member = -1;

	for x in 0..4 {
		if monsters[x as usize].is_none() { continue; }
		if x == result[0] { continue; }
		if monsters[x as usize].clone().unwrap().speed > largest { member = x as i32; }
	}
	if member == -1 { return result; }
	result[1] = member as i8;
	largest = 0;
	member = -1;

	for x in 0..4 {
		if monsters[x as usize].is_none() { continue; }
		if x == result[0] || x == result[1] { continue; }
		if monsters[x as usize].clone().unwrap().speed > largest { member = x as i32; }
	}
	if member == -1 { return result; }
	result[2] = member as i8;
	largest = 0;
	member = -1;
	
	for x in 0..4 {
		if monsters[x as usize].is_none() { continue; }
		if x == result[0] || x == result[1] || x == result[2] { continue; }
		if monsters[x as usize].clone().unwrap().speed > largest { member = x as i32; }
	}
	if member == -1 { return result; }
	result[3] = member as i8;

	return result;
}