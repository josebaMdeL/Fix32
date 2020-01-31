// imports
pub mod internal_constants;

use std::ops::{Add, Sub, Mul, Div};

#[derive(Eq, PartialEq, Clone, Copy, Debug, Ord, PartialOrd)]
// Basic class
pub struct Fix32 {
    pub n: i32,
}

// Public implementations
impl Fix32 {
    pub fn new() -> Fix32 {
        Fix32 { n: 0 }
    }

    pub fn from_i32(f: i32) -> Fix32 {
        Fix32 {
            n: limit((f * internal_constants::SU_I32) as i64),
        }
    }

    pub fn from_i64(f: i64) -> Fix32 {
        Fix32 {
            n: limit(f * internal_constants::SU_I64),
        }
    }

    pub fn from_f32(f: f32) -> Fix32 {
        Fix32 {
            n: limit((f * internal_constants::SU_F32) as i64),
        }
    }

    pub fn from_f64(f: f64) -> Fix32 {
        Fix32 {
            n: limit((f * internal_constants::SU_F64) as i64),
        }
    }

    /// returns Fix32 with internal integer set to f
    pub fn with_n(f: i32) -> Fix32 {
        Fix32 { n: f }
    }

    pub fn get_f32(self) -> f32 {
        internal_constants::SD_F32 * (self.n as f32)
    }

    pub fn get_f64(self) -> f64 {
        internal_constants::SD_F64 * (self.n as f64)
    }

    pub fn get_str(self) -> String {
        ((self.n as f64 ) * internal_constants::SD_F64).to_string()
    }

    pub fn get_i32(self) -> i32 {
        self.n/internal_constants::SU_I32
    }

    pub fn get_rem(self) -> i32 {
        self.n - self.get_i32()*internal_constants::SU_I32
    }

    pub fn trunc(&mut self) {
        self.n -= self.get_rem();
    }

    pub fn abs(self) -> Fix32 {
        Fix32 {
            n: self.n.abs()
        }
    }

    pub fn pow2(self) -> Fix32 {
        self*self
    }

   pub fn pow3(self) -> Fix32 {
        self*self*self
    }
}

// Public traits: Operator Overloadings
impl Add for Fix32 {
    type Output = Fix32;

    fn add(self, other: Fix32) -> Fix32 {
        Fix32{n: limit((self.n as i64)+(other.n as i64))}
    }
}

impl Sub for Fix32 {
    type Output = Fix32;

    fn sub(self, other: Fix32) -> Fix32 {
        Fix32{n: limit((self.n as i64)-(other.n as i64))}
    }
}

impl Mul for Fix32 {
    type Output = Fix32;

    fn mul(self, other: Fix32) -> Fix32 {
        Fix32{n: limit((self.n as i64)*(other.n as i64)/internal_constants::SU_I64)}
    }
}

impl Div for Fix32 {
    type Output = Fix32;

    fn div(self, other: Fix32) -> Fix32 {
        if other.n == 0_i32 {
            if self.n > 0 {
                Fix32 {n: internal_constants::MAX_I32}
            } else {
                Fix32 {n: internal_constants::MIN_I32}
            }
        } else {
            Fix32{n: limit((internal_constants::SU_I64*self.n as i64)/other.n as i64)}
        }
    }
}

use std::ops::Rem;

impl Rem for Fix32 {
    type Output = Fix32;

    fn rem(self, other: Fix32) -> Fix32 {
        Fix32{n: self.n % other.n}
    }
}


// Private implementations or auxiliar functions

// Input a long and returns the limited int value (overflow protection)
// make only upper and lower limiter functions to improve performance on some operations
fn limit(l: i64) -> i32 {
    if l > internal_constants::MAX_I64 {
        internal_constants::MAX_I32
    } else if l < internal_constants::MIN_I64 {
        internal_constants::MIN_I32
    } else {
        l as i32
    }
}

fn limitUp(l: i64) -> i32 {
    if l > internal_constants::MAX_I64 {
        internal_constants::MAX_I32
    } else {
        l as i32
    }
}

fn limitDown(l: i64) -> i32 {
    if l < internal_constants::MIN_I64 {
        internal_constants::MIN_I32
    } else {
        l as i32
    }
}
