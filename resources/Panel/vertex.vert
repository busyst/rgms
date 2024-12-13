#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texCoord;
out vec2 TexCoord;
uniform mat3x2 transform;
void main()
{
    gl_Position = vec4(transform * vec3(position,1.0), 1.0, 1.0);
    TexCoord = texCoord;
}