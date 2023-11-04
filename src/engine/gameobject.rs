use core::fmt;
use std::ops;

use mlua::prelude::*;
use crate::lune::table_builder::*;
use crate::lune::exports::*;
use crate::lune::userdata::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::transform::Transform;
use crate::engine::component::*;

#[derive(Clone)]
pub struct GameObject {
    transform: Transform,
    components: Vec<Box<ComponentRef>>,

}

impl LuaExportsTable<'_> for GameObject {
    const EXPORT_NAME: &'static str = "GameObject";

    fn create_exports_table(lua: &Lua) -> LuaResult<LuaTable> {
        TableBuilder::new(lua)?
            .build_readonly()
    }
}

impl LuaUserData for GameObject {



}