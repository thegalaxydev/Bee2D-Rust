use std::any::{Any, TypeId};
use crate::engine::gameobject::GameObject;

use std::rc::Rc;
use std::cell::RefCell;

pub trait Component: Any {
    fn start(&mut self);
    fn update(&mut self);
    fn draw(&mut self);

    fn box_clone(&self) -> Box<dyn Component>;
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct ComponentRef {
    pub owner: Rc<RefCell<GameObject>>,
    pub component: Box<dyn Component>,
}

impl ComponentRef {
    pub fn new(owner: Rc<RefCell<GameObject>>, component: Box<dyn Component>) -> ComponentRef {
        ComponentRef {
            owner,
            component,
        }
    }
}



