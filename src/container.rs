use std::{cell::RefCell, rc::Rc};

use crate::{resource_manager::ResourceManager, transform::Transform2D};

pub struct Container2D {
    transform: Transform2D,
    modules: Vec<Box<dyn Module2D>>,
    resource_manager: Rc<RefCell<ResourceManager>>,
    
    parent: usize,
    childerns: Vec<usize>,
}

impl Container2D {
    pub fn empty(resource_manager: Rc<RefCell<ResourceManager>>) -> Self {
        Self { transform : Transform2D::default(), modules : Vec::new(), resource_manager, childerns: Vec::new(), parent: 0 }
    }
    pub fn on_update(&mut self){

        for x in &mut self.modules {
            x.on_update();
        }
    }
}
pub trait Module2D{
    fn on_update(&mut self){}
    /// Only to preform thread safe operations
    fn on_paralel_update(&mut self){}
    fn on_render(&mut self){}
    fn on_delete(&mut self){}
}

