use std::rc::Rc;
use std::cell::RefCell;

use std::{fs::File, io::Read};

use glfw::{Action, Context, Key};
use image_data::ImageData;
use mesh::{Mesh, PlanarTextureVertex};
use palette::Rgb;
use resource_manager::ResourceManager;
use shader::Shader;
use texture::Texture;
use transform::Transform2D;
mod buffers;
mod shader;
mod texture;
mod mesh;
mod transform;
mod resource_manager;
mod image_data;
mod palette;
mod container;
use mlua::prelude::*;


fn error_callback(err: glfw::Error, description: String) {
    panic!("GLFW error {:?}: {:?}", err, description);
}
fn reload_and_execute_script(lua: &Lua, script_path: &str) -> LuaResult<()> {
    // Read the Lua script
    let script_content = std::fs::read_to_string(script_path)
        .map_err(|e| LuaError::RuntimeError(format!("Failed to read script: {}", e)))?;

    // Load and execute the script
    lua.load(&script_content).exec()?;

    Ok(())
}
/*unsafe {
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable( gl::BLEND );
    //gl::Enable(gl::CULL_FACE);
}*/
fn main() {
    let mut glfw = glfw::init(error_callback).unwrap();
    
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(
        800, 
        600, 
        "OpenGL Triangle",
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    fn read_file(_: &Lua, path: String) -> LuaResult<String> {
        let mut buf1 = String::new();
        if let Ok(mut a) = File::open(path) {
            let _ = a.read_to_string(&mut buf1);
        }
        return Ok(buf1);
    }
    let lua: Lua = Lua::new();
    lua.globals().set("read_file", lua.create_function(read_file).unwrap()).unwrap();

    
    let resource_manager = Rc::new(RefCell::new(ResourceManager::new()));
    let resource_manager_clone = Rc::clone(&resource_manager);
    let resource_manager_clone_2 = Rc::clone(&resource_manager);
    let resource_manager_clone_3 = Rc::clone(&resource_manager);
    
    lua.globals().set("material_load_shader", lua.create_function_mut(move |_: &Lua, x: (String, String, String)| {
        let mut s = Box::new(Shader::new()); 
        s.create(&x.1, &x.2); 
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
        if let Ok(texture) = image::open(x.1){
            let pixels: Vec<u8> = texture.flipv().to_rgb8()
            .pixels()
            .flat_map(|pixel| pixel.0.to_vec())
            .collect();

            let mut t = Box::new(Texture::new(gl::TEXTURE_2D));
            t.create();
            t.set_texture(gl::RGB, texture.width().try_into().unwrap(), texture.height().try_into().unwrap(), gl::RGB, pixels.as_slice());
            resource_manager_clone_3.borrow_mut().add_resource(&x.0, t);
        }
        Ok(())
    }).unwrap()).unwrap();

    let mut tex = Box::new(Texture::new(gl::TEXTURE_2D));
    tex.create();


    let mut id: ImageData<Rgb> = ImageData::new_allocate(256, 256);
    id.map_coords(|x,y| Rgb::new(x as u8, y as u8 ,0 as u8));
    tex.set_texture(gl::RGB, id.width() as i32, id.height() as i32, gl::RGB, id.as_u8_slice_mut());
    resource_manager.borrow_mut().add_resource("default_texture",  tex);


    let mut x: Transform2D = Transform2D::default();
    
    let mut lua_ok = false;
    let mut lua_loaded = false;
    if reload_and_execute_script(&lua,"./example.lua").inspect_err(|e| println!("{}",e)).is_ok(){
        lua_ok = true;
        lua_loaded = false;
    }
    
    while !window.should_close() {
        if lua_ok {
            if !lua_loaded{
                resource_manager.borrow_mut().clear();
                if let Ok(x) = lua.globals().get::<LuaFunction>("load") {
                    if let Err(e) =  x.call::<String>("") {
                        println!("Load script error:{}",e);
                    }else {
                        lua_loaded = true;
                    }
                };
                if !lua_loaded{
                    lua_ok = false;
                }
            }
        }
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw triangle
            let mut rm = resource_manager.borrow_mut();
            if let Some(shader) = rm.get_resource_mut::<Shader>("default_shader") {
                shader.bind();
                shader.set_uniform_mat3x2("transform", x.transformation_matrix());
            }
            if let Some(texture) = rm.get_resource::<Texture>("default_texture") {
                texture.bind(gl::TEXTURE0);
            }
            if let Some(mesh) = rm.get_resource::<Mesh>("default_quad_mesh") {
                mesh.bind();
            }
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(Key::R, _, Action::Press, _) => {
                    if reload_and_execute_script(&lua,"./example.lua").inspect_err(|e| println!("{}",e)).is_ok(){
                        lua_ok = true;
                        lua_loaded = false;
                    }
                }
                _ => {}
            }
        }
    }
}