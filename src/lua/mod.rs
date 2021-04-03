//! This module contains everything used to controll the lua state.
//!
//! Most of this functions are just wrappers over the offical `gmod-module-base` functions
//! and some of the documentation is ripped of too.  
//! Check here for the original stuff :  
//! - <https://github.com/Facepunch/gmod-module-base/blob/development/include/GarrysMod/Lua/LuaBase.h>
//! - <https://github.com/Facepunch/gmod-module-base/blob/development>
mod bridge;
mod error;
mod stack;
mod table;
mod user_data;

use std::{mem::MaybeUninit, str::Utf8Error};

pub use bridge::{CFunc, LuaStateRaw, MULT_RET};
pub use error::{Error, Result};
pub use stack::{FromStack, FromStackError, ToStack};
pub use table::{FromTable, TableView};
pub use user_data::{MetatableBuilder, UserData, UserType};

pub const NIL: Nil = Nil;

const LUA_GLOBALSINDEX: i32 = -10002;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LuaState(LuaStateRaw);
impl LuaState {
    /// # Safety
    /// `raw` should be a valid [LuaStateRaw] which is given to us when we are called by lua
    pub unsafe fn new(raw: LuaStateRaw) -> Self {
        Self(raw)
    }
    pub fn ptr(&self) -> LuaStateRaw {
        self.0
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Nil;
impl ToStack for Nil {
    fn push(self, state: LuaState) -> i32 {
        push_nil(state);
        1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeFunc(CFunc);
impl NativeFunc {
    pub fn new(func: CFunc) -> Self {
        Self(func)
    }
}
impl ToStack for NativeFunc {
    fn push(self, state: LuaState) -> i32 {
        push_c_function(state, self.0);
        1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaSpecial {
    /// Global table
    Glob,
    /// Environment
    Env,
    /// Registry
    Reg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaType {
    None,
    Nil,
    Bool,
    LightUserData,
    Number,
    String,
    Table,
    Function,
    UserData,
    Thread,
    Entity,
    Vector,
    Angle,
    Other(i32),
}
impl LuaType {
    fn to_number(&self) -> i32 {
        match self {
            LuaType::None => -1,
            LuaType::Nil => 0,
            LuaType::Bool => 1,
            LuaType::LightUserData => 2,
            LuaType::Number => 3,
            LuaType::String => 4,
            LuaType::Table => 5,
            LuaType::Function => 6,
            LuaType::UserData => 7,
            LuaType::Thread => 8,
            LuaType::Entity => 9,
            LuaType::Vector => 10,
            LuaType::Angle => 11,
            LuaType::Other(t) => *t,
        }
    }
    fn from_number(num: i32) -> Self {
        match num {
            -1 => LuaType::None,
            0 => LuaType::Nil,
            1 => LuaType::Bool,
            2 => LuaType::LightUserData,
            3 => LuaType::Number,
            4 => LuaType::String,
            5 => LuaType::Table,
            6 => LuaType::Function,
            7 => LuaType::UserData,
            8 => LuaType::Thread,
            9 => LuaType::Entity,
            10 => LuaType::Vector,
            11 => LuaType::Angle,
            unknown => LuaType::Other(unknown),
        }
    }
}

/// Pops `count` values from the stack.
pub fn pop(state: LuaState, count: u32) {
    unsafe { bridge::gmod_bridge_pop(state.ptr(), count as i32) }
}

/// Returns the number of elements on the stack.
pub fn top(state: LuaState) -> i32 {
    unsafe { bridge::gmod_bridge_top(state.ptr()) }
}

/// `Ok` if the value at `stack_pos` has type `ty`, `Err` otherwise.
pub fn expect_type(state: LuaState, stack_pos: i32, ty: LuaType) -> Result<(), FromStackError> {
    let actual = get_type(state, stack_pos);
    if actual != ty {
        Err(FromStackError::InvalidType {
            stack_pos,
            expected_type: ty,
            found_type: actual,
        })
    } else {
        Ok(())
    }
}

/// Converts a **relative** stack index to an **absolute** stack index.
/// If the index is absolute it does nothing.
pub fn rel_to_abs(state: LuaState, mut stack_pos: i32) -> i32 {
    if stack_pos < 0 {
        let size = top(state);
        stack_pos = size - (-stack_pos - 1);
    }
    stack_pos
}

/// Pushes `val` to the stack, returns the number of values pushed (usually just 1).
pub fn push<T: ToStack>(state: LuaState, val: T) -> i32 {
    T::push(val, state)
}

/// Pushes a copy of the value at `stack_pos` to the stack.
pub fn push_copy(state: LuaState, stack_pos: i32) {
    unsafe { bridge::gmod_bridge_push(state.ptr(), stack_pos) }
}

/// Pushes `nil` to the stack.
pub fn push_nil(state: LuaState) {
    unsafe { bridge::gmod_bridge_push_nil(state.ptr()) }
}

/// Pushes `string` to the stack.
pub fn push_string(state: LuaState, string: &str) {
    push_bytes(state, string.as_bytes())
}

/// Pushes `string` to the stack containing the given bytes
pub fn push_bytes(state: LuaState, data: &[u8]) {
    unsafe {
        bridge::gmod_bridge_push_string(state.ptr(), data.as_ptr() as *const i8, data.len() as u32)
    }
}

/// Pushes `number` to the stack.
pub fn push_number(state: LuaState, number: f64) {
    unsafe { bridge::gmod_bridge_push_number(state.ptr(), number) }
}

/// Pushes `boolean` to the stack.
pub fn push_bool(state: LuaState, boolean: bool) {
    unsafe { bridge::gmod_bridge_push_bool(state.ptr(), boolean) }
}

/// Pushes `func` to the stack.
pub fn push_c_function(state: LuaState, func: CFunc) {
    unsafe { bridge::gmod_bridge_push_c_function(state.ptr(), func) }
}

/// Pushes a special value to the stack. check [LuaSpecial].
///
/// ```
/// use gmrs::prelude::*;
/// fn set_global(state: LuaState) {
///     lua::push_special(state.ptr(), LuaSpecial::Glob);
///     lua::push(state.ptr(), "my value");
///     // the global table is now at -2.
///     lua::set_field(state.ptr(), -1, "my key");
/// }
/// ```
pub fn push_special(state: LuaState, special: LuaSpecial) {
    let special = match special {
        LuaSpecial::Glob => 0,
        LuaSpecial::Env => 1,
        LuaSpecial::Reg => 2,
    };
    unsafe { bridge::gmod_bridge_push_special(state.ptr(), special) }
}

/// Gets the value at `stack_pos`.
pub fn get<T: FromStack>(state: LuaState, stack_pos: i32) -> Result<T> {
    <T as FromStack>::from_stack(state, stack_pos).map(|(v, _)| v)
}

/// Gets the string at `stack_pos`.
pub fn get_string(state: LuaState, stack_pos: i32) -> Result<String, Utf8Error> {
    let str_bytes = unsafe {
        let mut outlen = 0;
        let ptr = bridge::gmod_bridge_get_string(state.ptr(), stack_pos, &mut outlen) as *const u8;
        std::slice::from_raw_parts(ptr, outlen as usize)
    };
    Ok(std::str::from_utf8(str_bytes)?.to_string())
}

/// Gets the string at `stack_pos` without trying to convert to valid utf-8.
pub fn get_string_bytes(state: LuaState, stack_pos: i32) -> Vec<u8> {
    unsafe {
        let mut outlen = 0;
        let ptr = bridge::gmod_bridge_get_string(state.ptr(), stack_pos, &mut outlen) as *const u8;
        std::slice::from_raw_parts(ptr, outlen as usize).to_owned()
    }
}

/// Gets the number at `stack_pos`.
pub fn get_number(state: LuaState, stack_pos: i32) -> f64 {
    unsafe { bridge::gmod_bridge_get_number(state.ptr(), stack_pos) }
}

/// Gets the bool at `stack_pos`.
pub fn get_bool(state: LuaState, stack_pos: i32) -> bool {
    unsafe { bridge::gmod_bridge_get_bool(state.ptr(), stack_pos) }
}

/// Checks if the value at `stack_pos` has type `ty`.
pub fn is_type(state: LuaState, stack_pos: i32, ty: LuaType) -> bool {
    unsafe { bridge::gmod_bridge_is_type(state.ptr(), stack_pos, ty.to_number()) }
}

/// Returns the type of the value at `stack_pos`.
pub fn get_type(state: LuaState, stack_pos: i32) -> LuaType {
    unsafe { LuaType::from_number(bridge::gmod_bridge_get_type(state.ptr(), stack_pos)) }
}

/// Creates a reference of the value at the top of the stack and pops the value.
/// Returns the reference. You have to remember to call `reference_free(reference)`.
pub fn reference_create(state: LuaState) -> i32 {
    unsafe { bridge::gmod_bridge_reference_create(state.ptr()) }
}

/// Frees the `reference`.
pub fn reference_free(state: LuaState, reference: i32) {
    unsafe { bridge::gmod_bridge_reference_free(state.ptr(), reference) }
}

/// Pushes the value pointed by `reference` to the stack.
pub fn reference_push(state: LuaState, reference: i32) {
    unsafe { bridge::gmod_bridge_reference_push(state.ptr(), reference) }
}

/// Pushes a new empty table to the stack.
/// Returns a [TableView] to more easily manipulate the table.
pub fn create_table(state: LuaState) -> TableView {
    unsafe { bridge::gmod_bridge_create_table(state.ptr()) };
    TableView::new(state, top(state))
}

/// Pushes table\[key\] on to the stack.  
/// table = value at iStackPos.  
/// key   = value at top of the stack.  
/// Pops the key from the stack.  
pub fn get_table(state: LuaState, stack_pos: i32) {
    unsafe { bridge::gmod_bridge_get_table(state.ptr(), stack_pos) }
}

/// Sets table\[key\] to the value at the top of the stack.  
/// table = value at iStackPos.  
/// key   = value 2nd to the top of the stack.  
/// Pops the key and the value from the stack.  
pub fn set_table(state: LuaState, stack_pos: i32) {
    unsafe { bridge::gmod_bridge_set_table(state.ptr(), stack_pos) }
}

/// Pushes table\[key\] on to the stack  
/// table = value at iStackPos  
/// key   = strName  
pub fn get_field(state: LuaState, stack_pos: i32, name: &str) {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { bridge::gmod_bridge_get_field(state.ptr(), stack_pos, name.as_ptr()) }
}

/// Sets table\[key\] to the value at the top of the stack.  
/// table = value at iStackPos  
/// key   = strName  
/// Pops the value from the stack  
pub fn set_field(state: LuaState, stack_pos: i32, name: &str) {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { bridge::gmod_bridge_set_field(state.ptr(), stack_pos, name.as_ptr()) }
}

/// To call a function first push the push function to stack then push
/// each argument from first to last.
/// `call` will pop the function and the arguments and will pop the function and the arguments.
/// `args` is the number of arguments you pushed.
/// `results` is the number of results that will be pushed to the stack after the function is called.
///
/// # Safety
/// This function is unsafe beacause if the function we are calling throws an error then
/// this functions never returns and we risk leaking resources. check [pcall].
pub unsafe fn call(state: LuaState, args: i32, results: i32) {
    bridge::gmod_bridge_call(state.ptr(), args, results)
}

#[must_use]
/// Similar to [call] but `true` if the function succeded and `false` if it failed.
/// If the function failed the error object will be at the top of the stack.
pub fn pcall(state: LuaState, args: i32, results: i32) -> bool {
    unsafe { bridge::gmod_bridge_pcall(state.ptr(), args, results, 0) == 0 }
}

#[must_use]
/// Similar to [pcall] but instead of pushing the arguments should be all pushed in the `args` function parameter.
/// The funcion to be called should be on the top of the stack prior to calling this function.
/// This function wraps [pcall] and counts the number of arguments pushed.
pub fn pcall_with<F>(state: LuaState, results: i32, args: F) -> bool
where
    F: FnOnce(LuaState),
{
    let size_before_args = top(state);
    args(state);
    let size_after_args = top(state);
    let arg_count = size_after_args - size_before_args;
    pcall(state, arg_count, results)
}

/// Similar to [pcall] but if the function fails nothing is pushed to the stack and instead an error
/// is returned. If the function succeds then Ok is returned and the results are pushed to the stack.
pub fn pcall_result(state: LuaState, args: i32, results: i32) -> Result<()> {
    match pcall(state, args, results) {
        true => Ok(()),
        false => {
            let err_msg = get_string_bytes(state, -1);
            pop(state, 1);
            Err(Error::CustomBytes(err_msg))
        }
    }
}

/// [pcall_with] + [pcall_result] => pcall_result_with
pub fn pcall_result_with<F>(state: LuaState, results: i32, args: F) -> Result<()>
where
    F: FnOnce(LuaState),
{
    let size_before_args = top(state);
    args(state);
    let size_after_args = top(state);
    let arg_count = size_after_args - size_before_args;
    pcall_result(state, arg_count, results)
}

/// Creates a new user data with size `size` and returns the pointer to the allocated memory.
pub fn new_user_data(state: LuaState, size: u32) -> *mut std::ffi::c_void {
    unsafe { bridge::gmod_bridge_new_user_data(state.ptr(), size) }
}

/// Returns the user data at `stack_pos`. If the value at `stack_pos` is not user data then it
/// returns a null pointer.
pub fn get_user_data(state: LuaState, stack_pos: i32) -> *mut std::ffi::c_void {
    unsafe { bridge::gmod_bridge_get_user_data(state.ptr(), stack_pos) }
}

/// Sets the metatable for the value at `stack_pos`.
/// The table itself should be at the top of the stack.
/// This function will pop the metatable.
pub fn set_metatable(state: LuaState, stack_pos: i32) {
    unsafe { bridge::gmod_bridge_set_meta_table(state.ptr(), stack_pos) }
}

#[must_use]
/// Returns `true` if the value at `stack_pos` has a metatable and pushes it to the stack.
/// Returns `false` if the value doesnt have a metatable and doesnt push anything to the stack.
pub fn get_metatable(state: LuaState, stack_pos: i32) -> bool {
    unsafe { bridge::gmod_bridge_get_meta_table(state.ptr(), stack_pos) }
}

/// # Safety
/// Throws an error message. This function will never return.
/// If you call this no destructors will be called and you risk leaking resources.
pub unsafe fn throw_error(state: LuaState, error: String) -> ! {
    let mut buffer = [0u8; 4096];
    let copy_len = (buffer.len() - 1).min(error.as_bytes().len());
    (&mut buffer[..copy_len]).copy_from_slice(&error.as_bytes()[..copy_len]);
    buffer[copy_len] = 0;
    drop(error);
    bridge::gmod_bridge_throw_error(state.ptr(), buffer.as_ptr() as *const i8);
    unreachable!()
}

/// Gets the upvalue `index`.
pub fn upvalue_index(index: i32) -> i32 {
    LUA_GLOBALSINDEX - index
}

/// Pushes a c closure.
/// First push the arguments you want to have as upvalues.
/// `nargs` is the number of arguments you pushed.
/// The `push_c_closure` will pop the values a push a closure with those upvalues.
///
///```
/// # use gmrs::prelude::*;
/// # fn example_main(state: LuaState) {
/// # let some_tables_index = 0;
/// lua::push(state.ptr(), "myvalue1");
/// lua::push_copy(state.ptr(), some_tables_index);
/// lua::push_c_closure(state.ptr(), my_closure, 2);
/// # }
///
/// #[gmrs::function]
/// fn my_closure(state: LuaState) -> lua::Result<()> {
/// let myvalue_index = lua::upvalue_index(1);
/// let some_table_index = lua::upvalue_index(2);
/// // lua::push_copy, lua::get, ...
/// Ok(())
///}
///```
pub fn push_c_closure(state: LuaState, func: CFunc, nargs: i32) {
    unsafe { bridge::gmod_bridge_push_c_closure(state.ptr(), func, nargs) }
}

pub struct Closure<R: ToStack + Send>(Box<dyn FnMut(LuaState) -> Result<R> + Send>);
impl<R: ToStack + Send> ToStack for Closure<R> {
    fn push(self, state: LuaState) -> i32 {
        unsafe {
            let ud = new_user_data(state, std::mem::size_of::<Closure<R>>() as u32)
                as *mut MaybeUninit<Closure<R>>;
            (*ud) = MaybeUninit::new(self);
        }
        create_table(state);
        push(state, NativeFunc::new(closure_gc::<R>));
        set_field(state, -2, "__gc");
        set_metatable(state, -2);
        push_c_closure(state, closure_call::<R>, 1);
        1
    }
}

/// Creates a rust closure that can be pushed to the stack.
pub fn closure<F, R>(func: F) -> Closure<R>
where
    F: FnMut(LuaState) -> Result<R> + Send + 'static,
    R: ToStack + Send,
{
    Closure(Box::new(func))
}

/// Pushes a rust closure to the stack
pub fn push_closure<R, F>(state: LuaState, func: F)
where
    F: FnMut(LuaState) -> Result<R> + Send + 'static,
    R: ToStack + Send,
{
    let closure = closure(func);
    push(state, closure);
}

unsafe extern "C" fn closure_call<R: ToStack + Send>(state: LuaStateRaw) -> i32 {
    let state = LuaState::new(state);
    let ud = get_user_data(state, upvalue_index(1)) as *mut Closure<R>;
    let result = (*ud).0(state);
    match result {
        Ok(value) => push(state, value),
        Err(e) => {
            let msg = format!("{}", e);
            drop(e);
            throw_error(state, msg);
        }
    }
}

unsafe extern "C" fn closure_gc<R: ToStack + Send>(state: LuaStateRaw) -> i32 {
    let state = LuaState::new(state);
    let ud = get_user_data(state, 1) as *mut Closure<R>;
    if !ud.is_null() {
        // why would it be null ? probably wont be
        std::ptr::drop_in_place(ud);
    }
    0
}

/// Returns an error with custom message.
///
///```
///# use gmrs::prelude::*;
/// #[gmrs::function]
/// fn my_function(state: LuaState) -> lua::Result<()> {
///     # let some_condition = false;
///     if !some_condition {
///         // This error will be propagated up and eventually be thrown.
///         return lua::error_message("Condition failed");
///     }
///     Ok(())  
/// }
///```
pub fn error_message<S: Into<String>>(message: S) -> Result<()> {
    Err(error::Error::CustomMessage(message.into()))
}
