#version 330 core

in vec2 tex_coord;
out vec4 frag_color;

uniform sampler2D sprite_texture;
uniform vec2 sprite_size = vec2(1.0, 1.0);
uniform vec2 sprite_offset = vec2(0.0, 0.0);


void main() {
    // Calculate the actual texture coordinates
    vec2 coord = tex_coord;
    // Apply sprite offset and size
    coord = (coord * sprite_size + sprite_offset);
    
    // Sample the texture
    frag_color = texture(sprite_texture, coord);
}
