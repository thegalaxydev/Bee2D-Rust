pub mod matrix3;
pub use matrix3::Matrix3;

pub mod vector2;
pub use vector2::Vector2;

use mlua::prelude::*;

use crate::lune::table_builder::TableBuilder;
use crate::lune::exports::export;

fn create_all_exports(lua: &Lua) -> LuaResult<Vec<(&'static str, LuaValue)>> {

    Ok(vec![
        export::< Matrix3>(lua)?,
        export::< Vector2>(lua)?,
    ])
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = create_all_exports(lua)?;
    TableBuilder::new(lua)?
        .with_values(exports)?
        .build_readonly()
}