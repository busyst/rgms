use gl::types::GLenum;

pub struct BufferObject {
    buffer: u32,
    buffer_type: GLenum,
}
pub struct VertexArrayObject {
    buffer: u32,
}
pub struct FramebufferObject {
    buffer: u32,
}

impl BufferObject {
    pub fn new(buffer_type: GLenum) -> BufferObject {
        BufferObject {
            buffer: 0,
            buffer_type,
        }
    }

    pub fn create(&mut self) {
        self.delete();
        unsafe {
            gl::GenBuffers(1, &mut self.buffer);
            gl::BindBuffer(self.buffer_type, self.buffer);
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.buffer_type, self.buffer) };
    }
    
    pub fn set_data<T>(&self, data: &[T], usage: GLenum) {
        unsafe {
            gl::BindBuffer(self.buffer_type, self.buffer);
            gl::BufferData(
                self.buffer_type,
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const _,
                usage
            );
        }
    }
    fn delete(&mut self) {
        if self.buffer != 0 {
            unsafe { gl::DeleteBuffers(1, &self.buffer) }; // Changed from DeleteVertexArrays to DeleteBuffers
            self.buffer = 0;
        }
    }
}
impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        // Initialize with buffer = 0
        VertexArrayObject {
            buffer: 0
        }
    }

    pub fn create(&mut self) {
        self.delete();
        unsafe {
            gl::GenVertexArrays(1, &mut self.buffer);
        }
    }

    pub fn bind(&self) {
        unsafe {gl::BindVertexArray(self.buffer) };
    }
    fn delete(&mut self) {
        if self.buffer != 0 {
            unsafe { gl::DeleteVertexArrays(1, &self.buffer) }; // Changed from DeleteVertexArrays to DeleteBuffers
            self.buffer = 0;
        }
    }
}

impl FramebufferObject {
    pub fn new() -> VertexArrayObject {
        // Initialize with buffer = 0
        VertexArrayObject {
            buffer: 0
        }
    }

    pub fn create(&mut self) {
        self.delete();
        unsafe {
            gl::GenFramebuffers(1, &mut self.buffer);
        }
    }

    pub fn bind(&self) {
        unsafe {gl::BindFramebuffer(gl::FRAMEBUFFER,self.buffer) };
    }
    fn delete(&mut self) {
        if self.buffer != 0 {
            unsafe { gl::DeleteFramebuffers(1, &self.buffer) }; // Changed from DeleteVertexArrays to DeleteBuffers
            self.buffer = 0;
        }
    }
}


impl Drop for BufferObject {
    fn drop(&mut self) {
        self.delete();
    }
}
impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        self.delete();
    }
}
impl Drop for FramebufferObject {
    fn drop(&mut self) {
        self.delete();
    }
}