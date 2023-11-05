use mlua::prelude::*;
use std::thread::sleep;
use raylib::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::time::Instant;
use std::collections::HashMap;


struct bee2d {
    lua: Lua,
    raylib: RaylibHandle,
    thread: RaylibThread,

}  