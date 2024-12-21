use gl::types::{GLenum, GLint, GLsizei};
use image::RgbImage;

pub struct Texture{
    handle: u32,
    texture_type: GLenum,
}
impl Texture {
    /*pub fn new(texture_type: GLenum) -> Texture {
        Texture {
            handle : 0,
            texture_type,
        }
    }*/
    pub fn create_new(texture_type: GLenum) -> Texture {
        let mut x =Texture {
            handle : 0,
            texture_type,
        };
        x.create();
        x
    }
    pub fn create(&mut self){
        self.delete();
        unsafe {
            gl::GenTextures(1, &mut self.handle);
        }
    }
    pub fn bind(&self, texture_unit : GLenum) {
        unsafe { gl::ActiveTexture(texture_unit); gl::BindTexture(self.texture_type,self.handle); };
    }
    pub fn delete(&mut self) {
        if self.handle != 0 {
            unsafe {gl::DeleteTextures(1, &self.handle)} ;
            self.handle = 0;
        }
    }
    pub fn set_texture_rgb(&mut self,image: &RgbImage){
        self.set_texture(gl::RGB, image.width() as i32, image.height() as i32, gl::RGB, image.as_raw().as_ptr());
    }
    /*
    pub fn set_texture_rgba(&mut self,image: &RgbaImage){
        self.set_texture(gl::RGBA, image.width() as i32, image.height() as i32, gl::RGBA, image.as_raw().as_ptr());
    }*/
    pub fn set_texture(&mut self,internal_format : GLenum, width : GLsizei, height : GLsizei, format : GLenum, data: *const u8){
        self.set_texture_wo_mipmap(internal_format,width,height,format,data);
        self.bind(gl::TEXTURE0);
        unsafe {
            gl::GenerateMipmap(self.texture_type);
            gl::BindTexture(self.texture_type, 0);
        }
    }
    pub fn set_texture_wo_mipmap(&mut self,internal_format : GLenum, width : GLsizei, height : GLsizei, format : GLenum, data: *const u8){
        self.bind(gl::TEXTURE0);
        unsafe {
            gl::TexImage2D(self.texture_type, 0, internal_format as GLint, width, height, 0, format, gl::UNSIGNED_BYTE, data as *const _);
            gl::BindTexture(self.texture_type, 0);
        }
    }
    
}
impl Drop for Texture {
    fn drop(&mut self) {
        self.delete();
    }
}