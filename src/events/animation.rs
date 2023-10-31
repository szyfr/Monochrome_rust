

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{data, raylib};


//= Enumerations


//= Structures
pub struct Animation{
	pub currentAnimation: String,
	pub order: Vec<i32>,
	pub ticksPerFrame: i32,
	pub hold: bool,

	pub frame: i32,
	pub ticks: i32,
}
impl Clone for Animation {
	fn clone(&self) -> Self {
		Self {
			currentAnimation: self.currentAnimation.clone(),
			order: self.order.clone(),
			ticksPerFrame: self.ticksPerFrame.clone(),
			hold: self.hold.clone(),
			frame: self.frame.clone(),
			ticks: self.ticks.clone(),
		}
	}
}


//= Procedures

/// Logic for animation
pub fn run( gamestate: &mut data::Gamestate, animation: Animation ) -> bool {
	if gamestate.worldData.eventHandler.animation.is_some() {
		if !same_anim(gamestate.worldData.eventHandler.animation.as_ref().unwrap().clone(), animation.clone()) {
			gamestate.worldData.eventHandler.animation = None;
			run(gamestate, animation);
			return false;
		}
		gamestate.worldData.eventHandler.animation.as_mut().unwrap().ticks += 1;
		if gamestate.worldData.eventHandler.animation.as_mut().unwrap().ticks >= gamestate.worldData.eventHandler.animation.as_mut().unwrap().ticksPerFrame {
			gamestate.worldData.eventHandler.animation.as_mut().unwrap().ticks = 0;
			gamestate.worldData.eventHandler.animation.as_mut().unwrap().frame += 1;
			if gamestate.worldData.eventHandler.animation.as_mut().unwrap().frame >= gamestate.worldData.eventHandler.animation.as_mut().unwrap().order.len() as i32 {
				gamestate.worldData.eventHandler.animation.as_mut().unwrap().frame -= 1;
				if !gamestate.worldData.eventHandler.animation.as_mut().unwrap().hold {
					gamestate.worldData.eventHandler.animation = None;
				}
				return true;
			}
		}
	} else {
		gamestate.worldData.eventHandler.animation = Some(animation);
	}

	return false
}

/// Draw animations
pub fn draw( gamestate : &mut data::Gamestate ) {
	if gamestate.worldData.eventHandler.animation.is_none() { return }

	let frame = gamestate.worldData.eventHandler.animation.as_ref().unwrap().frame;

	let mut animName = "ui_animation_".to_string();
	animName += &gamestate.worldData.eventHandler.animation.as_mut().unwrap().currentAnimation.to_string();
	animName += "_";
	animName += &gamestate.worldData.eventHandler.animation.as_mut().unwrap().order[frame as usize].to_string();
	print!("{}\n",animName);
	let texture = *gamestate.textures.get(&animName).unwrap();
	
	raylib::draw_texture_pro(
		texture, 
		raylib_ffi::Rectangle { x: 0.0, y: 0.0, width: texture.width as f32, height: texture.height as f32 },
		raylib_ffi::Rectangle { x: 0.0, y: 0.0, width: data::get_screenwidth() as f32, height: data::get_screenheight() as f32 },
		raylib_ffi::Vector2 { x: 0.0, y: 0.0 },
		0.0,
		raylib_ffi::colors::WHITE,
	);
}

fn same_anim( a1: Animation, a2: Animation ) -> bool {
	let mut result = true;

	if a1.order.len() >= a2.order.len() {
		for i in 0..a1.order.len() {
			if a1.order[i] != a2.order[i] { result = false; }
		}
	} else {
		for i in 0..a1.order.len() {
			if a2.order[i] != a1.order[i] { result = false; }
		}
	}

	return result;
}