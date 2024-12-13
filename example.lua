
local function load_shader(name,vertex_path,fragment_path)
    print("Load shader function")
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
    {x = 0.5, y = 0.5, tx = 1.0, ty = 1.0}
}
function load(x)
    print("Load function: " .. x)
    load_shader("default_shader","./resources/Panel/vertex.vert","./resources/Panel/fragment.frag")
    material_load_mesh("default_quad_mesh",quad)
    return ""
end