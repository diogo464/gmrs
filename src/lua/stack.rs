use super::{LuaStateRaw, LuaType, Result};
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FromStackError {
    #[error(
        "Invalid argument at position {stack_pos}, expected {expected_type:?} found {found_type:?}"
    )]
    InvalidType {
        stack_pos: i32,
        expected_type: LuaType,
        found_type: LuaType,
    },
    #[error("Invalid string, {0}")]
    InvalidString(#[from] Utf8Error),
    #[error("Invalid userdata type")]
    InvalidUserdataType,
}

pub trait ToStack {
    /// returns how many values were pushed to the stack, usally just 1
    fn push(self, state: LuaStateRaw) -> i32;
}

pub trait FromStack: Sized {
    /// Returns the value and how many stack slots were used
    fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)>;
}

macro_rules! impl_number_stack_type {
    ($ty:ty) => {
        impl ToStack for $ty {
            fn push(self, state: LuaStateRaw) -> i32 {
                super::push_number(state, self as f64);
                1
            }
        }
        impl ToStack for &$ty {
            fn push(self, state: LuaStateRaw) -> i32 {
                super::push_number(state, *self as f64);
                1
            }
        }
        impl FromStack for $ty {
            fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)> {
                super::expect_type(state, stack_pos, LuaType::Number)?;
                Ok((super::get_number(state, stack_pos) as $ty, 1))
            }
        }
    };
}

impl_number_stack_type!(i8);
impl_number_stack_type!(u8);
impl_number_stack_type!(i16);
impl_number_stack_type!(u16);
impl_number_stack_type!(i32);
impl_number_stack_type!(u32);
impl_number_stack_type!(i64);
impl_number_stack_type!(u64);
impl_number_stack_type!(f32);
impl_number_stack_type!(f64);
impl_number_stack_type!(usize);
impl_number_stack_type!(isize);

impl ToStack for bool {
    fn push(self, state: LuaStateRaw) -> i32 {
        super::push_bool(state, self);
        1
    }
}

impl ToStack for &bool {
    fn push(self, state: LuaStateRaw) -> i32 {
        super::push_bool(state, *self);
        1
    }
}

impl FromStack for bool {
    fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)> {
        super::expect_type(state, stack_pos, LuaType::Bool)?;
        Ok((super::get_bool(state, stack_pos), 1))
    }
}

impl ToStack for &str {
    fn push(self, state: LuaStateRaw) -> i32 {
        super::push_string(state, self);
        1
    }
}

impl ToStack for String {
    fn push(self, state: LuaStateRaw) -> i32 {
        super::push_string(state, self.as_str());
        1
    }
}

impl FromStack for String {
    fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)> {
        super::expect_type(state, stack_pos, LuaType::String)?;
        Ok((super::get_string(state, stack_pos)?, 1))
    }
}

impl FromStack for Vec<u8> {
    fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)> {
        super::expect_type(state, stack_pos, LuaType::String)?;
        Ok((super::get_string_bytes(state, stack_pos), 1))
    }
}

impl ToStack for () {
    fn push(self, _state: LuaStateRaw) -> i32 {
        0
    }
}

impl FromStack for LuaStateRaw {
    fn from_stack(state: LuaStateRaw, _stack_pos: i32) -> Result<(Self, i32)> {
        Ok((state, 0))
    }
}

impl<T: ToStack> ToStack for Option<T> {
    fn push(self, state: LuaStateRaw) -> i32 {
        match self {
            Some(v) => v.push(state),
            None => {
                super::push_nil(state);
                1
            }
        }
    }
}
