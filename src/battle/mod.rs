

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::monsters;


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
	},
	/// Data for a Double Battle
	Double{
		trainerName: [String;2],
		singleTrainer: bool,
	
		easyTeam:	[monsters::MonsterTeam;2],
		mediumTeam:	[monsters::MonsterTeam;2],
		hardTeam:	[monsters::MonsterTeam;2],
	},
	/// Data for a Wild Battle
	Wild{
		monster: monsters::Monster,
	},
}


//= Structures

/// The general battle data struct
pub struct BattleData {
	//* Battle variables */
	pub started: bool,

	//* Data */
	pub battleType: BattleType,
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
		}
	}

	/// Clear battle structure to empty
	pub fn clear(&mut self) {
		self.started = false;
		self.battleType = BattleType::Empty;
	}

	/// Start battle
	pub fn start_trainer_battle(&mut self, battle: BattleType) {
		self.started = true;

		self.battleType = battle;
	}

	//
	pub fn update(&self) {
		
	}

}
