use core::fmt;
use std::ops;

use mlua::prelude::*;
use crate::lune::table_builder::*;
use crate::lune::exports::*;
use crate::lune::userdata::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub m00: f32, pub m01: f32, pub m02: f32,
    pub m10: f32, pub m11: f32, pub m12: f32,
    pub m20: f32, pub m21: f32, pub m22: f32,
}

impl Matrix3 {
    pub fn identity() -> Matrix3 {
        Matrix3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        }
    }

    // returns a base rotation matrix 
    pub fn get_rotation(&self) -> Matrix3 {
        Matrix3 {
            m00: 0.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 0.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 0.0,

        }
    }
    
}

impl LuaExportsTable<'_> for Matrix3 {
    const EXPORT_NAME: &'static str = "Matrix3";

    fn create_exports_table(lua: &Lua) -> LuaResult<LuaTable> {
        let matrix3_new = |_, (m00, m01, m02, m10, m11, m12, m20, m21, m22): (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>)| {
            Ok(Matrix3 {
                m00: m00.unwrap_or(0.0), m01: m01.unwrap_or(0.0), m02: m02.unwrap_or(0.0),
                m10: m10.unwrap_or(0.0), m11: m11.unwrap_or(0.0), m12: m12.unwrap_or(0.0),
                m20: m20.unwrap_or(0.0), m21: m21.unwrap_or(0.0), m22: m22.unwrap_or(0.0),
            })
        };
        
        let matrix3_rotation = |_, angle: Option<f32>| {
            let angle = angle.unwrap_or(0.0);
            Ok(Matrix3 {
                m00: angle.cos(), m01: -angle.sin(), m02: 0.0,
                m10: angle.sin(), m11: angle.cos(), m12: 0.0,
                m20: 0.0, m21: 0.0, m22: 1.0,
            })
        };

        let matrix3_translation = |_, (x, y): (Option<f32>, Option<f32>)| {
            Ok(Matrix3 {
                m00: 1.0, m01: 0.0, m02: x.unwrap_or(0.0),
                m10: 0.0, m11: 1.0, m12: y.unwrap_or(0.0),
                m20: 0.0, m21: 0.0, m22: 1.0,
            })
        };

        let matrix3_scale = |_, (x, y): (Option<f32>, Option<f32>)| {
            Ok(Matrix3 {
                m00: x.unwrap_or(1.0), m01: 0.0, m02: 0.0,
                m10: 0.0, m11: y.unwrap_or(1.0), m12: 0.0,
                m20: 0.0, m21: 0.0, m22: 1.0,
            })
        };

        let matrix3_lerp = |_, (a, b, t): (LuaUserDataRef<Matrix3>, LuaUserDataRef<Matrix3>, f32)| {
            Ok(Matrix3 {
                m00: a.m00 + (b.m00 - a.m00) * t, m01: a.m01 + (b.m01 - a.m01) * t, m02: a.m02 + (b.m02 - a.m02) * t,
                m10: a.m10 + (b.m10 - a.m10) * t, m11: a.m11 + (b.m11 - a.m11) * t, m12: a.m12 + (b.m12 - a.m12) * t,
                m20: a.m20 + (b.m20 - a.m20) * t, m21: a.m21 + (b.m21 - a.m21) * t, m22: a.m22 + (b.m22 - a.m22) * t,
            })
        };

        TableBuilder::new(lua)?
            .with_function("new", matrix3_new)?
            .with_function("fromRotationXYZ", matrix3_rotation)?
            .with_function("fromTranslation", matrix3_translation)?
            .with_function("fromScale", matrix3_scale)?
            .with_function("lerp", matrix3_lerp)?
            .with_value("identity", Matrix3 {
                m00: 1.0, m01: 0.0, m02: 0.0,
                m10: 0.0, m11: 1.0, m12: 0.0,
                m20: 0.0, m21: 0.0, m22: 1.0,
            })?
            .build_readonly()
    }
}

impl LuaUserData for Matrix3 {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("M00", |_, this| Ok(this.m00));
        fields.add_field_method_get("M01", |_, this| Ok(this.m01));
        fields.add_field_method_get("M02", |_, this| Ok(this.m02));
        fields.add_field_method_get("M10", |_, this| Ok(this.m10));
        fields.add_field_method_get("M11", |_, this| Ok(this.m11));
        fields.add_field_method_get("M12", |_, this| Ok(this.m12));
        fields.add_field_method_get("M20", |_, this| Ok(this.m20));
        fields.add_field_method_get("M21", |_, this| Ok(this.m21));
        fields.add_field_method_get("M22", |_, this| Ok(this.m22));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("Lerp", |_, this, (other, t) : (LuaUserDataRef<Matrix3>, f32)| {
            Ok(Matrix3 {
                m00: this.m00 + (other.m00 - this.m00) * t, m01: this.m01 + (other.m01 - this.m01) * t, m02: this.m02 + (other.m02 - this.m02) * t,
                m10: this.m10 + (other.m10 - this.m10) * t, m11: this.m11 + (other.m11 - this.m11) * t, m12: this.m12 + (other.m12 - this.m12) * t,
                m20: this.m20 + (other.m20 - this.m20) * t, m21: this.m21 + (other.m21 - this.m21) * t, m22: this.m22 + (other.m22 - this.m22) * t,
            })
        });

        methods.add_meta_method(LuaMetaMethod::Eq, userdata_impl_eq);
        methods.add_meta_method(LuaMetaMethod::ToString, userdata_impl_to_string);
        methods.add_meta_method(LuaMetaMethod::Add, userdata_impl_add);
        methods.add_meta_method(LuaMetaMethod::Sub, userdata_impl_sub);
        methods.add_meta_method(LuaMetaMethod::Mul, userdata_impl_mul);
    }
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix3 {{\n\t{}, {}, {},\n\t{}, {}, {},\n\t{}, {}, {}\n}}", self.m00, self.m01, self.m02, self.m10, self.m11, self.m12, self.m20, self.m21, self.m22)
    }
}

impl ops::Add for Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Matrix3) -> Matrix3 {
        Matrix3 {
            m00: self.m00 + rhs.m00, m01: self.m01 + rhs.m01, m02: self.m02 + rhs.m02,
            m10: self.m10 + rhs.m10, m11: self.m11 + rhs.m11, m12: self.m12 + rhs.m12,
            m20: self.m20 + rhs.m20, m21: self.m21 + rhs.m21, m22: self.m22 + rhs.m22,
        }
    }
}

impl ops::Sub for Matrix3 {
    type Output = Matrix3;

    fn sub(self, rhs: Matrix3) -> Matrix3 {
        Matrix3 {
            m00: self.m00 - rhs.m00, m01: self.m01 - rhs.m01, m02: self.m02 - rhs.m02,
            m10: self.m10 - rhs.m10, m11: self.m11 - rhs.m11, m12: self.m12 - rhs.m12,
            m20: self.m20 - rhs.m20, m21: self.m21 - rhs.m21, m22: self.m22 - rhs.m22,
        }
    }
}

impl ops::Mul for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Matrix3 {
        let m00 = self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20;
        let m01 = self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21;
        let m02 = self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22;

        let m10 = self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20;
        let m11 = self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21;
        let m12 = self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22;

        let m20 = self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20;
        let m21 = self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21;
        let m22 = self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22;

        Matrix3 {
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        }
    }
}