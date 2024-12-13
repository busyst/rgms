#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texCoord;
out vec2 tex_coord;

uniform mat3x2 transform;

layout (std140) uniform Matrices
{
    mat4 Projection;
    mat4 View;
};

void main()
{
    gl_Position =  Projection * (View * vec4(transform*vec3(position,1.0), 1.0, 1.0));

    tex_coord = texCoord; // pass texture coordinates to the fragment shader
}
