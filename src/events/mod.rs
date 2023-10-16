

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
pub mod conditionals;
pub mod event_handler;
use crate::overworld::Direction;


//= Enumerations

/// Event types
pub enum EventChain{
	/// Test / Debug Event
	Test{ text: String },
	
	/// Textbox display Event
	Text{ text: String },

	/// Teleport unit Event
	Warp{
		entityID:	String,
		position:	[i32;3],
		direction:	Direction,
		doMove:		bool,
	
	},
}


//= Structures

/// Basic structure for all events
pub struct Event{
	pub chain : Vec<EventChain>,
}


//= Procedures