use mlua::prelude::*;
use raylib::prelude::*;


struct LuauInterface {
    lua: Lua,

    pub draw_callbacks: LuaTable,
    pub update_callbacks: LuaTable,
    pub start_callbacks: LuaTable,

    pub draw_storage: LuaTable,
    pub texture_storage: LuaTable,

    pub texture_load_cache: LuaTable,
}

impl LuauInterface {
    pub fn new() -> Result<Self, LuaError> {
        self.lua = Lua::new();

        self.draw_callbacks = self.lua.create_table()?;
        self.update_callbacks = self.lua.create_table()?;
        self.start_callbacks = self.lua.create_table()?;
        self.draw_storage = self.lua.create_table()?;
        self.texture_storage = self.lua.create_table()?;
        self.texture_load_cache = self.lua.create_table()?;

        Ok(Self {
            lua: self.lua,
            draw_callbacks: self.draw_callbacks,
            update_callbacks: self.update_callbacks,
            start_callbacks: self.start_callbacks,
            draw_storage: self.draw_storage,
            texture_storage: self.texture_storage,
            texture_load_cache: self.texture_load_cache,
        })
    }

    pub fn run(raylib: &mut RaylibHandle, thread: RaylibThread, script_content: String) -> Result<(), LuaError> {
        

    }
}