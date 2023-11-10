

//= Allows
#![allow(non_snake_case)]
#![allow(dead_code)]


//= Imports
use crate::{data, camera::Camera};
use raylib_ffi::Vector3;


//= Enumeration
pub enum ShaderLocationIndex {
	ShaderLocVertexPosition = 0, // Shader location: vertex attribute: position
	ShaderLocVertexTexcoord01,   // Shader location: vertex attribute: texcoord01
	ShaderLocVertexTexcoord02,   // Shader location: vertex attribute: texcoord02
	ShaderLocVertexNormal,       // Shader location: vertex attribute: normal
	ShaderLocVertexTangent,      // Shader location: vertex attribute: tangent
	ShaderLocVertexColor,        // Shader location: vertex attribute: color
	ShaderLocMatrixMvp,          // Shader location: matrix uniform: model-view-projection
	ShaderLocMatrixView,         // Shader location: matrix uniform: view (camera transform)
	ShaderLocMatrixProjection,   // Shader location: matrix uniform: projection
	ShaderLocMatrixModel,        // Shader location: matrix uniform: model (transform)
	ShaderLocMatrixNormal,       // Shader location: matrix uniform: normal
	ShaderLocVectorView,         // Shader location: vector uniform: view
	ShaderLocColorDiffuse,       // Shader location: vector uniform: diffuse color
	ShaderLocColorSpecular,      // Shader location: vector uniform: specular color
	ShaderLocColorAmbient,       // Shader location: vector uniform: ambient color
	ShaderLocMapAlbedo,          // Shader location: sampler2d texture: albedo (same as: SHADER_LOC_MAP_DIFFUSE)
	ShaderLocMapMetalness,       // Shader location: sampler2d texture: metalness (same as: SHADER_LOC_MAP_SPECULAR)
	ShaderLocMapNormal,          // Shader location: sampler2d texture: normal
	ShaderLocMapRoughness,       // Shader location: sampler2d texture: roughness
	ShaderLocMapOcclusion,       // Shader location: sampler2d texture: occlusion
	ShaderLocMapEmission,        // Shader location: sampler2d texture: emission
	ShaderLocMapHeight,          // Shader location: sampler2d texture: height
	ShaderLocMapCubemap,         // Shader location: samplerCube texture: cubemap
	ShaderLocMapIrradiance,      // Shader location: samplerCube texture: irradiance
	ShaderLocMapPrefilter,       // Shader location: samplerCube texture: prefilter
	ShaderLocMapBrdf             // Shader location: sampler2d texture: brdf
}

pub enum ShaderUniformDataType {
	ShaderUniformFloat = 0,       // Shader uniform type: float
	ShaderUniformVec2,            // Shader uniform type: vec2 (2 float)
	ShaderUniformVec3,            // Shader uniform type: vec3 (3 float)
	ShaderUniformVec4,            // Shader uniform type: vec4 (4 float)
	ShaderUniformInt,             // Shader uniform type: int
	ShaderUniformIvec2,           // Shader uniform type: ivec2 (2 int)
	ShaderUniformIvec3,           // Shader uniform type: ivec3 (3 int)
	ShaderUniformIvec4,           // Shader uniform type: ivec4 (4 int)
	ShaderUniformSampler2d        // Shader uniform type: sampler2d
}

