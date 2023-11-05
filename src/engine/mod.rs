pub mod gameobject;
pub use gameobject::GameObject;

pub mod transform;
pub use transform::Transform;


use mlua::prelude::*;

use crate::lune::table_builder::TableBuilder;
use crate::lune::exports::export;

fn create_all_exports(lua: &Lua) -> LuaResult<Vec<(&'static str, LuaValue)>> {

    Ok(vec![
        export::<GameObject>(lua)?,
        export::<Transform>(lua)?,
    ])
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = create_all_exports(lua)?;
    TableBuilder::new(lua)?
        .with_values(exports)?
        .build_readonly()
}