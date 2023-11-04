use mlua::prelude::*;
use std::thread::sleep;
use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::time::Instant;
use std::collections::HashMap;

mod math;
mod lune;
mod engine;


fn lua_wait_func(_lua: &Lua, seconds: LuaNumber) -> Result<(), LuaError> {

	if seconds == 0.0 {
		sleep(Duration::from_secs_f32(1.0/60.0));
		return Ok(())
	}

	sleep(Duration::from_secs(seconds as u64));
	
	Ok(())
}

struct Bee2D {

}

// we start by modeling our data, not our behavior.

fn start() {
	
}

fn update() {

}

fn draw() {

}




fn run(raylib: &mut RaylibHandle, thread: RaylibThread, lua: &Lua, script_content: String) -> Result<(), LuaError> {

	lua.load(&script_content).exec()?;

	let globals = lua.globals();

	let start_callbacks: LuaTable = globals.get("start_callbacks")?;
	for pair in start_callbacks.pairs::<LuaNumber, LuaFunction>() {
		let (_, func) = pair?;
		func.call(())?;
	}

	// could we *please* rewrite your code?
	// this is insanely messy.
	let mut texture_cache: HashMap<LuaString, Texture2D> = HashMap::new();

	let texture_load_cache: LuaTable = globals.get("texture_load_cache")?;
	for pair in texture_load_cache.pairs::<LuaNumber, LuaString>() {
		let (_, tex_str) = pair?;

		let texture: Texture2D = raylib.load_texture(&thread, tex_str.to_str()?).unwrap();

		texture_cache.insert(tex_str, texture); 
	}

	let mut last_time = Instant::now();

	while !raylib.window_should_close() {
		let height: LuaNumber = globals.get("_bee2dHeight")?;
		let width: LuaNumber = globals.get("_bee2dWidth")?;
		let title: LuaString = globals.get("_bee2dTitle")?;

		let bee2d: LuaTable = globals.get("Bee2D")?;

		let bee2d_height: LuaNumber = bee2d.get("height")?;
		let bee2d_width: LuaNumber = bee2d.get("width")?;
		let bee2d_title: LuaString = bee2d.get("title")?;

		if bee2d_height != height || bee2d_width != width {
			raylib.set_window_size(width as i32, height as i32);
			
			bee2d.set("height", height)?;
			bee2d.set("width", width)?;
			
		}

		if bee2d_title != title {
			raylib.set_window_title(&thread, title.to_str()?);

			bee2d.set("title", title)?;
		}

		let current_time = Instant::now();
		let delta_time = current_time.duration_since(last_time);
		last_time = current_time;
		
		let update_callbacks: LuaTable = globals.get("update_callbacks")?;
		let draw_callbacks: LuaTable = globals.get("draw_callbacks")?;

		bee2d.set("deltaTime", delta_time.as_secs_f64() as LuaNumber)?;

		lua.globals().set("Bee2D", bee2d)?;

		for pair in update_callbacks.pairs::<LuaNumber, LuaFunction>() {
			let (_, func) = pair?;
			func.call(delta_time.as_secs_f64() as LuaNumber)?;
		}


		for pair in draw_callbacks.pairs::<LuaNumber, LuaFunction>() {
			let (_, func) = pair?;
			func.call(())?;
		}

		let global_draw_storage: LuaTable = globals.get("global_draw_storage")?;
		let global_tex_storage: LuaTable = globals.get("global_tex_storage")?;

		let mut draw_handle: RaylibDrawHandle<'_> = raylib.begin_drawing(&thread);
		draw_handle.clear_background(Color::BLACK);

		for pair in global_tex_storage.pairs::<LuaNumber, LuaTable>() {
			let (key, tex_info) = pair?;
			let tex_str: LuaString = tex_info.get("texture")?;

			if texture_cache.get(&tex_str).is_none() {
				continue;
			}

			if let Some(texture) = texture_cache.get(&tex_str) {
				// texture is a Texture2D here, not the option, because this code only runs when the pattern on the 'let'
				// matches.

				// put your code in here.

				let x: LuaNumber = tex_info.get("x")?;
				let y: LuaNumber = tex_info.get("y")?;
				let scale = tex_info.get("scale")?;
				let rotation = tex_info.get("rotation")?;
				let color: LuaTable = tex_info.get("color")?;

				let r: LuaNumber = color.get(1)?;
				let g: LuaNumber = color.get(2)?;
				let b: LuaNumber = color.get(3)?;
				let a: LuaNumber = color.get(4)?;

				let new_color = Color::new(r as u8, g as u8, b as u8, a as u8);
				
				let position : &mut Vector2 = &mut Vector2::new(x as f32,y as f32);

				

				draw_handle.draw_texture_ex(texture, *position, rotation, scale, new_color)
			}

		}

		for pair in global_draw_storage.pairs::<LuaNumber, LuaTable>() {
			let (key, shape) = pair?;
			let type_of_shape: LuaString = shape.get("type")?;

			if type_of_shape == lua.create_string("rectangle")? {
				let height: LuaNumber = shape.get("height")?;
				let width: LuaNumber = shape.get("width")?;
				let x: LuaNumber = shape.get("x")?;
				let y: LuaNumber = shape.get("y")?;
				let color: LuaTable = shape.get("color")?;

				let r: LuaNumber = color.get(1)?;
				let g: LuaNumber = color.get(2)?;
				let b: LuaNumber = color.get(3)?;
				let a: LuaNumber = color.get(4)?;

				let new_color = Color::new(r as u8, g as u8, b as u8, a as u8);

				draw_handle.draw_rectangle(x as i32, y as i32, width as i32, height as i32, new_color);
			}

			
			
		}

		
		
		
	}
    Ok(())
}

