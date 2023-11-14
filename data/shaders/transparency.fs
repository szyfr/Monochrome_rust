#version 330

uniform sampler2D myTexture;
varying vec2 TexCoord;

void main(void) {
	if(texture2D(myTexture, TexCoord).a != 0.0f) {
	    discard;
	}
	gl_FragColor = texture2D(myTexture, TexCoord);
}