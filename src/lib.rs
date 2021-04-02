#[macro_use]
extern crate lazy_static;

pub mod internal;
pub mod lua;
pub mod owned_ref;

pub use gmrs_impl::{entry, exit, function, raw_function};
pub use internal::{get_lua_state, remote_execute};
pub use owned_ref::OwnedRef;

use lua::{LuaSpecial, LuaState, ToStack};

pub mod prelude {
    pub use super::lua::{
        self, FromStack, FromTable, LuaSpecial, LuaState, LuaStateRaw, MetatableBuilder,
        NativeFunc, TableView, ToStack, UserData, UserType,
    };
    pub use super::OwnedRef;
}

/// Prints the message using gmod's `print` function, the message should show up on the console.
pub fn print(state: LuaState, message: &str) {
    lua::push_special(state, LuaSpecial::Glob);
    lua::get_field(state, -1, "print");
    lua::push_string(state, message);
    unsafe { lua::call(state, 1, 0) };
    lua::pop(state, 1);
}

#[macro_export]
/// Same as `print` but allows for formatting.
macro_rules! print {
    ($state:expr, $($arg:tt)*) => {
        $crate::print($state, &format!($($arg)*));
    };
}

/// Equivalent to `hook.Add(event, id, callback)`.
pub fn hook_add<T: ToStack>(state: LuaState, event: &str, id: &str, val: T) {
    lua::push_special(state, LuaSpecial::Glob);
    lua::get_field(state, -1, "hook");
    lua::get_field(state, -1, "Add");
    lua::push(state, event);
    lua::push(state, id);
    lua::push(state, val);
    let status = lua::pcall(state, 3, 0);
    assert!(status, "Calling hook.Add cant fail");
}

/// Equivalent to `hook.Remove(event, id)`.
pub fn hook_remove(state: LuaState, event: &str, id: &str) {
    lua::push_special(state, LuaSpecial::Glob);
    lua::get_field(state, -1, "hook");
    lua::get_field(state, -1, "Remove");
    lua::push(state, event);
    lua::push(state, id);
    let status = lua::pcall(state, 2, 0);
    assert!(status, "Calling hook.Remove cant fail");
}

/// Runs hook with name `name` by calling `hook.Run`.  
/// `func` should push the parameters to the stack.
pub fn hook_run<F>(state: LuaState, name: &str, func: F)
where
    F: FnOnce(LuaState),
{
    lua::push_special(state, LuaSpecial::Glob);
    lua::get_field(state, -1, "hook");
    lua::get_field(state, -1, "Run");
    lua::push(state, name);
    let size_before_args = lua::top(state);
    func(state);
    let size_after_args = lua::top(state);
    let arg_count = size_after_args - size_before_args;
    assert!(
        arg_count >= 0,
        "Dont pop values in the function that is supposed to push values"
    );
    let status = lua::pcall(state, arg_count, 0);
    assert!(status, "Calling hook.Run cant fail");
}

/// Sets a value on the global table.
pub fn set_global<T: ToStack>(state: LuaState, key: &str, value: T) {
    lua::push_special(state, LuaSpecial::Glob);
    lua::push(state, value);
    lua::set_field(state, -2, key);
    lua::pop(state, 1);
}
