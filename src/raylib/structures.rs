

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::enums;
use std::{ffi::c_void, borrow::BorrowMut};


//= Structures

/// Vector2 type
#[derive(Copy, Clone)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

/// Vector3 type
#[derive(Copy, Clone)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

/// Vector4 type
#[derive(Copy, Clone)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

/// Quaternion type
#[derive(Copy, Clone)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

/// Matrix, 4x4 components, column major, OpenGL style, right handed
pub struct Matrix {
	pub m0: f32, pub m4: f32, pub  m8: f32, pub m12: f32, //* Matrix first row  (4 components)
	pub m1: f32, pub m5: f32, pub  m9: f32, pub m13: f32, //* Matrix second row (4 components)
	pub m2: f32, pub m6: f32, pub m10: f32, pub m14: f32, //* Matrix third row  (4 components)
	pub m3: f32, pub m7: f32, pub m11: f32, pub m15: f32, //* Matrix fourth row (4 components)
}

/// Rectangle type
pub struct Rectangle {
	pub x: f32,			//* Rectangle top-left corner position x
	pub y: f32,			//* Rectangle top-left corner position y
	pub width: f32,		//* Rectangle width
	pub height: f32,	//* Rectangle height
}

/// Color, 4 components, R8G8B8A8 (32bit)
pub struct Color {
	pub r: u8,		//* Color red value
	pub g: u8,		//* Color green value
	pub b: u8,		//* Color blue value
	pub a: u8,		//* Color alpha value
}

/// Image type, bpp always RGBA (32bit)
pub struct Image {
	pub data:		*mut c_void,		// Image raw data
	pub width:		i32,				// Image base width
	pub height:		i32,				// Image base height
	pub mipmaps:	i32,				// Mipmap levels, 1 by default
	pub format:		enums::PixelFormat,	// Data format (PixelFormat type)
}

/// Texture type
#[derive(Copy, Clone)]
pub struct Texture {
	pub id:			u32,				// OpenGL texture id
	pub width:		i32,				// Texture base width
	pub height:		i32,				// Texture base height
	pub mipmaps:	i32,				// Mipmap levels, 1 by default
	pub format:		enums::PixelFormat,	// Data format (PixelFormat type)
	pub origin:		Vector2,			// Origin point
	pub tint:		raylib_ffi::Color,	// Color
}
/// Texture2D type, same as Texture
pub struct Texture2D (Texture);
/// TextureCubemap type, same as Texture
pub struct TextureCubeMap (Texture);


//= Procedures

impl Vector2 {

	/// Creating a zeroed V3
	pub fn zero() -> Vector2 {
		return Vector2 { x: 0.0, y: 0.0 }
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Vector2 {
		return raylib_ffi::Vector2 { x: self.x, y: self.y };
	}

}

impl Vector3 {

	/// Creating a zeroed V3
	pub fn zero() -> Vector3 {
		return Vector3 { x: 0.0, y: 0.0, z: 0.0 }
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Vector3 {
		return raylib_ffi::Vector3 { x: self.x, y: self.y, z: self.z };
	}

}

impl Rectangle {

