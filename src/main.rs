use std::rc::Rc;
use std::cell::RefCell;

use glfw::{Action, Context, Key};
use lua_bindings::{bind_lua, reload_and_execute_script};
use mesh::Mesh;
use mlua::Lua;
use resource_manager::ResourceManager;
use shader::Shader;
use texture::Texture;
use transform::Transform2D;
mod buffers;
mod shader;
mod texture;
mod mesh;
mod lua_bindings;
mod camera;
mod transform;
mod resource_manager;
mod container;
use mlua::prelude::*;

fn error_callback(err: glfw::Error, description: String) {
    panic!("GLFW error {:?}: {:?}", err, description);
}

/*
unsafe {
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    gl::Enable( gl::BLEND );
    //gl::Enable(gl::CULL_FACE);
}*/
/*
    let mut tex = Box::new(Texture::new(gl::TEXTURE_2D));
    tex.create();
    let mut id = RgbImage::new(256, 256);
    for (x, y, pixel) in id.enumerate_pixels_mut() {
        *pixel = Rgb([x as u8, y as u8, 0]);
    }
    tex.set_texture_rgb(&id);
    resource_manager.borrow_mut().add_resource("default_texture",  tex);
*/
fn main() {
    let mut glfw = glfw::init(error_callback).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    
    let (mut window, events) = glfw.create_window(800, 600, "OpenGL Triangle",glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let lua: Lua = Lua::new();
    let resource_manager: Rc<RefCell<ResourceManager>> = Rc::new(RefCell::new(ResourceManager::new()));
    bind_lua(&lua,&resource_manager);

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
                    if let Err(e) =  x.call::<()>(()) {
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
            rm.camera_mut().update();
            if let Some(shader) = rm.get_resource_mut::<Shader>("default_shader") {
                shader.bind();
                shader.set_uniform_mat3x2("transform", x.transformation_matrix());
            }
            if let Some(texture) = rm.get_resource_mut::<Texture>("default_texture") {
                texture.bind(gl::TEXTURE0);
            }
            if let Some(mesh) = rm.get_resource_mut::<Mesh>("default_quad_mesh_strip") {
                mesh.bind();
            }
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
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