fn main() -> LuaResult<()> {
	let (raylib, thread) = raylib::init()
		.size(800, 800)
		.title("Bee2D")
		.build();

	let raylib = Rc::new(RefCell::new(raylib));

	let lua: Lua = Lua::new();
	
	let bee2d = lua.create_table()?;

	bee2d.set("GLOBAL_STORAGE", lua.create_table()?)?;

	lua.globals().set("_bee2dHeight", 800)?;
	lua.globals().set("_bee2dWidth", 800)?;
	lua.globals().set("_bee2dTitle", "Bee2D")?;

	bee2d.set("height", 800)?;
	bee2d.set("width", 800)?;
	bee2d.set("title", "Bee2D")?;

	for pair in math::module(&lua)?.pairs::<LuaString, LuaTable>() {
		let (key, value) = pair?;
		lua.globals().set(key, value)?;
	}

	lua.globals().set("update_callbacks", lua.create_table()?)?;
	lua.globals().set("draw_callbacks", lua.create_table()?)?;
	lua.globals().set("start_callbacks", lua.create_table()?)?;	
	lua.globals().set("global_draw_storage", lua.create_table()?)?;
	lua.globals().set("global_tex_storage", lua.create_table()?)?;
	lua.globals().set("texture_load_cache", lua.create_table()?)?;

	bee2d.set("bindToStart", lua.create_function_mut({
		|_lua: &Lua, func: LuaFunction| { 
			let globals = _lua.globals();
			let start_callbacks: LuaTable = globals.get("start_callbacks")?;
			let next_index = start_callbacks.len()? + 1;
			start_callbacks.set(next_index, func)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;
	
	bee2d.set("bindToUpdate", lua.create_function_mut({
		|_lua: &Lua, func: LuaFunction| { 
			let globals = _lua.globals();
			let update_callbacks: LuaTable = globals.get("update_callbacks")?;
			let next_index = update_callbacks.len()? + 1;
			update_callbacks.set(next_index, func)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;

	bee2d.set("bindToDraw", lua.create_function_mut({
		|_lua: &Lua, func: LuaFunction| { 
			let globals = _lua.globals();
			let draw_callbacks: LuaTable = globals.get("draw_callbacks")?;
			let next_index = draw_callbacks.len()? + 1;
			draw_callbacks.set(next_index, func)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;

	bee2d.set("drawRectangle", lua.create_function_mut({

		|_lua: &Lua, (x,y,width,height, color) : (LuaNumber, LuaNumber, LuaNumber, LuaNumber, LuaTable)| { 
			let globals = _lua.globals();
			let global_draw_storage: LuaTable = globals.get("global_draw_storage")?;
			let next_index = global_draw_storage.len()? + 1;

			let rectangle = _lua.create_table()?;
			rectangle.set("x", x)?;
			rectangle.set("y", y)?;
			rectangle.set("width", width)?;
			rectangle.set("height", height)?;
			rectangle.set("color", color)?;
			rectangle.set("type", "rectangle")?;

			global_draw_storage.set(next_index, rectangle)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;

	bee2d.set("loadTexture", lua.create_function_mut({

		|_lua: &Lua, texturestr: LuaString| { 
			let globals = _lua.globals();
			let texture_load_cache: LuaTable = globals.get("texture_load_cache")?;
			let next_index = texture_load_cache.len()? + 1;

			texture_load_cache.set(next_index, texturestr)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;
		

	bee2d.set("drawTexture", lua.create_function_mut({

		|_lua: &Lua, (texturestr ,x,y, rotation, scale, color) : (LuaString, LuaNumber, LuaNumber,LuaNumber, LuaNumber, LuaTable)| { 
			let globals = _lua.globals();
			let global_tex_storage: LuaTable = globals.get("global_tex_storage")?;
			let next_index = global_tex_storage.len()? + 1;

			let texture = _lua.create_table()?;
			texture.set("x", x)?;
			texture.set("y", y)?;
			texture.set("rotation", rotation)?;
			texture.set("scale", scale)?;
			texture.set("color", color)?;
			texture.set("texture", texturestr)?;

			global_tex_storage.set(next_index, texture)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;
		


	bee2d.set("setHeight", lua.create_function_mut({
		|_lua: &Lua, num: LuaNumber| { 
			let globals = _lua.globals();
			
			globals.set("_bee2dHeight", num)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;

	bee2d.set("setWidth", lua.create_function_mut({
		|_lua: &Lua, num: LuaNumber| { 
			let globals = _lua.globals();
			
			globals.set("_bee2dWidth", num)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;

	bee2d.set("setTitle", lua.create_function_mut({
		|_lua: &Lua, string: LuaString| { 
			let globals = _lua.globals();
			
			globals.set("_bee2dTitle", string)?;

			Ok(())
		}
	}).expect("Failed to set global function"))?;

	lua.globals().set("Bee2D", bee2d)?;

	lua.globals().set("wait", lua.create_function(|_ctx: &Lua,  seconds: LuaNumber| {  // renamed lua to _ctx
		lua_wait_func(_ctx, seconds)
	})?)?;

	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("Please provide a path to the Lua script.");
		std::process::exit(1);
	}

	let script_path = &args[1];
	let script_content = std::fs::read_to_string(script_path)?;

	let result = run(&mut *raylib.borrow_mut(), thread, &lua, script_content);

	result

}