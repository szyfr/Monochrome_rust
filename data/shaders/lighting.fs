#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;


uniform vec2 textureSize;
uniform float time;

void main() {
	vec2 onePixel = vec2(1.0, 1.0) / textureSize;
	
	vec4 texelColor = texture(texture0, fragTexCoord);

	vec4 off;
	if (time < 1f) { off = vec4(time, time, 1f, 1f); }
	else { off = vec4(2f - time, 2f - time, 1f, 1f); }

	bool light = false;
	vec4 lightColor = vec4(1f, 1f, 0.22745098173618316650390625f, 1f);

	if (texelColor == lightColor) { light = true; }

	//if (texture(texture0, vec2(fragTexCoord.x, fragTexCoord.y + scale.y)) == lightColor) { light = true; }
	//if (texture(texture0, vec2(fragTexCoord.x, fragTexCoord.y - scale.y)) == lightColor) { light = true; }
	//if (texture(texture0, vec2(fragTexCoord.x + scale.x, fragTexCoord.y)) == lightColor) { light = true; }
	//if (texture(texture0, vec2(fragTexCoord.x - scale.x, fragTexCoord.y)) == lightColor) { light = true; }

	if (texture(texture0, vec2(fragTexCoord.x, fragTexCoord.y + onePixel.y)) == lightColor) { light = true; }
	if (texture(texture0, vec2(fragTexCoord.x, fragTexCoord.y - onePixel.y)) == lightColor) { light = true; }
	if (texture(texture0, vec2(fragTexCoord.x + onePixel.x, fragTexCoord.y)) == lightColor) { light = true; }
	if (texture(texture0, vec2(fragTexCoord.x - onePixel.x, fragTexCoord.y)) == lightColor) { light = true; }
	
	if (light) { finalColor = texelColor; }
	else { finalColor = texelColor*off; }
}