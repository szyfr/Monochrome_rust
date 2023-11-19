

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{fmt::Display, str::FromStr};

use crate::utilities::debug;


//= Enumerations

/// The monster's species.
#[derive(Clone)]
pub enum MonsterSpecies {
	Mon152, //* Grass starter */
	Mon155, //* Fire starter */
	Mon158, //* Water starter */
}
impl Display for MonsterSpecies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self {
			MonsterSpecies::Mon152 => return write!(f, "mon152"),
			MonsterSpecies::Mon155 => return write!(f, "mon155"),
			MonsterSpecies::Mon158 => return write!(f, "mon158"),
		}
    }
}
impl FromStr for MonsterSpecies {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
			"mon152" => Ok(MonsterSpecies::Mon152),
			"mon155" => Ok(MonsterSpecies::Mon155),
			"mon158" => Ok(MonsterSpecies::Mon158),
			_ => Err(()),
		}
    }
}

/// The elemental typing of a monster.
#[derive(Clone)]
pub enum MonsterTypes {
	None,
	Normal,
	Fire,
	Water,
	Grass,
}
impl Display for MonsterTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self {
			MonsterTypes::None		=> return write!(f, ""),
			MonsterTypes::Normal	=> return write!(f, "Normal"),
			MonsterTypes::Fire		=> return write!(f, "Fire"),
			MonsterTypes::Water		=> return write!(f, "Water"),
			MonsterTypes::Grass		=> return write!(f, "Grass"),
		}
    }
}

/// The attacks a monster can use in battle.
#[derive(Clone)]
pub enum MonsterAttacks {
	None,

	Tackle,
	Scratch,

	Growl,
	Leer,

	Leafage,
	Ember,
	Aquajet,
}
impl Display for MonsterAttacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
			MonsterAttacks::None	=> return write!(f, ""),
			MonsterAttacks::Tackle	=> return write!(f, "tackle"),
			MonsterAttacks::Scratch	=> return write!(f, "scratch"),
			MonsterAttacks::Growl	=> return write!(f, "growl"),
			MonsterAttacks::Leer	=> return write!(f, "leer"),
			MonsterAttacks::Leafage	=> return write!(f, "leafage"),
			MonsterAttacks::Ember	=> return write!(f, "ember"),
			MonsterAttacks::Aquajet	=> return write!(f, "aquajet"),
		}
    }
}

/// The abilities that monster possess.
pub enum MonsterAbilities {
	None,
}

/// The various condtions that a monster can have.
pub enum MonsterConditions {
	None,
	Burned{ stacks: i32, },
}

/// The experience needed to level up.
#[derive(Clone, Copy)]
pub enum MonsterGrowthRate {
	Fast,
	MediumFast,
	MediumSlow,
	Slow,
	Erratic,
	Fluctuating,
}


//= Structures

/// The structure of a monster team.
pub struct MonsterTeam(pub [Option<Monster>;4]);
impl Display for MonsterTeam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut monsters = ["".to_string(),"".to_string(),"".to_string(),"".to_string()];
		
		for i in 0..4 {
			if self.0[i].is_some() { monsters[i] = self.0[i].clone().unwrap().to_string() }
		}
		return write!(
			f,
			"[{}, {}, {}, {}]\n",
			monsters[0],
			monsters[1],
			monsters[2],
			monsters[3],
		);
    }
}

/// The structure of a monster.
#[derive(Clone)]
pub struct Monster {
	species: MonsterSpecies,
	types: [MonsterTypes; 2],

	nickname: String,

	health: [i32;2],
	stamina: [i32;2],

	physicalAttack: i32,
	physicalDefense: i32,
	specialAttack: i32,
	specialDefense: i32,
	speed: i32,

	statChanges: [u8;5],
	flinch: bool, //???

	experience: i32,
	level: i32,
	growthRate: MonsterGrowthRate,

	attacks: [MonsterAttacks;4],
}
impl Display for Monster {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(
			f,
			"{}-{}",
			self.species,
			self.nickname,
		);
		//	"{}-{} [{},{}]\n{}\t{}\n{}\t{}\n{}\n[{},{},{},{}]",
		//	self.species,
		//	self.nickname,
		//	self.types[0],self.types[1],
		//	self.physicalAttack,self.specialAttack,
		//	self.physicalDefense,self.specialDefense,
		//	self.speed,
		//	self.attacks[0],self.attacks[1],self.attacks[2],self.attacks[3],
		//);
    }
}


//= Procedures