pub enum PixelFormat {
	PixelformatUncompressedGrayscale = 1,	// 8 bit per pixel (no alpha)
    PixelformatUncompressedGrayAlpha,		// 8*2 bpp (2 channels)
    PixelformatUncompressedR5g6b5,			// 16 bpp
    PixelformatUncompressedR8g8b8,			// 24 bpp
    PixelformatUncompressedR5g5b5a1,		// 16 bpp (1 bit alpha)
    PixelformatUncompressedR4g4b4a4,		// 16 bpp (4 bit alpha)
    PixelformatUncompressedR8g8b8a8,		// 32 bpp
    PixelformatUncompressedR32,				// 32 bpp (1 channel - float)
    PixelformatUncompressedR32g32b32,		// 32*3 bpp (3 channels - float)
    PixelformatUncompressedR32g32b32a32,	// 32*4 bpp (4 channels - float)
    PixelformatUncompressedR16,				// 16 bpp (1 channel - half float)
    PixelformatUncompressedR16g16b16,		// 16*3 bpp (3 channels - half float)
    PixelformatUncompressedR16g16b16a16,	// 16*4 bpp (4 channels - half float)
    PixelformatCompressedDxt1Rgb,			// 4 bpp (no alpha)
    PixelformatCompressedDxt1Rgba,			// 4 bpp (1 bit alpha)
    PixelformatCompressedDxt3Rgba,			// 8 bpp
    PixelformatCompressedDxt5Rgba,			// 8 bpp
    PixelformatCompressedEtc1Rgb,			// 4 bpp
    PixelformatCompressedEtc2Rgb,			// 4 bpp
    PixelformatCompressedEtc2EacRgba,		// 8 bpp
    PixelformatCompressedPvrtRgb,			// 4 bpp
    PixelformatCompressedPvrtRgba,			// 4 bpp
    PixelformatCompressedAstc4x4Rgba,		// 8 bpp
    PixelformatCompressedAstc8x8Rgba		// 2 bpp
}

pub enum RlFramebufferAttachType {
	RlAttachmentColorChannel0 = 0,	// Framebuffer attachment type: color 0
	RlAttachmentColorChannel1 = 1,	// Framebuffer attachment type: color 1
	RlAttachmentColorChannel2 = 2,	// Framebuffer attachment type: color 2
	RlAttachmentColorChannel3 = 3,	// Framebuffer attachment type: color 3
	RlAttachmentColorChannel4 = 4,	// Framebuffer attachment type: color 4
	RlAttachmentColorChannel5 = 5,	// Framebuffer attachment type: color 5
	RlAttachmentColorChannel6 = 6,	// Framebuffer attachment type: color 6
	RlAttachmentColorChannel7 = 7,	// Framebuffer attachment type: color 7
	RlAttachmentDepth = 100,		// Framebuffer attachment type: depth
	RlAttachmentStencil = 200,		// Framebuffer attachment type: stencil
}

pub enum RlFramebufferAttachTextureType {
	RlAttachmentCubemapPositiveX = 0,   // Framebuffer texture attachment type: cubemap, +X side
    RlAttachmentCubemapNegativeX = 1,   // Framebuffer texture attachment type: cubemap, -X side
    RlAttachmentCubemapPositiveY = 2,   // Framebuffer texture attachment type: cubemap, +Y side
    RlAttachmentCubemapNegativeY = 3,   // Framebuffer texture attachment type: cubemap, -Y side
    RlAttachmentCubemapPositiveZ = 4,   // Framebuffer texture attachment type: cubemap, +Z side
    RlAttachmentCubemapNegativeZ = 5,   // Framebuffer texture attachment type: cubemap, -Z side
    RlAttachmentTexture2d = 100,          // Framebuffer texture attachment type: texture2d
    RlAttachmentRenderbuffer = 200,       // Framebuffer texture attachment type: renderbuffer
}


//= Procedures
pub fn begin_drawing() {
	unsafe { raylib_ffi::BeginDrawing(); }
}
pub fn end_drawing() {
	unsafe { raylib_ffi::EndDrawing(); }
}

pub fn clear_background( color : raylib_ffi::Color ) {
	unsafe { raylib_ffi::ClearBackground(color); }
}

pub fn draw_text( text : *const std::os::raw::c_char, posX : i32, posY : i32, fontSize : i32, color : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawText(text, posX, posY, fontSize, color); }
}
pub fn draw_text_pro( font : raylib_ffi::Font, text : &str, position : raylib_ffi::Vector2, origin : raylib_ffi::Vector2, rotation : f32, fontSize : f32, spacing : f32, tint : raylib_ffi::Color ) {
	unsafe {
		raylib_ffi::DrawTextPro(
			font,
			raylib_ffi::rl_str!(text),
			position,
			origin,
			rotation,
			fontSize,
			spacing,
			tint,
		);
	}
}

pub fn window_should_close() -> bool {
	unsafe { return raylib_ffi::WindowShouldClose(); }
}

pub fn set_trace_log_level( logLevel : raylib_ffi::enums::TraceLogLevel ) {
	unsafe { raylib_ffi::SetTraceLogLevel(logLevel as i32); }
}

