

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{data, raylib::{self, vectors::Vector3, rectangles::Rectangle}, overworld};


//= Enumerations


//= Structures

/// Basic animation structure
pub struct Animation {
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

/// Emote aniamtion structure
pub struct EmoteAnimation {
	pub emote: String,
	pub unitID: String,

	pub duration: i32,
	pub ticks: i32,
}


//= Procedures

/// Logic for animation
pub fn run( gamestate: &mut data::Gamestate, animation: Animation ) -> bool {
	if gamestate.eventHandler.animation.is_some() {
		if !same_anim(gamestate.eventHandler.animation.as_ref().unwrap().clone(), animation.clone()) {
			gamestate.eventHandler.animation = None;
			run(gamestate, animation);
			return false;
		}
		gamestate.eventHandler.animation.as_mut().unwrap().ticks += 1;
		if gamestate.eventHandler.animation.as_mut().unwrap().ticks >= gamestate.eventHandler.animation.as_mut().unwrap().ticksPerFrame {
			gamestate.eventHandler.animation.as_mut().unwrap().ticks = 0;
			gamestate.eventHandler.animation.as_mut().unwrap().frame += 1;
			if gamestate.eventHandler.animation.as_mut().unwrap().frame >= gamestate.eventHandler.animation.as_mut().unwrap().order.len() as i32 {
				gamestate.eventHandler.animation.as_mut().unwrap().frame -= 1;
				if !gamestate.eventHandler.animation.as_mut().unwrap().hold {
					gamestate.eventHandler.animation = None;
				}
				return true;
			}
		}
	} else {
		gamestate.eventHandler.animation = Some(animation);
	}

	return false
}

/// Draw animations
pub fn draw( gamestate : &mut data::Gamestate ) {
	if gamestate.eventHandler.animation.is_none() { return }

	let frame = gamestate.eventHandler.animation.as_ref().unwrap().frame;

	let mut animName = "ui_animation_".to_string();
	animName += &gamestate.eventHandler.animation.as_mut().unwrap().currentAnimation.to_string();
	animName += "_";
	animName += &gamestate.eventHandler.animation.as_mut().unwrap().order[frame as usize].to_string();
	let texture = *gamestate.graphics.textures.get(&animName).unwrap();
	
	texture.draw_pro(
		Rectangle { x: 0.0, y: 0.0, width: texture.width as f32, height: texture.height as f32 },
		Rectangle { x: 0.0, y: 0.0, width: data::get_screenwidth() as f32, height: data::get_screenheight() as f32 },
		0.0,
	);
}

/// Draw Emotes
pub fn draw_emotes( gamestate : &mut data::Gamestate ) {
	let mut remove: Vec<usize> = Vec::new();
	
	for i in 0..gamestate.eventHandler.emotes.len() {
		gamestate.eventHandler.emotes[i].ticks += 1;
		if gamestate.eventHandler.emotes[i].ticks >= gamestate.eventHandler.emotes[i].duration {
			remove.push(i);
		} else {
			//* Get model and skin it */
			let model = &gamestate.graphics.models["unit"];
			let textureName = &("emote_".to_string() + &gamestate.eventHandler.emotes[i].emote);
			raylib::set_material_texture(model.materials, raylib_ffi::enums::MaterialMapIndex::Albedo, gamestate.graphics.textures[textureName].into());

			//* Get unit */
			let unit: Option<overworld::Unit>;
			if gamestate.eventHandler.emotes[i].unitID == "player" { unit = Some(gamestate.player.unit.clone()); }
			else { unit = Some(gamestate.worldData.unitMap[&gamestate.eventHandler.emotes[i].unitID].clone()); }

			//* Draw */
			let mut offset = 0.5;
			if gamestate.eventHandler.emotes[i].ticks <= 10 {
				offset += gamestate.eventHandler.emotes[i].ticks as f32 / 9.0;
			} else { offset = 1.0; }
			model.draw_ex(
				Vector3{
					x: unit.as_ref().unwrap().position.x,
					y: (unit.as_ref().unwrap().position.y/2.0) + offset,
					z: unit.as_ref().unwrap().position.z,
				},
				Vector3{x:0.0,y:1.0,z:0.0},
				-gamestate.camera.rotation,
				Vector3{x:0.8,y:0.8,z:0.8},
				raylib_ffi::colors::WHITE,
			);
			//raylib::draw_model_ex(
			//	model,
			//	raylib_ffi::Vector3{x: unit.as_ref().unwrap().position.x, y: (unit.as_ref().unwrap().position.y/2.0) + offset, z: unit.as_ref().unwrap().position.z},
			//	raylib_ffi::Vector3{x:0.0,y:1.0,z:0.0},
			//	-gamestate.camera.rotation,
			//	raylib_ffi::Vector3{x:0.8,y:0.8,z:0.8},
			//	raylib_ffi::colors::WHITE,
			//);
		}
	}

	for i in remove.iter() {
		gamestate.eventHandler.emotes.remove(*i);
	}
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