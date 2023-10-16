

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Enumerations
use std::collections::HashMap;
use super::conditionals::Condition;



//= Structures
pub struct EventHandler{
	pub currentEvent : String,
	//pub textbox : textbox:Box,
	pub currentChain : i32,

	pub internal : i32,

	pub eventVariables : HashMap<String, Condition>,

	pub playerName : String,
	pub playerPronouns : [String; 3],
	pub rivalName : String,
}


//= Procedures

/// Create new eventHandler
pub fn create() -> EventHandler {
	return EventHandler{
		currentEvent:	"".to_string(),
		currentChain:	0,

		internal:		0,

		eventVariables: HashMap::new(),

		playerName:		"Mono".to_string(),
		playerPronouns: ["they".to_string(), "them".to_string(), "theirs".to_string()],
		rivalName:		"Chrome".to_string(),

	}
}