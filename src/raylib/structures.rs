

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use super::enums;
use std::ffi::c_void;


//= Structures

/// Vector2 type
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

/// Vector3 type
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

/// Vector4 type
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

/// Quaternion type
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
pub struct Texture {
	pub id:			u32,				// OpenGL texture id
	pub width:		i32,				// Texture base width
	pub height:		i32,				// Texture base height
	pub mipmaps:	i32,				// Mipmap levels, 1 by default
	pub format:	enums::PixelFormat,		// Data format (PixelFormat type)
}
/// Texture2D type, same as Texture
pub struct Texture2D (Texture);
/// TextureCubemap type, same as Texture
pub struct TextureCubeMap (Texture);

//


//= Procedures

impl Image {

	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return image_from_ffi(raylib_ffi::LoadImage(raylib_ffi::rl_str!(fileName)));
		}
	}

	/// Unloading Image
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadImage(self.to_ffi());
		}
	}

	/// Loading a texture from the image
	pub fn load_texture(&self) -> Texture {
		unsafe {
			return texture_from_ffi(raylib_ffi::LoadTextureFromImage(self.to_ffi()));
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
}

impl Texture {

	/// Loading Image
	pub fn load(fileName: &str) -> Self {
		unsafe {
			return texture_from_ffi(raylib_ffi::LoadTexture(raylib_ffi::rl_str!(fileName)));
		}
	}

	/// Unloading Texture
	pub fn unload(&self) {
		unsafe {
			raylib_ffi::UnloadTexture(self.to_ffi());
		}
	}

	/// Draw texture to screen
	pub fn draw(&self, posX: i32, posY: i32, tint: raylib_ffi::Color) {
		unsafe {
			raylib_ffi::DrawTexture(self.to_ffi(), posX, posY, tint);
		}
	}

	/// Converting to raylib_ffi version
	pub fn to_ffi(&self) -> raylib_ffi::Texture {
		print!("{}\n{}x{}\n{} - {}",self.id, self.width, self.height, self.mipmaps, self.format as i32);
		return raylib_ffi::Texture{
			id:			self.id,
			width:		self.width,
			height:		self.height,
			mipmaps:	self.mipmaps,
			format:		self.format as i32,
		}
	}
}

///
pub fn texture_from_ffi(texture: raylib_ffi::Texture) -> Texture {
	print!("{}\n{}x{}\n{} - {}",texture.id, texture.width, texture.height, texture.mipmaps, texture.format);
	return Texture {
		id:			texture.id,
		width:		texture.width,
		height:		texture.height,
		mipmaps:	texture.mipmaps,
		format:		enums::PixelFormat::from_i32(texture.format),
	}
}

///
pub fn image_from_ffi(image: raylib_ffi::Image) -> Image {
	return Image {
		data:		image.data,
		width:		image.width,
		height:		image.height,
		mipmaps:	image.mipmaps,
		format:		enums::PixelFormat::from_i32(image.format),
	}
}