// Numeric constants and configuration

// decimal places for our numbers
pub const DEC_I32: i32 = 3;
pub const DEC_F32: f32 = DEC_I32 as f32;
pub const DEC_F64: f64 = DEC_I32 as f64;

// 10 number will be tha base for our maths!
pub const B_I32: i32 = 10;
pub const B_I64: i64 = 10;
pub const B_F32: f32 = 10.0;
pub const B_F64: f64 = 10.0;

// Scaling constants: Scale Up (SU) & Scale Dows (SD)
// pub const SU_I32: i32 = B_I32.pow(DEC_I32 as u32);
// pub const SU_I64: i64 = B_I64.pow(DEC_I32 as u32);
// pub const SU_F32: f32 = B_F32.powi(DEC_I32);
// pub const SU_F64: f64 = B_F64.powi(DEC_I32);
// pub const SD_F32: f32 = B_F32.powi(-DEC_I32);
// pub const SD_F64: f64 = B_F64.powi(-DEC_I32);
// !!! i will fix this values for the moment, as no constexpr are supported jet
pub const SU_I32: i32 = 1000;
pub const SU_I64: i64 = 1000;
pub const SU_F32: f32 = 1000.0;
pub const SU_F64: f64 = 1000.0;
pub const SD_F32: f32 = 0.001;
pub const SD_F64: f64 = 0.001;

// Numeric limits
pub const MAX_I32: i32 = i32::max_value();
pub const MIN_I32: i32 = i32::min_value() + 1; // +1 to center 0 value ;)
pub const MAX_I64: i64 = i32::max_value() as i64;
pub const MIN_I64: i64 = i32::min_value() as i64 + 1; // +1 to center 0 value ;)

// Mathematical numeric constants
// pub const PI_F64: f64 = f64::constants::PI;
// pub const PI_F32: f32 = f32::constants::PI;