impl MonsterTeam {

	/// Create new Empty team
	pub fn new() -> Self {
		return MonsterTeam([None,None,None,None])
	}

	/// Removes member by index
	pub fn remove_member_index(&mut self, index: usize) -> Option<Monster> {
		if self.0[index].is_none() {
			debug::log("[ERROR] - Attempted to remove monster that doesn't exist.");
			return None;
		}
		
		let monster = self.0[index].clone();
		self.shift_down(index);
		return monster;
	}

	/// Add member to team
	pub fn add_member(&mut self, monster: Monster) -> bool {
		for i in 0..4 {
			if self.0[i].is_none() {
				self.0[i] = Some(monster);
				return true;
			}
		}
		//* */ Can't add member
		return false;
	}

	/// Shifts everything down
	pub fn shift_down(&mut self, index: usize) {
		if index != 3 {
			if self.0[index+1].is_some() {
				self.0[index] = self.0[index+1].clone();
				self.0[index+1] = None;
			}

			return self.shift_down(index+1)
		}
	}

	/// Count the number of monsters on team
	pub fn number_of_monsters(&self) -> i32 {
		let mut count = 0;
		for i in self.0.iter() {
			if !i.is_none() { count += 1; }
		}
		return count;
	}

}

impl Monster {

	/// Create a new monster only using species and level.
	pub fn new(species: MonsterSpecies, level: i32) -> Self {
		let mut result = Monster {
			species,
			types: [MonsterTypes::None,MonsterTypes::None],

			nickname: "".to_string(),

			health: [0,0],
			stamina: [0,0],

			physicalAttack: 0,
			physicalDefense: 0,
			specialAttack: 0,
			specialDefense: 0,
			speed: 0,

			statChanges: [0,0,0,0,0],
			flinch: false,

			experience: 0,
			level,
			growthRate: MonsterGrowthRate::Fast,

			//TODO Generate attacks from their attack list and level.
			attacks: [MonsterAttacks::None,MonsterAttacks::None,MonsterAttacks::None,MonsterAttacks::None],
		};

		result.generate_stats();

		return result;
	}

	/// Generate monster stats from a clean monster.
	pub fn generate_stats(&mut self) {
		match self.species {
			MonsterSpecies::Mon152 => {
				self.types = [MonsterTypes::Grass, MonsterTypes::None];

				self.calculate_stats();

				self.growthRate = MonsterGrowthRate::MediumSlow;
				self.experience = experience_from_level(self.level, MonsterGrowthRate::MediumSlow);

				// TODO
				self.attacks = [
					MonsterAttacks::Tackle,
					MonsterAttacks::Growl,
					MonsterAttacks::Leafage,
					MonsterAttacks::None,
				]
			}
			MonsterSpecies::Mon155 => {
				self.types = [MonsterTypes::Fire, MonsterTypes::None];

				self.calculate_stats();

				self.growthRate = MonsterGrowthRate::MediumSlow;
				self.experience = experience_from_level(self.level, MonsterGrowthRate::MediumSlow);

				// TODO
				self.attacks = [
					MonsterAttacks::Tackle,
					MonsterAttacks::Leer,
					MonsterAttacks::Ember,
					MonsterAttacks::None,
				]
			}
			MonsterSpecies::Mon158 => {
				self.types = [MonsterTypes::Water, MonsterTypes::None];

				self.calculate_stats();

				self.growthRate = MonsterGrowthRate::MediumSlow;
				self.experience = experience_from_level(self.level, MonsterGrowthRate::MediumSlow);

				// TODO
				self.attacks = [
					MonsterAttacks::Scratch,
					MonsterAttacks::Leer,
					MonsterAttacks::Aquajet,
					MonsterAttacks::None,
				]
			}
		}
	}

