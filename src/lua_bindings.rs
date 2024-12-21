use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use glam::Vec2;
use mlua::prelude::*;

use crate::{camera::Camera, mesh::{Mesh, PlanarTextureVertex}, resource_manager::ResourceManager, shader::Shader, texture::Texture};
pub fn reload_and_execute_script(lua: &Lua, script_path: &str) -> LuaResult<()> {
    // Read the Lua script
    let script_content = std::fs::read_to_string(script_path)
        .map_err(|e| LuaError::RuntimeError(format!("Failed to read script: {}", e)))?;

    // Load and execute the script
    lua.load(&script_content).exec()?;

    Ok(())
}
pub fn bind_lua(lua :&Lua, resource_manager : &Rc<RefCell<ResourceManager>>){
    let resource_manager_clone = Rc::clone(&resource_manager);
    let resource_manager_clone_2 = Rc::clone(&resource_manager);
    let resource_manager_clone_3 = Rc::clone(&resource_manager);
    let resource_manager_clone_4 = Rc::clone(&resource_manager);
    let resource_manager_clone_5 = Rc::clone(&resource_manager);
    
    lua.globals().set("material_load_shader", lua.create_function_mut(move |_: &Lua, x: (String, String, String)| {
        let s = Box::new(Shader::create_new(&x.1,&x.2)); 
        s.bind_ubo("Matrices", Camera::MATRICES_BINDING_POINT);
        resource_manager_clone.borrow_mut().add_resource(&x.0, s);
        Ok(())
    }).unwrap()).unwrap();
    lua.globals().set("material_load_mesh", lua.create_function_mut(move |_: &Lua, x: (String, LuaTable)| {
        let mesh_table: LuaTable = x.1;
        
        // Convert LuaTable to Vec of PlanarTextureVertex
        let mesh: Vec<PlanarTextureVertex> = mesh_table.pairs::<String, LuaTable>()
            .map(|pair| {
                let vertex_table = pair.unwrap().1;
                let x: f32 = vertex_table.get::< f32>("x").unwrap_or(0.0);
                let y: f32 = vertex_table.get::<f32>("y").unwrap_or(0.0);
                let tx: f32 = vertex_table.get::<f32>("tx").unwrap_or(0.0);
                let ty: f32 = vertex_table.get::<f32>("ty").unwrap_or(0.0);
                
                PlanarTextureVertex::new(x, y, tx, ty)
            })
            .collect();
    
        let mut s = Box::new(Mesh::new());
        s.create::<PlanarTextureVertex,u8>(&mesh, None);
        resource_manager_clone_2.borrow_mut().add_resource(&x.0, s);
        Ok(())
    }).unwrap()).unwrap();
    lua.globals().set("material_load_texture", lua.create_function_mut(move |_: &Lua, x: (String, String)| {
        let path = x.1.clone();
        if let Ok(texture) = image::open(path){
            let mut t = Box::new(Texture::create_new(gl::TEXTURE_2D));
            t.set_texture_rgb(&texture.flipv().to_rgb8());
            resource_manager_clone_3.borrow_mut().add_resource(&x.0, t);
        }else {
            println!("File \"{}\" is not found or not a image",x.1);
        }
        Ok(())
    }).unwrap()).unwrap();

    lua.globals().set("read_file", lua.create_function(|_: &Lua, path: String| {
        let mut buf1 = String::new();
        if let Ok(mut a) = File::open(path) {
            let _ = a.read_to_string(&mut buf1);
        }
        return Ok(buf1);
    }).unwrap()).unwrap();

    lua.globals().set("get_camera_position", lua.create_function_mut(move |lua: &Lua, ()| {
        // Create a Lua table representing a vector
        let rm = resource_manager_clone_4.borrow();
        let a = rm.camera();
        let vec_table = lua.create_table()?;
        vec_table.set("x", a.position().x)?; // x component
        vec_table.set("y", a.position().y)?; // y component
        Ok(vec_table)
    }).unwrap()).unwrap();
    lua.globals().set("set_camera_position", lua.create_function_mut(move |_: &Lua, x: LuaTable| {
        let x_ = x.get("x").unwrap_or(0.0);
        let y_ = x.get("y").unwrap_or(0.0);
        let mut rm = resource_manager_clone_5.borrow_mut();
        rm.camera_mut().set_position(Vec2::new(x_,y_));
        Ok(())
    }).unwrap()).unwrap();
}