use gl::types::*;
use glam::{I16Vec2, Vec2};

use crate::buffers::BufferObject;

pub struct Camera {
    position: Vec2,
    screen_pos: Vec2,
    world_pos: Vec2,
    window_size: I16Vec2,
    scale: f32,
    aspect_ratio: f32,
    
    update_mat_proj: bool,
    update_mat_position: bool,
    update_mat_scale: bool,

    ubo: BufferObject,
    matrices: [f32; 32],
}

impl Camera {
    pub const MATRICES_BINDING_POINT: GLuint = 2;
    pub const Z_NEAR_PLANE: f32 = 0.01;
    pub const Z_FAR_PLANE: f32 = 30.0;

    pub fn new() -> Self {
        let mut camera = Camera {
            position: Vec2::new(0.0, 0.0),
            screen_pos: Vec2::new(0.0, 0.0),
            world_pos: Vec2::new(0.0, 0.0),
            window_size: I16Vec2::new(1, 1),
            scale: 1.0,
            aspect_ratio: 1.0,
            update_mat_proj: true,
            update_mat_position: true,
            update_mat_scale: true,
            ubo: BufferObject::new(gl::UNIFORM_BUFFER),
            matrices: [0.0; 32],
        };
        camera.initialize_ubo();
        camera
    }

    pub fn update(&mut self) {
        unsafe {
            let mut viewport = [0; 4];
            gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());
            
            let new_size = I16Vec2::new(viewport[2] as i16, viewport[3] as i16);
            self.update_mat_position |= self.window_size != new_size;
            self.window_size = new_size;
        }

        self.ubo.bind();

        if self.update_mat_proj {
            self.update_mat_proj = false;
            self.aspect_ratio = self.window_size.x as f32 / self.window_size.y as f32;
            self.matrices[0] = 1.0 / (self.aspect_ratio * self.scale);
            self.ubo.update_data(0,std::mem::size_of::<f32>(), &self.matrices[0..1]);
        }

        if self.update_mat_scale {
            self.update_mat_scale = false;
            self.matrices[0] = 1.0 / (self.aspect_ratio * self.scale);
            self.matrices[5] = 1.0 / self.scale;
            self.ubo.update_data(0, std::mem::size_of::<f32>() * 6, &self.matrices[0..6]);
        }

        if self.update_mat_position {
            self.update_mat_position = false;
            self.matrices[28] = -self.position.x;
            self.matrices[29] = -self.position.y;
            self.ubo.update_data(
                std::mem::size_of::<f32>() * 28,
                std::mem::size_of::<f32>() * 2,
                &self.matrices[28..30]
            );
        }

        self.update_mouse_pos();
    }

    fn initialize_ubo(&mut self) {
        self.ubo.create();
        
        // Set up projection matrix
        self.matrices[0] = 1.0 / (self.aspect_ratio * self.scale);
        self.matrices[5] = 1.0 / self.scale;
        self.matrices[10] = 1.0 / (Self::Z_NEAR_PLANE - Self::Z_FAR_PLANE);
        self.matrices[14] = Self::Z_NEAR_PLANE / (Self::Z_NEAR_PLANE - Self::Z_FAR_PLANE);
        self.matrices[15] = 1.0;
        
        // Set up view matrix (identity)
        self.matrices[16] = 1.0;
        self.matrices[21] = 1.0;
        self.matrices[26] = 1.0;
        self.matrices[31] = 1.0;

        self.ubo.set_data(&self.matrices,gl::STATIC_DRAW);
        self.ubo.bind_to_binding_point(Self::MATRICES_BINDING_POINT);
    }

    fn update_mouse_pos(&mut self) {
        self.screen_pos = Vec2::new(0.0,0.0
        //    (self.input.get_mouse_x() / (self.window_size.x as f32 - 1.0)) - 0.5,
        //    0.5 - (self.input.get_mouse_y() / (self.window_size.y as f32 - 1.0))
        ) * 2.0;
        
        self.world_pos = self.screen_pos * self.scale;
        self.world_pos.x *= self.aspect_ratio;
        self.world_pos += self.position;
    }

    // Getters
    pub fn mouse_screen_position(&self) -> &Vec2 { &self.screen_pos }
    pub fn mouse_projected_pos(&self) -> &Vec2 { &self.world_pos }
    pub fn viewport_size(&self) -> &I16Vec2 { &self.window_size }
    pub fn position(&self) -> &Vec2 { &self.position }
    pub fn scale(&self) -> f32 { self.scale }

    // Setters
    pub fn set_position(&mut self, pos: Vec2) {
        self.update_mat_position |= self.position != pos;
        self.position = pos;
    }

    pub fn set_scale(&mut self, scale: f32) {
        let new_scale = scale.max(0.00001);
        self.update_mat_scale |= (self.scale - new_scale).abs() > f32::EPSILON;
        self.scale = new_scale;
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        self.ubo = BufferObject::new(gl::UNIFORM_BUFFER);
    }
}