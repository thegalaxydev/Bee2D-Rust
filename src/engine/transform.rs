use core::fmt;
use std::ops;

use std::rc::Rc;
use std::cell::RefCell;

use mlua::prelude::*;
use crate::lune::table_builder::*;
use crate::lune::exports::*;
use crate::lune::userdata::*;

use crate::math::matrix3::Matrix3;
use crate::math::vector2::Vector2;

use crate::engine::gameobject::GameObject;

#[derive(Clone)]
pub struct Transform {
    parent: Option<Rc<RefCell<Transform>>>,
    children: Vec<Rc<RefCell<Transform>>>,
    owner: Rc<RefCell<GameObject>>,
    local_matrix: Matrix3,
    global_matrix: Matrix3,
    local_rotation: Matrix3,
    local_translation: Matrix3,
    local_scale: Matrix3,
    local_rotation_angle: f32,
}

impl Transform {
    pub fn new(owner: &GameObject) -> Transform {
        Transform {
            parent: None,
            children: Vec::new(),
            owner: Rc::new(RefCell::new(owner.clone())),
            local_matrix: Matrix3::identity(),
            global_matrix: Matrix3::identity(),
            local_rotation: Matrix3::identity(),
            local_translation: Matrix3::identity(),
            local_scale: Matrix3::identity(),
            local_rotation_angle: 0.0,
        }
    }
}

impl LuaExportsTable<'_> for Transform {
    const EXPORT_NAME: &'static str = "Transform";



    fn create_exports_table(lua: &Lua) -> LuaResult<LuaTable> {
        let transform_new = |_, owner: LuaUserDataRef<GameObject>| {
            Ok(Transform {
                parent: None,
                children: Vec::new(),
                owner: Rc::new(RefCell::new(owner.clone())),
                local_matrix: Matrix3::identity(),
                global_matrix: Matrix3::identity(),
                local_rotation: Matrix3::identity(),
                local_translation: Matrix3::identity(),
                local_scale: Matrix3::identity(),
                local_rotation_angle: 0.0,
            })
        };

        TableBuilder::new(lua)?
            .with_function("new", transform_new)?
            .build_readonly()
    }
}

impl LuaUserData for Transform {



}

