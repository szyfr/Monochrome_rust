

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