

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use std::fmt::Display;


//= Enumerations
#[derive(Copy, Clone, PartialEq)]
pub enum Condition{
	Integer(i32),
	Boolean(bool),
}
impl Display for Condition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match *self {
			Condition::Integer(_) => write!(f, "Int:{}\n", *self),
			Condition::Boolean(_) => write!(f, "Bool:{}\n", *self),
		}
	}
}


//= Structures
pub struct EntityEvent{
	pub val : Vec<Condition>,
	pub key : String,
}


//= Procedures