pub fn init_window( gamestate : &data::Gamestate ) {
	unsafe {
		raylib_ffi::InitWindow(
			data::SETTINGS.screenWidth,
			data::SETTINGS.screenHeight,
			raylib_ffi::rl_str!(gamestate.localization["title"]),
		);
	}
}
pub fn close_window() {
	unsafe { raylib_ffi::CloseWindow(); }
}
pub fn is_window_ready() -> bool {
	unsafe { return raylib_ffi::IsWindowReady(); }
}

pub fn set_target_fps( fps : i32 ) {
	unsafe { raylib_ffi::SetTargetFPS(fps); }
}
pub fn draw_fps( x : i32, y : i32 ) {
	unsafe { raylib_ffi::DrawFPS(x, y); }
}

pub fn set_exit_key( key : raylib_ffi::enums::KeyboardKey ) {
	unsafe { raylib_ffi::SetExitKey(key as i32); }
}

pub fn load_font( filename : &str ) -> raylib_ffi::Font {
	unsafe { return raylib_ffi::LoadFont(raylib_ffi::rl_str!(filename)); }
}
pub fn load_image( filename : &str ) -> raylib_ffi::Image {
	unsafe { return raylib_ffi::LoadImage(raylib_ffi::rl_str!(filename)); }
}
pub fn image_from_image( image : raylib_ffi::Image, rec : raylib_ffi::Rectangle ) -> raylib_ffi::Image {
	unsafe { return raylib_ffi::ImageFromImage(image, rec); }
}
pub fn image_copy( image : raylib_ffi::Image ) -> raylib_ffi::Image {
	unsafe { return raylib_ffi::ImageCopy(image); }
}
pub fn unload_image( image : raylib_ffi::Image ) {
	unsafe { raylib_ffi::UnloadImage(image) }
}
pub fn load_texture( filename : &str ) -> raylib_ffi::Texture {
	unsafe { return raylib_ffi::LoadTexture(raylib_ffi::rl_str!(filename)) }
}
pub fn load_texture_from_image( img : raylib_ffi::Image ) -> raylib_ffi::Texture {
	unsafe { return raylib_ffi::LoadTextureFromImage(img); }
}
pub fn draw_texture( texture : raylib_ffi::Texture, posX : i32, posY : i32, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawTexture(texture, posX, posY, tint) }
}
pub fn draw_texture_pro( texture : raylib_ffi::Texture, source : raylib_ffi::Rectangle, dest : raylib_ffi::Rectangle, origin : raylib_ffi::Vector2, rotation : f32, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawTexturePro(texture, source, dest, origin, rotation, tint); }
}
pub fn draw_texture_npatch( texture : raylib_ffi::Texture, dest : raylib_ffi::Rectangle, origin : raylib_ffi::Vector2, rotation : f32, tint : raylib_ffi::Color ) {
	let nPatchInfo = raylib_ffi::NPatchInfo {
		source: raylib_ffi::Rectangle {
			x: 0.0,
			y: 0.0,
			width: texture.width as f32,
			height: texture.height as f32,
		},
		left: texture.width / 3,
		top: texture.height / 3,
		right: texture.width / 3,
		bottom: texture.height / 3,
		layout: raylib_ffi::enums::NPatchLayout::NinePatch as i32,
	};
	unsafe { raylib_ffi::DrawTextureNPatch(texture, nPatchInfo, dest, origin, rotation, tint); }
}
pub fn image_resize_nn( image : &mut raylib_ffi::Image, scale : i32 ) {
	unsafe { raylib_ffi::ImageResizeNN(image, image.width * scale, image.height * scale); }
}

pub fn set_material_texture( material : *mut raylib_ffi::Material, mapType : raylib_ffi::enums::MaterialMapIndex, texture : raylib_ffi::Texture ) {
	unsafe { raylib_ffi::SetMaterialTexture(material, mapType as i32, texture) }
}

pub fn load_model( filename : &str ) -> raylib_ffi::Model {
	unsafe { return raylib_ffi::LoadModel(raylib_ffi::rl_str!(filename)) }
}

