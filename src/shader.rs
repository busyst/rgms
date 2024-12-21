use std::{collections::HashMap, ffi::CString, ptr};

use gl::types::{GLchar, GLenum, GLint, GLuint};


pub struct Shader{
    program: u32,
    uniform_location_cache: HashMap<String, i32>,
}

impl Shader {
    /*pub fn new() -> Shader {
        Shader{program:0, uniform_location_cache: HashMap::new() }
    }*/
    pub fn create_new(vertex_shader: &str, fragment_shader:  &str) -> Shader {
        let mut x =Shader {
            program : 0,
            uniform_location_cache: HashMap::new(),
        };
        x.create(vertex_shader,fragment_shader);
        x
    }
    pub fn create(&mut self,vertex_shader: &str, fragment_shader:  &str) {
        self.delete();
        let vert = Self::compile_shader(vertex_shader, gl::VERTEX_SHADER).expect("VERTEX SHADER failed to compile");
        let frag = Self::compile_shader(fragment_shader, gl::FRAGMENT_SHADER).expect("FRAGMENT SHADER failed to compile");

        unsafe { 
            self.program = gl::CreateProgram();
            gl::AttachShader(self.program,vert);
            gl::AttachShader(self.program,frag);
            let link_res = Self::link_program(self.program);
            gl::DeleteShader(vert);
            gl::DeleteShader(frag);
            if link_res.is_err(){
                self.delete();
            }
            
        };
    }
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program); };
    }


    fn delete(&mut self){
        if self.program !=0{
            unsafe {gl::DeleteProgram(self.program)};
            self.program = 0;
        }
    }

    pub fn set_uniform_mat3x2(&mut self, name: &str, value: &[f32; 6]) {
        self.bind();
        unsafe {
            gl::UniformMatrix3x2fv(
                self.get_uniform_location(name),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        if let Some(&location) = self.uniform_location_cache.get(name) {
            return location;
        }

        let c_name = CString::new(name).unwrap();
        let location = unsafe {
            gl::GetUniformLocation(self.program, c_name.as_ptr())
        };

        if location == -1 {
            println!("{} not found", name);
            return 0;
        }

        // Since self.uniform_location_cache is behind a reference,
        // we need to clone the String to own it
        self.uniform_location_cache.insert(name.to_string(), location);
        location
    }

    fn compile_shader(shader_source: &str, shader_type: GLenum) -> Result<GLuint, String> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        
        // Convert shader source to C string
        let source = CString::new(shader_source)
            .map_err(|e| format!("Failed to convert shader source to CString: {}", e))?;
        
        unsafe {
            gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Check compilation status
            let mut success: GLint = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                // Create buffer for error message
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1); // subtract 1 to skip the null terminator

                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar
                );

                gl::DeleteShader(shader);
                return Err(String::from_utf8_lossy(&buffer).to_string());
            }
        }

        Ok(shader)
    }
    fn link_program(program: GLuint) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(program);
    
            // Check linking status
            let mut success: GLint = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    
            if success == 0 {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
    
                // Create buffer for error message
                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1); // subtract 1 to skip the null terminator
    
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar
                );
    
                return Err(format!(
                    "ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}",
                    String::from_utf8_lossy(&buffer)
                ));
            }
    
            Ok(())
        }
    }
    pub fn bind_ubo(&self, name: &str, binding_point: GLuint) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let block_index = gl::GetUniformBlockIndex(self.program, c_str.as_ptr());
            gl::UniformBlockBinding(self.program, block_index, binding_point);
        }
    }
    
}
impl Drop for Shader {
    fn drop(&mut self) {
        self.delete();
    }
}