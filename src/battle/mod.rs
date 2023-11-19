

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::str::FromStr;
use crate::{monsters, data, settings};


//= Enumerations

///
pub enum Combatant {
	Empty,
	Trainer(Enemy),
	Wild(monsters::Monster),
	WildDouble([monsters::Monster;2]),
}

///
pub enum BattleType {
	Normal,
	Double,
}
impl FromStr for BattleType {
	type Err = ();
	fn from_str( input : &str ) -> Result<BattleType, Self::Err> {
		match input {
			"single"	=> Ok(BattleType::Normal),
			"double"	=> Ok(BattleType::Double),
			_			=> Err(()),
		}
	}
}


//= Structures

///
pub struct BattleData {
	pub started: bool,
	pub combatant: Combatant,
}

///
pub struct Battle {
	pub battleType: BattleType,
	pub trainerName: String,

	pub easyTeam: monsters::MonsterTeam,
	pub mediumTeam: monsters::MonsterTeam,
	pub hardTeam: monsters::MonsterTeam,
}

///
pub struct Enemy {
	pub name: String,
	pub team: monsters::MonsterTeam,
	pub batType: BattleType,
}


//= Procedures

impl BattleData {
	
	/// Create new battle structure
	pub fn init() -> Self {
		return BattleData{
			started:	false,
    		combatant:	Combatant::Empty,
		}
	}
	/// Clear battle structure to empty
	pub fn clear(&mut self) {
		self.started = false;
		self.combatant = Combatant::Empty;
	}

	/// Start battle
	pub fn start_tariner_battle(&mut self, battle: Battle) {
		self.started = true;

		let team: monsters::MonsterTeam;
		match data::get_difficulty() {
			settings::Difficulty::Easy => { team = battle.easyTeam; }
			settings::Difficulty::Medium => { team = battle.mediumTeam; }
			settings::Difficulty::Hard => { team = battle.hardTeam; }
		}
		self.combatant = Combatant::Trainer(
			Enemy {
				name:		battle.trainerName,
				team,
				batType:	battle.battleType,
			},
		);
	}

}

impl Enemy {
	
}