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
    local_matrix: Matrix3,
    global_matrix: Matrix3,
    local_rotation: Matrix3,
    local_rotation_angle: f32,
    local_translation: Matrix3,
    local_scale: Matrix3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            parent: None,
            children: Vec::new(),
            local_matrix: Matrix3::identity(),
            global_matrix: Matrix3::identity(),
            local_rotation: Matrix3::identity(),
            local_rotation_angle: 0.0,
            local_translation: Matrix3::identity(),
            local_scale: Matrix3::identity(),
        }
    }

    pub fn update_transform(&mut self) {
        self.local_matrix = self.local_rotation * self.local_translation * self.local_scale;

        if let Some(parent) = &self.parent {
            self.global_matrix = parent.borrow().global_matrix * self.local_matrix;
        } else {
            self.global_matrix = self.local_matrix;
        }
    }
}

impl LuaExportsTable<'_> for Transform {
    const EXPORT_NAME: &'static str = "Transform";



    fn create_exports_table(lua: &Lua) -> LuaResult<LuaTable> {
        TableBuilder::new(lua)?
            .build_readonly()
    }
}

impl LuaUserData for Transform {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Parent", |_, this| Ok(this.parent.clone()));
        fields.add_field_method_get("Children", |_, this| Ok(this.children.clone()));
        fields.add_field_method_get("LocalMatrix", |_, this| Ok(this.local_matrix));
        fields.add_field_method_get("GlobalMatrix", |_, this| Ok(this.global_matrix));
        fields.add_field_method_get("LocalRotationAngle", |_, this| Ok(this.local_rotation_angle));

        fields.add_field_method_set("Parent", |_, this, parent: LuaUserDataRef<Transform>| {
            this.parent = Some(Rc::new(RefCell::new(parent.clone())));
            this.update_transform();
            Ok(())
        });

        fields.add_field_method_get("LocalRotation", |_, this|
            Ok(this.local_rotation)
        );
        fields.add_field_method_get("LocalPosition", |_, this| {
            Ok(Vector2::new(this.local_translation.m02, this.local_translation.m12))
            }
        );
        fields.add_field_method_get("LocalScale", |_, this| {
            Ok(Vector2::new(this.local_scale.m00, this.local_scale.m11))
        });

        fields.add_field_method_get("GlobalRotation", |_, this| {
            Ok(this.global_matrix.m10.atan2(this.global_matrix.m00))
        });

        fields.add_field_method_get("GlobalPosition", |_, this| {
            Ok(Vector2::new(this.global_matrix.m02, this.global_matrix.m12))
        });

        fields.add_field_method_get("GlobalScale", |_, this| {
            Ok(Vector2::new(this.global_matrix.m00, this.global_matrix.m11))
        });

        fields.add_field_method_set("LocalRotation", |_, this, m: LuaUserDataRef<Matrix3>| {
            this.local_rotation = *m;
            this.local_rotation_angle = this.local_rotation.m10.atan2(this.local_rotation.m00);
            this.update_transform();
            Ok(())
        });

        fields.add_field_method_set("LocalPosition", |_, this, v : LuaUserDataRef<Vector2>| {
            this.local_translation = Matrix3 {
                m00: 1.0, m01: 0.0, m02: v.get_x(),
                m10: 0.0, m11: 1.0, m12: v.get_y(),
                m20: 0.0, m21: 0.0, m22: 1.0,
            };
            this.update_transform();
            Ok(())
        });

        fields.add_field_method_set("LocalScale", |_, this, v : LuaUserDataRef<Vector2>| {
            this.local_scale = Matrix3 {
                m00: v.get_x(), m01: 0.0, m02: 0.0,
                m10: 0.0, m11: v.get_y(), m12: 0.0,
                m20: 0.0, m21: 0.0, m22: 1.0,
            };
            this.update_transform();
            Ok(())
        });
        
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, userdata_impl_to_string);
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LocalMatrix: {},\nGlobalMatrix: {},\nLocalRotationAngle: {}", self.local_matrix, self.global_matrix, self.local_rotation_angle)
    }
}