pub fn draw_mesh( mesh : *mut raylib_ffi::Mesh, material : &raylib_ffi::Material, transform : raylib_ffi::Matrix ) {
	unsafe {
		raylib_ffi::DrawMesh(*mesh, *material, transform);
	}
}
pub fn draw_model( model : raylib_ffi::Model, position : raylib_ffi::Vector3, scale : f32, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawModel(model, position, scale, tint); }
}
pub fn draw_model_ex( model : raylib_ffi::Model, position : raylib_ffi::Vector3, rotationAxis : raylib_ffi::Vector3, rotationAngle : f32, scale : raylib_ffi::Vector3, tint : raylib_ffi::Color ) {
	unsafe { raylib_ffi::DrawModelEx(model, position, rotationAxis, rotationAngle, scale, tint); }
}

pub fn load_default_material() -> raylib_ffi::Material {
	unsafe { return raylib_ffi::LoadMaterialDefault(); }
}
pub fn unload_material( material : raylib_ffi::Material ) {
	unsafe { raylib_ffi::UnloadMaterial(material); }
}

pub fn get_frame_time() -> f32 {
	unsafe { return raylib_ffi::GetFrameTime(); }
}

pub fn begin_3d_mode( camera : &Camera ) {
	unsafe {
		let rlCamera = raylib_ffi::Camera3D{
			position:	camera.camPosition,
			target:		camera.position,
			up:			Vector3{x:0.0,y:1.0,z:0.0},
			fovy:		camera.fovy,
			projection:	raylib_ffi::enums::CameraProjection::Perspective as i32,
		};

		raylib_ffi::BeginMode3D(rlCamera);
	}
}
pub fn end_3d_mode() {
	unsafe { raylib_ffi::EndMode3D(); }
}

pub fn draw_grid( slices : i32, spacing : f32 ) {
	unsafe { raylib_ffi::DrawGrid(slices, spacing); }
}

pub fn button_pressed( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyPressed(key ); }
}
pub fn button_down( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyDown(key ); }
}
pub fn button_released( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyReleased(key); }
}
pub fn button_up( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsKeyUp(key); }
}
pub fn get_key_pressed() -> String {
	unsafe {
		let keyAsI32 = raylib_ffi::GetKeyPressed();

		match keyAsI32 {
			65 => {
				if !button_down(340) && !button_down(344) { return "a".to_string(); }
				else { return "A".to_string(); }
			},
			66 => {
				if !button_down(340) && !button_down(344) { return "b".to_string(); }
				else { return "B".to_string(); }
			},
			67 => {
				if !button_down(340) && !button_down(344) { return "c".to_string(); }
				else { return "C".to_string(); }
			},
			68 => {
				if !button_down(340) && !button_down(344) { return "d".to_string(); }
				else { return "D".to_string(); }
			},
			69 => {
				if !button_down(340) && !button_down(344) { return "e".to_string(); }
				else { return "E".to_string(); }
			},
			70 => {
				if !button_down(340) && !button_down(344) { return "f".to_string(); }
				else { return "F".to_string(); }
			},
			71 => {
				if !button_down(340) && !button_down(344) { return "g".to_string(); }
				else { return "G".to_string(); }
			},
			72 => {
				if !button_down(340) && !button_down(344) { return "h".to_string(); }
				else { return "H".to_string(); }
			},
			73 => {
				if !button_down(340) && !button_down(344) { return "i".to_string(); }
				else { return "I".to_string(); }
			},
			74 => {
				if !button_down(340) && !button_down(344) { return "j".to_string(); }
				else { return "J".to_string(); }
			},
			75 => {
				if !button_down(340) && !button_down(344) { return "k".to_string(); }
				else { return "K".to_string(); }
			},
			76 => {
				if !button_down(340) && !button_down(344) { return "l".to_string(); }
				else { return "L".to_string(); }
			},
			77 => {
				if !button_down(340) && !button_down(344) { return "m".to_string(); }
				else { return "M".to_string(); }
			},
			78 => {
				if !button_down(340) && !button_down(344) { return "n".to_string(); }
				else { return "N".to_string(); }
			},
			79 => {
				if !button_down(340) && !button_down(344) { return "o".to_string(); }
				else { return "O".to_string(); }
			},
			80 => {
				if !button_down(340) && !button_down(344) { return "p".to_string(); }
				else { return "P".to_string(); }
			},
			81 => {
				if !button_down(340) && !button_down(344) { return "q".to_string(); }
				else { return "Q".to_string(); }
			},
			82 => {
				if !button_down(340) && !button_down(344) { return "r".to_string(); }
				else { return "R".to_string(); }
			},
			83 => {
				if !button_down(340) && !button_down(344) { return "s".to_string(); }
				else { return "S".to_string(); }
			},
			84 => {
				if !button_down(340) && !button_down(344) { return "t".to_string(); }
				else { return "T".to_string(); }
			},
			85 => {
				if !button_down(340) && !button_down(344) { return "u".to_string(); }
				else { return "U".to_string(); }
			},
			86 => {
				if !button_down(340) && !button_down(344) { return "v".to_string(); }
				else { return "V".to_string(); }
			},
			87 => {
				if !button_down(340) && !button_down(344) { return "w".to_string(); }
				else { return "W".to_string(); }
			},
			88 => {
				if !button_down(340) && !button_down(344) { return "x".to_string(); }
				else { return "X".to_string(); }
			},
			89 => {
				if !button_down(340) && !button_down(344) { return "y".to_string(); }
				else { return "Y".to_string(); }
			},
			90 => {
				if !button_down(340) && !button_down(344) { return "z".to_string(); }
				else { return "Z".to_string(); }
			},
			32 => return " ".to_string(),
			259 => return ".".to_string(),
			_ => return "".to_string(),
		}
	}
}

