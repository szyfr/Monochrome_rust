

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports


//= Enumerations


//= Structures
pub struct Event{

}
pub struct EntityEvent{
	pub conditions : Vec<Conditions>,
	pub id : String,
}
pub union Conditions {
	pub int	: i32,
	pub bl	: bool,
}


//= Procedures