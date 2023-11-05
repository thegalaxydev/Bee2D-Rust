use core::fmt;
use std::ops;

use mlua::prelude::*;
use crate::lune::table_builder::*;
use crate::lune::exports::*;
use crate::lune::userdata::*;

use crate::engine::transform::Transform;

use std::rc::Rc;
use std::cell::RefCell;




pub struct GameObject {
    pub name: String,
    pub transform: Transform,
    

}

impl LuaExportsTable<'_> for GameObject {
    const EXPORT_NAME: &'static str = "GameObject";

    fn create_exports_table(lua: &Lua) -> LuaResult<LuaTable> {
        let gameobject_new = lua.create_function(|lua_ctx, name: String| {
            let gameobject = GameObject {
                name: name,
                transform: Transform::new()
            };

            Ok(gameobject)
        })?;

        let exports = lua.create_table()?;
        exports.set("new", gameobject_new)?;

        Ok(exports)
    }
}

impl LuaUserData for GameObject {

    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_set("Name", |_, this, name: String| {
            this.name = name;
            Ok(())
        });

        fields.add_field_method_get("Transform", |_, this| Ok(this.transform.clone()));
    }

}