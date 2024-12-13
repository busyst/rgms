use gl::types::{GLenum, GLint, GLsizei, GLubyte};

pub struct Texture{
    handle: u32,
    texture_type: GLenum,
}
impl Texture {
    pub fn new(texture_type: GLenum) -> Texture {
        Texture {
            handle : 0,
            texture_type,
        }
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
    pub fn set_texture(&mut self,internal_format : GLenum, width : GLsizei, height : GLsizei, format : GLenum, data: &[u8]){
        self.set_texture_wo_mipmap(internal_format,width,height,format,data);
        self.bind(gl::TEXTURE0);
        unsafe {
            gl::GenerateMipmap(self.texture_type);
            gl::BindTexture(self.texture_type, 0);
        }
    }
    pub fn set_texture_wo_mipmap(&mut self,internal_format : GLenum, width : GLsizei, height : GLsizei, format : GLenum, data: &[GLubyte]){
        self.bind(gl::TEXTURE0);
        unsafe {
            gl::TexImage2D(self.texture_type, 0, internal_format as GLint, width, height, 0, format, gl::UNSIGNED_BYTE, data.as_ptr() as *const _);
            gl::BindTexture(self.texture_type, 0);
        }
    }
    
}
impl Drop for Texture {
    fn drop(&mut self) {
        self.delete();
    }
}