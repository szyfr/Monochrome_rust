

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::fmt::Display;


//= Enumerations
#[derive(Clone, PartialEq)]
pub enum Condition{
	Integer(i32),
	Boolean(bool),
	String(String),
}
impl Display for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Condition::Integer(i)		=> write!(f, "Int({})", i),
			Condition::Boolean(b)	=> write!(f, "Bool({})", b),
			Condition::String(s)	=> write!(f, "String({})",s),
		}
	}
}


//= Structures
pub struct EntityEvent{
	pub val : Vec<Condition>,
	pub key : String,
}


//= Procedures

impl Condition {
	///
	pub fn as_integer(&self) -> i32 {
		match self {
			Condition::Integer(int) => { return *int; }
			_ => { return 0; }
		}
	}

	///
	pub fn as_bool(&self) -> bool {
		match self {
			Condition::Boolean(bool) => { return *bool; }
			_ => { return false; }
		}
	}

	///
	pub fn as_string(&self) -> String {
		match self {
			Condition::String(str) => { return str.to_string(); }
			_ => { return "".to_string(); }
		}
	}
}