	/// Calculate the stats of a monster
	pub fn calculate_stats(&mut self) {
		match self.species {
			MonsterSpecies::Mon152 => {
				let health: i32 = stat_calculation_a(50, self.level);
				let hpdiff = health - self.health[1];
				self.health[0] += hpdiff;
				self.health[1] = health;

				let stamina: i32 = stat_calculation_a(35, self.level);
				let stdiff = stamina - self.stamina[1];
				self.stamina[0] += stdiff;
				self.stamina[1] = stamina;

				self.physicalAttack		= stat_calculation_b(45, self.level);
				self.physicalDefense	= stat_calculation_b(65, self.level);
				self.specialAttack		= stat_calculation_b(45, self.level);
				self.specialDefense		= stat_calculation_b(65, self.level);
				self.speed				= stat_calculation_b(45, self.level);
			}
			MonsterSpecies::Mon155 => {
				let health: i32 = stat_calculation_a(35, self.level);
				let hpdiff = health - self.health[1];
				self.health[0] += hpdiff;
				self.health[1] = health;

				let stamina: i32 = stat_calculation_a(55, self.level);
				let stdiff = stamina - self.stamina[1];
				self.stamina[0] += stdiff;
				self.stamina[1] = stamina;

				self.physicalAttack		= stat_calculation_b(55, self.level);
				self.physicalDefense	= stat_calculation_b(40, self.level);
				self.specialAttack		= stat_calculation_b(60, self.level);
				self.specialDefense		= stat_calculation_b(40, self.level);
				self.speed				= stat_calculation_b(65, self.level);
			}
			MonsterSpecies::Mon158 => {
				let health: i32 = stat_calculation_a(35, self.level);
				let hpdiff = health - self.health[1];
				self.health[0] += hpdiff;
				self.health[1] = health;

				let stamina: i32 = stat_calculation_a(55, self.level);
				let stdiff = stamina - self.stamina[1];
				self.stamina[0] += stdiff;
				self.stamina[1] = stamina;

				self.physicalAttack		= stat_calculation_b(55, self.level);
				self.physicalDefense	= stat_calculation_b(40, self.level);
				self.specialAttack		= stat_calculation_b(60, self.level);
				self.specialDefense		= stat_calculation_b(40, self.level);
				self.speed				= stat_calculation_b(65, self.level);
			}
		}
	}

	/// Give a monster experience and check if it levels up.
	pub fn give_experience(&mut self, experience: i32) -> bool {
		self.experience += experience;

		return self.check_for_level();
	}

	/// Checks if the current amount of experience would level up the monster.
	pub fn check_for_level(&self) -> bool {
		return self.experience >= experience_from_level(self.level + 1, self.growthRate);
	}
}

/// Calculates health or stamina.
//TODO IVs, EVs, and natures
pub fn stat_calculation_a(baseStat: i32, level: i32) -> i32 {
	return (((2.0 * baseStat as f32) * level as f32) / 100.0).floor() as i32 + level + 10;
}
/// Calculates the stats other than health and stamina
pub fn stat_calculation_b(baseStat: i32, level: i32) -> i32 {
	return (((((2.0 * baseStat as f32) * level as f32) / 100.0).floor() + 5.0) * 1.0).floor() as i32;
}

/// Calculates the amount of experience at a minimum a monster can have at input level.
pub fn experience_from_level(level: i32, growth: MonsterGrowthRate) -> i32 {
	match growth {
		MonsterGrowthRate::Fast =>			return ((4.0 * (level as f32).powi(3)) / 5.0) as i32,
		MonsterGrowthRate::MediumFast => 	return (level as f32).powi(3) as i32,
		MonsterGrowthRate::MediumSlow => 	return ((1.2 * (level as f32).powi(3)) - (15.0 * (level as f32).powi(2)) + (100.0 * level as f32) - 140.0) as i32,
		MonsterGrowthRate::Slow =>			return ((5.0 * (level as f32).powi(3)) / 4.0) as i32,
		MonsterGrowthRate::Erratic => {
			if level < 50 {
				return (((level as f32).powi(3) * (100.0 - (level as f32))) / 50.0) as i32;
			}
			if level >= 50 && level < 68 {
				return (((level as f32).powi(3) * (150.0 - (level as f32))) / 100.0) as i32;
			}
			if level >= 68 && level < 98 {
				return (((level as f32) * ((1911.0 - (10.0 * (level as f32))) / 3.0).floor()) / 500.0) as i32;
			}
			if level >= 98 && level <= 100 {
				return (((level as f32).powi(3) * (160.0 - (level as f32))) / 100.0) as i32;
			}
			return 0;
		}
		MonsterGrowthRate::Fluctuating => {
			if level < 15 {
				return (((level as f32).powi(3) * ((((level as f32) + 1.0) / 3.0).floor() + 24.0)) / 50.0) as i32;
			}
			if level >= 15 && level < 36 {
				return (((level as f32).powi(3) * ((level as f32) + 14.0)) / 50.0) as i32;
			}
			if level >= 36 && level <= 100 {
				return (((level as f32).powi(3) * (((level as f32) / 2.0).floor() + 32.0)) / 50.0) as i32;
			}
			return 0;
		}
	}
}