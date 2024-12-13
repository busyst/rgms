use crate::buffers::{BufferObject, VertexArrayObject};
pub trait MeshVertex {
    unsafe fn enable_vertex_attrib();
}


pub struct Mesh{
    vao: VertexArrayObject,
    vbo: BufferObject,
    ebo: BufferObject,

    vertices_count: u32, 
    indices_count: u32,
}
impl Mesh {
    pub fn new() -> Mesh{
        Mesh{
            vao: VertexArrayObject::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER),
            vertices_count: 0,
            indices_count: 0,
        }
    }

    pub fn create<Tv,Ti>(&mut self,vertices: &[Tv],indices:Option<&[Ti]>) where Tv : MeshVertex
    {
        self.delete();

        self.vao.create();
        self.vbo.create();

        self.vao.bind();
        self.vbo.bind();

        self.vbo.set_data(vertices, gl::STATIC_DRAW);

        self.vertices_count = vertices.len() as u32;

        unsafe { Tv::enable_vertex_attrib() };

        if let Some(idx) = indices {
            self.ebo.create();
            self.ebo.bind();
            self.ebo.set_data(idx, gl::STATIC_DRAW);
            self.indices_count = idx.len() as u32;
        }
    }
    pub fn bind(&self) {
        self.vao.bind();
    }
    fn delete(&mut self){
        self.vao = VertexArrayObject::new();
        self.vbo = BufferObject::new(gl::ARRAY_BUFFER);
        self.ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER);
    }
}
impl Drop for Mesh {
    fn drop(&mut self) {
        self.delete();
    }
}



pub struct PlanarTextureVertex{
    _x:[u8;std::mem::size_of::<f32>()*4]
}
impl MeshVertex for PlanarTextureVertex {
    unsafe fn enable_vertex_attrib() {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, 0 as *const _);
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<f32>() as i32, (2 * std::mem::size_of::<f32>()) as *const _);
    }
}
impl PlanarTextureVertex {
    pub fn new(x: f32, y: f32, tx: f32, ty: f32) -> Self {
        let bytes: [u8; std::mem::size_of::<f32>() * 4] = unsafe {
            std::mem::transmute([x, y, tx, ty])
        };
        Self { _x: bytes }
    }
}