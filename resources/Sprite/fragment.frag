#version 330 core

in vec2 tex_coord;
out vec4 frag_color;

uniform sampler2D sprite_texture;
uniform vec2 sprite_size = vec2(1.0, 1.0);
uniform vec2 sprite_offset = vec2(0.0, 0.0);
uniform vec4 color_tint = vec4(1.0, 1.0, 1.0, 1.0);

uniform bool is_animated = false;
uniform float time = 0;
uniform int num_frames = 1;
uniform float anim_speed = 0.0;

uniform bool is_flipped_x = false;
uniform bool is_flipped_y = false;

uniform bool use_normal_map = false;
uniform sampler2D normal_map;
uniform vec3 light_dir = vec3(0.25, 0.1, 0.25);


void main() {
    // Calculate the actual texture coordinates
    vec2 coord = tex_coord;
    
    // Handle sprite flipping
    if (is_flipped_x) coord.x = 1.0 - coord.x;
    if (is_flipped_y) coord.y = 1.0 - coord.y;
    
    // Handle sprite animation
    if (is_animated) {
        float frame_index = mod(floor(time * anim_speed), float(num_frames));
        coord.x = (coord.x + frame_index) / float(num_frames);
    }
    
    // Apply sprite offset and size
    coord = (coord * sprite_size + sprite_offset);
    
    // Sample the texture
    vec4 tex_color = texture(sprite_texture, coord);
    
    // Apply color tint and alpha
    vec4 tinted_color = tex_color * color_tint;
    
    // Apply normal mapping if enabled
    if (use_normal_map) {
        vec3 normal = texture(normal_map, coord).xyz * 2.0 - 1.0;
        float light_intensity = max(dot(normal, normalize(light_dir)), 0.0);
        tinted_color.rgb *= light_intensity;
    }
    
    frag_color = tinted_color;
}
