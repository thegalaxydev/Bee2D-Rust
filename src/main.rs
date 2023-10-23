use mlua::prelude::*;
use std::thread::sleep;
use raylib::prelude::*;
use std::time::Duration;

fn lua_wait_func(_lua: &Lua, seconds: LuaNumber) -> Result<(), LuaError> {

    if seconds == 0.0 {
        sleep(Duration::from_secs_f32(1.0/60.0));
        return Ok(())
    }

    sleep(Duration::from_secs(seconds as u64));
    
    Ok(())
}

fn lua_set_window_properties(_lua: &Lua, props: ) -> Result<(), LuaError> {

}

fn lua_clear_background(rl: &RaylibHandle) {

}

fn main() -> LuaResult<()> {
    let (rl, thread) = raylib::init()
        .size(800, 800)
        .title("Bee2D")
        .build();

	let lua: Lua = Lua::new();

	let bee2d = lua.create_table()?;

	lua.globals().set("Bee2D", bee2d)?;
    lua.globals().set("wait", lua.create_function(|lua: &Lua,  seconds: LuaNumber| {
        lua_wait_func(lua, seconds)
    })?)?;

	let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a path to the Lua script.");
        std::process::exit(1);
    }

    let script_path = &args[1];
    let script_content = std::fs::read_to_string(script_path)?;
    
    Ok(lua.load(&script_content).exec()?)

}