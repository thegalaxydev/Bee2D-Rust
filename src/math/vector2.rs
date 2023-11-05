use core::fmt;
use std::ops;

use mlua::Variadic;
use mlua::prelude::*;
use crate::lune::table_builder::*;
use crate::lune::exports::*;
use crate::lune::userdata::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl LuaExportsTable<'_> for Vector2 {
    const EXPORT_NAME: &'static str = "Vector2";

    fn create_exports_table(lua: &Lua) -> LuaResult<LuaTable> {
        let vector2_new = |_, (x, y): (Option<f32>, Option<f32>)| {
            Ok(Vector2 {
                x: x.unwrap_or(0.0), y: y.unwrap_or(0.0),
            })
        };
        
       

        TableBuilder::new(lua)?
            .with_function("new", vector2_new)?
            .with_value("one", Vector2{x: 1.0, y: 1.0})?
            .with_value("zero", Vector2{x: 0.0, y: 0.0})?
            .with_value("yAxis", Vector2{x: 0.0, y: 1.0})?
            .with_value("xAxis", Vector2{x: 1.0, y: 0.0})?
            .build_readonly()
    }
}

impl LuaUserData for Vector2 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, this| Ok(this.x));
        fields.add_field_method_get("Y", |_, this| Ok(this.y));
        fields.add_field_method_get("Magnitude", |_, this| Ok((this.x * this.x + this.y * this.y).sqrt()));
        fields.add_field_method_get("Unit", |_, this| 
            Ok(Vector2 {
                x: this.x / (this.x * this.x + this.y * this.y).sqrt(), 
                y: this.y / (this.x * this.x + this.y * this.y).sqrt()
            }));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("Lerp", |_, this, ( v, alpha): (LuaUserDataRef<Vector2>, f32)| {
            Ok(Vector2 {
                x: this.x + (v.x - this.x) * alpha, y: this.y + (v.y - this.y) * alpha
            })
        });

        methods.add_method("Dot", |_, this, v: LuaUserDataRef<Vector2>| {
            Ok(this.x * v.x + this.y * v.y)
        });

        methods.add_method("Cross", |_, this, v: LuaUserDataRef<Vector2>| {
            Ok(this.x * v.y - this.y * v.x)
        });

        methods.add_method("Distance", |_, this, v: LuaUserDataRef<Vector2>| {
            Ok(((this.x - v.x) * (this.x - v.x) + (this.y - v.y) * (this.y - v.y)).sqrt())
        });

        methods.add_method("Max", |_, this: &Vector2, args: Variadic::<LuaUserDataRef<Vector2>>| {
            let mut max_vector = *this;
            let mut max_magnitude = (this.x * this.x + this.y * this.y).sqrt();
        
            for vector in args.iter() {
                let magnitude = (vector.x * vector.x + vector.y * vector.y).sqrt();
                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                    max_vector = **vector;
                }
            }
        
            Ok(max_vector)
        });
        
        methods.add_method("Min", |_, this: &Vector2, args: Variadic::<LuaUserDataRef<Vector2>>| {
            let mut min_vector = *this;
            let mut min_magnitude = (this.x * this.x + this.y * this.y).sqrt();
        
            for vector in args.iter() {
                let magnitude = (vector.x * vector.x + vector.y * vector.y).sqrt();
                if magnitude < min_magnitude {
                    min_magnitude = magnitude;
                    min_vector = **vector;
                }
            }
        
            Ok(min_vector)
        });

        methods.add_meta_method(LuaMetaMethod::Eq, userdata_impl_eq);
        methods.add_meta_method(LuaMetaMethod::ToString, userdata_impl_to_string);
        methods.add_meta_method(LuaMetaMethod::Add, userdata_impl_add);
        methods.add_meta_method(LuaMetaMethod::Sub, userdata_impl_sub);

        methods.add_meta_method_mut(LuaMetaMethod::Mul, |lua, this, value: LuaValue| {
            match value {
                LuaValue::Number(n) => {
                    let factor = n as f32;
                    Ok(Vector2 {
                        x: this.x * factor,
                        y: this.y * factor,
                    })
                },
                LuaValue::UserData(ud) => {
                    if let Ok(rhs) = ud.borrow::<Vector2>() {
                        Ok(Vector2 {
                            x: this.x * rhs.x,
                            y: this.y * rhs.y,
                        })
                    } else {
                        Err(LuaError::FromLuaConversionError {
                            from: "UserData",
                            to: "Vector2",
                            message: Some("expected a Vector2".into()),
                        })
                    }
                },
                LuaValue::Integer(i) => {
                    let factor = i as f32;
                    Ok(Vector2 {
                        x: this.x * factor,
                        y: this.y * factor,
                    })
                },
                _ => {
                    Err(LuaError::FromLuaConversionError {
                        from: value.type_name(),
                        to: "f32 or Vector2",
                        message: None,
                    })
                }
            }
        });

        methods.add_meta_method_mut(LuaMetaMethod::Div, |lua, this, value: LuaValue| {
            match value {
                LuaValue::Number(n) => {
                    let divisor = n as f32;
                    if divisor != 0.0 {
                        Ok(Vector2 {
                            x: this.x / divisor,
                            y: this.y / divisor,
                        })
                    } else {
                        Err(LuaError::RuntimeError("attempt to divide by zero".into()))
                    }
                },
                LuaValue::Integer(i) => {
                    let factor = i as f32;
                    Ok(Vector2 {
                        x: this.x / factor,
                        y: this.y / factor,
                    })
                },
                LuaValue::UserData(ud) => {
                    if let Ok(rhs) = ud.borrow::<Vector2>() {
                        if rhs.x != 0.0 && rhs.y != 0.0 {
                            Ok(Vector2 {
                                x: this.x / rhs.x,
                                y: this.y / rhs.y,
                            })
                        } else {
                            Err(LuaError::RuntimeError("attempt to divide by zero in vector components".into()))
                        }
                    } else {
                        Err(LuaError::FromLuaConversionError {
                            from: "UserData",
                            to: "Vector2",
                            message: Some("expected a Vector2".into()),
                        })
                    }
                },
                _ => {
                    Err(LuaError::FromLuaConversionError {
                        from: "Number or UserData",
                        to: "Vector2 or Number",
                        message: None,
                    })
                }
            }
        });

    }
}


impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl ops::Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x, y: self.y + rhs.y
        }
    }
}

impl ops::Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x, y: self.y - rhs.y
        }
    }
}