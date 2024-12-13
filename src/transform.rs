use std::f32::consts::PI;

use glam::{vec2, Vec2};

#[derive(Debug)]
pub struct Transform2D {
    scale: Vec2,
    radians: f32,
    transform_matrix: [f32; 6],
    update_required: bool,
}

impl Transform2D {
    pub fn new(position: Vec2, scale: Vec2, radians: f32) -> Self {

        let mut transform = Self {
            scale,
            radians,
            transform_matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            update_required: false,
        };
        transform.set_position(position);
        transform
    }

    pub fn default() -> Self {
        let hashcode = 19995124;
        Self {
            scale: Vec2::new(1.0, 1.0),
            radians: 0.0,
            transform_matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            update_required: false,
        }
    }

    pub fn transformation_matrix(&mut self) -> &[f32; 6] {
        if self.update_required {
            self.update();
        }
        &self.transform_matrix
    }
    pub fn up(&self) -> Vec2{
        let rad = self.radians - PI/4.0;
        vec2(rad.sin()+rad.cos(), rad.cos() - rad.sin())
    }
    pub fn right(&self) -> Vec2{
        let rad = self.radians + PI/4.0;
        vec2(rad.sin()+rad.cos(), rad.cos() - rad.sin())
    }
    pub fn calculate_hash(&self) -> i32 {
        let data: &[i32] = unsafe {
            std::slice::from_raw_parts(
                self.transform_matrix.as_ptr() as *const i32,
                6
            )
        };
        
        let mut hash = 0;
        for &value in data {
            hash += value;
            hash += hash << 10;
            hash ^= hash >> 6;
        }
        hash += hash << 3;
        hash ^= hash >> 11;
        hash += hash << 15;
        hash
    }

    fn update(&mut self) {
        self.update_required = false;
        let epsilon = 0.1 * (PI / 180.0);

        let (cos, sin) = if (self.radians).abs() < epsilon {
            // Exact case for zero rotation
            (1.0, 0.0)
        } else if (self.radians.abs() - PI).abs() < epsilon {
            // Exact case for 180 degree rotation
            (-1.0, 0.0)
        } else if (self.radians.abs() - PI / 2.0).abs() < epsilon {
            // Exact case for 90 or 270 degree rotation
            (0.0, if self.radians < 0.0 { -1.0 } else { 1.0 })
        } else {
            // Arbitrary rotation
            (self.radians.cos(), self.radians.sin())
        };

        self.transform_matrix[0] = cos * self.scale.x;
        self.transform_matrix[1] = sin * self.scale.x;
        self.transform_matrix[2] = -sin * self.scale.y;
        self.transform_matrix[3] = cos * self.scale.y;
    }
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.transform_matrix[4], self.transform_matrix[5])
    }
    pub fn set_position(&mut self, pos: Vec2) {
        self.update_required |= (self.transform_matrix[4] != pos.x) || (self.transform_matrix[5] != pos.y);
        self.transform_matrix[4] = pos.x;
        self.transform_matrix[5] = pos.y;
    }
    pub fn scale(&self) -> Vec2 {
        self.scale
    }
    pub fn set_scale(&mut self, scale: Vec2) {
        self.update_required |= self.scale != scale;
        self.scale = scale;
    }
    pub fn radians(&self) -> f32 {
        self.radians
    }
    pub fn set_radians(&mut self, radians: f32) {
        let normalized_radians = radians % (2.0 * PI);
        self.update_required |= self.radians != normalized_radians;
        self.radians = normalized_radians;
    }
    pub fn angle(&self) -> f32 {
        (self.radians / PI) * 180.0
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.set_radians((angle / 180.0) * PI);
    }
}