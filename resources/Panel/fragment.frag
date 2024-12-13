#version 330 core

out vec4 FragColor;
in vec2 TexCoord;
uniform vec4 color = vec4(1.0, 1.0, 1.0, 1.0);
uniform vec2 offset = vec2(0.0,0.0);
uniform vec2 scale = vec2(1.0, 1.0);
uniform sampler2D image;
void main()
{
    FragColor = texture(image,(TexCoord*scale)+offset) * color;
}