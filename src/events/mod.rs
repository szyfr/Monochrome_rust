

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::{collections::HashMap, mem::ManuallyDrop};

use crate::overworld::Direction;


//= Enumerations

///
#[derive(Clone)]
pub enum ConditionType {
	Null,
	Integer,
	Boolean,
}


//= Structures
pub struct EventHandler{
	pub currentEvent : String,
	//pub textbox : textbox:Box,
	pub currentChain : i32,

	pub internal : i32,

	pub eventVariables : HashMap<String, ConditionsType>,

	pub playerName : String,
	pub playerPronouns : [String; 3],
	pub rivalName : String,
}
#[derive(Clone)]
pub struct EntityEvent{
	pub val : Vec<Condition>,
	pub key : String,
}
impl EntityEvent {
	pub fn new() -> EntityEvent {
		return EntityEvent {val: Vec::new(), key: "".to_string()};
	}
}

#[derive(Clone)]
pub struct Condition {
	pub value		: ConditionsType,
	pub condType	: ConditionType,
	pub key			: String,
}
impl Condition {
	pub fn new() -> Condition {
		return Condition {value: ConditionsType{int:0}, condType: ConditionType::Integer, key: "".to_string()};
	}
}

#[derive(Copy, Clone)]
pub union ConditionsType {
	pub int	: i32,
	pub bl	: bool,
}

pub struct Event{
	pub chain : Vec<EventChain>,
}
pub union EventChain{
	pub warp: ManuallyDrop<WarpEvent>,
	pub text: ManuallyDrop<TextEvent>,
}
pub struct WarpEvent{
	pub entityID	: String,
	pub position	: [i32;3],
	pub direction	: Direction,
	pub doMove		: bool,
}
pub struct TextEvent{
	pub text : String,
}


//= Procedures
pub fn create_eventhandler() -> EventHandler {
	return EventHandler{
		currentEvent: "".to_string(),
		currentChain: 0,
		internal: 0,
		eventVariables: HashMap::new(),
		playerName: "".to_string(),
		playerPronouns: ["".to_string(),"".to_string(),"".to_string()],
		rivalName: "".to_string(),
	};
}
pub fn create_empty_entityevents() -> [Option<EntityEvent>; 5] {
	return [
		None,
		None,
		None,
		None,
		None,
	];
}
pub fn create_empty_conditions() -> [Option<Condition>; 5] {
	return [
		None,
		None,
		None,
		None,
		None,
	];
}