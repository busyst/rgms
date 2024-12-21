function Vec(x, y)
    local vector = {x = x, y = y}
    local mt = {
        __add = function(a, b)
            return Vec(a.x + b.x, a.y + b.y)
        end
    }
    setmetatable(vector, mt)
    return vector
end

local function load_shader(name,vertex_path,fragment_path)
    local v = read_file(vertex_path)
    if v == "" then
        return
    end
    local f = read_file(fragment_path)
    if f == "" then
        return
    end
    material_load_shader(name,v,f)
end
local quad = {
    {x = -0.5, y = -0.5, tx = 0.0, ty = 0.0},
    {x = 0.5, y = -0.5, tx = 1.0, ty = 0.0},
    {x = -0.5, y = 0.5, tx = 0.0, ty = 1.0},
    {x = 0.5, y = 0.5, tx = 1.0, ty = 1.0},
}
function load()
    load_shader("default_shader","./resources/Sprite/vertex.vert","./resources/Sprite/fragment.frag")
    material_load_mesh("default_quad_mesh_strip",quad)
    material_load_texture("default_texture","./resources/image.png")

    set_camera_position(Vec(0.5,0.5) + get_camera_position())
end