pub fn mouse_button_pressed( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonPressed(key); }
}
pub fn mouse_button_down( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonDown(key); }
}
pub fn mouse_button_released( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonReleased(key); }
}
pub fn mouse_button_up( key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsMouseButtonUp(key); }
}

pub fn gamepad_available( gamepad : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadAvailable(gamepad); }
}
pub fn gamepad_button_pressed( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonPressed(gamepad, key); }
}
pub fn gamepad_button_down( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonDown(gamepad, key); }
}
pub fn gamepad_button_released( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonReleased(gamepad, key); }
}
pub fn gamepad_button_up( gamepad : i32, key : i32 ) -> bool {
	unsafe { return raylib_ffi::IsGamepadButtonUp(gamepad, key); }
}

pub fn init_audio_device() {
	unsafe { raylib_ffi::InitAudioDevice() }
}
pub fn close_audio_device() {
	unsafe { raylib_ffi::CloseAudioDevice() }
}

pub fn play_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::PlaySound(sound) }
}
pub fn stop_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::StopSound(sound) }
}
pub fn pause_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::PauseSound(sound) }
}
pub fn resume_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::ResumeSound(sound) }
}
pub fn is_sound_playing( sound: raylib_ffi::Sound ) -> bool {
	unsafe { return raylib_ffi::IsSoundPlaying(sound) }
}
pub fn set_sound_volume( sound: raylib_ffi::Sound, volume: f32 ) {
	unsafe { raylib_ffi::SetSoundVolume(sound, volume) }
}
pub fn load_sound( fileName: &str ) -> raylib_ffi::Sound {
	unsafe { return raylib_ffi::LoadSound(raylib_ffi::rl_str!(fileName)) }
}
pub fn unload_sound( sound: raylib_ffi::Sound ) {
	unsafe { raylib_ffi::UnloadSound(sound) }
}

pub fn play_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::PlayMusicStream(music) }
}
pub fn stop_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::StopMusicStream(music) }
}
pub fn pause_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::PauseMusicStream(music) }
}
pub fn resume_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::ResumeMusicStream(music) }
}
pub fn is_music_playing( music: raylib_ffi::Music ) -> bool {
	unsafe { return raylib_ffi::IsMusicStreamPlaying(music) }
}
pub fn set_music_volume( music: raylib_ffi::Music, volume: f32 ) {
	unsafe { raylib_ffi::SetMusicVolume(music, volume) }
}
pub fn load_music( fileName: &str ) -> raylib_ffi::Music {
	unsafe { return raylib_ffi::LoadMusicStream(raylib_ffi::rl_str!(fileName)) }
}
pub fn unload_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::UnloadMusicStream(music) }
}
pub fn is_music_ready( music: raylib_ffi::Music ) -> bool {
	unsafe { return raylib_ffi::IsMusicReady(music) }
}
pub fn update_music( music: raylib_ffi::Music ) {
	unsafe { raylib_ffi::UpdateMusicStream(music) }
}