	/// Create a rectangle using an index in a spritesheet
	pub fn tex_rect(index: i32, size: [i32;2]) -> Rectangle {
		return Rectangle {
			x:		(index * size[0]) as f32,
			y:		0.0,
			width: 	size[0] as f32,
			height:	size[1] as f32,
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Rectangle {
		return raylib_ffi::Rectangle{
			x: 		self.x,
			y:		self.y,
			width:	self.width,
			height:	self.height,
		};
	}

}

impl Image {

	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Image::from_ffi(raylib_ffi::LoadImage(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Image
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadImage(self.to_ffi());
		}
	}

	/// Create duplicate of Image
	pub fn copy(&self) -> Self {
		unsafe {
			return Image::from_ffi(raylib_ffi::ImageCopy(self.to_ffi()));
		}
	}
	/// Create duplicate of portion of Image
	pub fn from_image(&self, rec: Rectangle) -> Self {
		unsafe {
			return Image::from_ffi(raylib_ffi::ImageFromImage(self.to_ffi(), rec.to_ffi()));
		}
	}

	/// Resize image using nerarest neighbor
	pub fn resize_nn(&self, scale: i32) -> Self {
		unsafe {
			let mut ffiImg = self.to_ffi();
			let mutImage: &mut raylib_ffi::Image = ffiImg.borrow_mut();
			raylib_ffi::ImageResizeNN(mutImage, self.width * scale, self.height * scale);

			return Image::from_ffi(*mutImage);
		}
	}

	/// Loading a texture from the image
	pub fn load_texture(&self) -> Texture {
		unsafe {
			return Texture::from_ffi(raylib_ffi::LoadTextureFromImage(self.to_ffi()));
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Image {
		return raylib_ffi::Image{
			data:		self.data,
			width:		self.width,
			height:		self.height,
			mipmaps:	self.mipmaps,
			format:		self.format as i32,
		}
	}
	/// Converting to Image
	pub fn from_ffi(image: raylib_ffi::Image) -> Self {
		return Image {
			data:		image.data,
			width:		image.width,
			height:		image.height,
			mipmaps:	image.mipmaps,
			format:		enums::PixelFormat::from_i32(image.format),
		}
	}

}

impl Texture {

	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return Texture::from_ffi(raylib_ffi::LoadTexture(raylib_ffi::rl_str!(fileName)));
		}
	}
	/// Unloading Texture
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadTexture(self.to_ffi());
		}
	}

	/// Draw texture using raylib_ffi::DrawTexture
	pub fn draw(&self, posX: i32, posY: i32) -> Self {
		unsafe {
			raylib_ffi::DrawTexture(self.to_ffi(), posX, posY, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureV
	pub fn draw_v(&self, position: Vector2) -> Self {
		unsafe {
			raylib_ffi::DrawTextureV(self.to_ffi(), position.to_ffi(), self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureEX
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32) -> Self {
		unsafe {
			raylib_ffi::DrawTextureEx(self.to_ffi(), position.to_ffi(), rotation, scale, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureRec
	pub fn draw_rec(&self, source: Rectangle, position: Vector2) -> Self {
		unsafe {
			raylib_ffi::DrawTextureRec(self.to_ffi(), source.to_ffi(), position.to_ffi(), self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTexturePro
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, rotation: f32) -> Self {
		unsafe {
			raylib_ffi::DrawTexturePro(self.to_ffi(), source.to_ffi(), dest.to_ffi(), self.origin.to_ffi(), rotation, self.tint);

			return self.clone();
		}
	}
	/// Draw texture using raylib_ffi::DrawTextureNPatch
	pub fn draw_npatch(&self, dest: Rectangle, rotation: f32) -> Self {
		unsafe {
			let nPatchInfo = raylib_ffi::NPatchInfo {
				source: raylib_ffi::Rectangle {
					x:		0.0,
					y:		0.0,
					width:	self.width as f32,
					height:	self.height as f32,
				},
				left:	self.width / 3,
				top:	self.height / 3,
				right:	self.width / 3,
				bottom:	self.height / 3,
				layout:	0,
			};
			raylib_ffi::DrawTextureNPatch(self.to_ffi(), nPatchInfo, dest.to_ffi(), self.origin.to_ffi(), rotation, self.tint);

			return self.clone();
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Texture {
		return raylib_ffi::Texture{
			id:			self.id,
			width:		self.width,
			height:		self.height,
			mipmaps:	self.mipmaps,
			format:		self.format as i32,
		}
	}
	/// Converting to Texture
	pub fn from_ffi(texture: raylib_ffi::Texture) -> Self {
		return Texture {
			id:			texture.id,
			width:		texture.width,
			height:		texture.height,
			mipmaps:	texture.mipmaps,
			format:		enums::PixelFormat::from_i32(texture.format),
			origin:		Vector2 { x: 0.0, y: 0.0 },
			tint:		raylib_ffi::colors::WHITE,
		}
	}
}