pub fn load_shader(vsFileName: &str, fsFileName: &str) -> raylib_ffi::Shader {
	unsafe { return raylib_ffi::LoadShader(raylib_ffi::rl_str!(vsFileName), raylib_ffi::rl_str!(fsFileName)); }
}
pub fn get_shader_location(shader: raylib_ffi::Shader, uniformName: &str) -> i32 {
	unsafe { return raylib_ffi::GetShaderLocation(shader, raylib_ffi::rl_str!(uniformName)); }
}
pub fn set_shader_value(shader: raylib_ffi::Shader, locIndex: i32, value: *const std::ffi::c_void, uniformType: ShaderUniformDataType ) {
	unsafe { raylib_ffi::SetShaderValue(shader, locIndex, value, uniformType as i32) }
}

pub fn begin_texture_mode(target: raylib_ffi::RenderTexture) {
	unsafe { raylib_ffi::BeginTextureMode(target) }
}
pub fn end_texture_mode() {
	unsafe { raylib_ffi::EndTextureMode() }
}
pub fn draw_texture_rec(texture: raylib_ffi::Texture, source: raylib_ffi::Rectangle , position: raylib_ffi::Vector2 , tint: raylib_ffi::Color) {
	unsafe { raylib_ffi::DrawTextureRec(texture, source, position, tint); }
}

pub fn begin_shader_mode(shader: raylib_ffi::Shader) {
	unsafe { raylib_ffi::BeginShaderMode(shader) }
}
pub fn end_shader_mode() {
	unsafe { raylib_ffi::EndShaderMode(); }
}

extern "C" {
	pub fn rlLoadTexture(
		data: i32,
		width: i32,
		height: i32,
		format: i32,
		mipmapCount: i32,
	) -> u32;
}
extern "C" {
	pub fn rlLoadTextureDepth(
		width: i32,
		height: i32,
		useRenderBuffer: bool,
	) -> u32;
}
extern "C" {
	pub fn rlLoadFramebuffer(
		width: i32,
		height: i32,
	) -> u32;
}
extern "C" {
	pub fn rlEnableFramebuffer(
		id: u32,
	);
}
extern "C" {
	pub fn rlFramebufferAttach(
		fboId: u32,
		texId: u32,
		attachType: i32,
		texType: i32,
		mipLevel: i32,
	);
}
extern "C" {
	pub fn rlFramebufferComplete(
		id: u32,
	) -> bool;
}
extern "C" {
	pub fn rlDisableFramebuffer();
}
pub fn load_render_texture_depth_tex(width: i32, height: i32) -> raylib_ffi::RenderTexture2D {
	unsafe {
		let target: raylib_ffi::RenderTexture2D = raylib_ffi::RenderTexture2D{
			id: rlLoadFramebuffer(width, height),
			texture: raylib_ffi::Texture {
				id: rlLoadTexture(0, width, height, PixelFormat::PixelformatUncompressedR8g8b8 as i32, 1),
				width: width,
				height: height,
				mipmaps: 1,
				format: PixelFormat::PixelformatUncompressedR8g8b8 as i32,
			},
			depth: raylib_ffi::Texture {
				id: rlLoadTextureDepth(width, height, false),
				width: width,
				height: height,
				mipmaps: 1,
				format: 19,
			}
		};

		rlEnableFramebuffer(target.id);
		rlFramebufferAttach(
			target.id as u32,
			target.texture.id as u32,
			RlFramebufferAttachType::RlAttachmentColorChannel0 as i32,
			RlFramebufferAttachTextureType::RlAttachmentTexture2d as i32,
			0,
		);
		rlFramebufferAttach(
			target.id as u32,
			target.depth.id as u32,
			RlFramebufferAttachType::RlAttachmentDepth as i32,
			RlFramebufferAttachTextureType::RlAttachmentTexture2d as i32,
			0,
		);

		if rlFramebufferComplete(target.id) { print!("Fucking hell this is hard... But it worked?\n") }
		else { print!("It fucking failed. Fuck\n"); }

		rlDisableFramebuffer();

		return target;
	}
}