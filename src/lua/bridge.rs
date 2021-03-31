/// Functions that are exported by bridge.cpp

pub type LuaStateRaw = *mut std::ffi::c_void;
pub type CFunc = unsafe extern "C" fn(LuaStateRaw) -> i32;
pub const MULT_RET: i32 = -1;

#[allow(unused)]
extern "C" {
    pub fn gmod_bridge_top(state: LuaStateRaw) -> i32;
    pub fn gmod_bridge_push(state: LuaStateRaw, stack_pos: i32);
    pub fn gmod_bridge_pop(state: LuaStateRaw, amount: i32);
    pub fn gmod_bridge_get_table(state: LuaStateRaw, stack_pos: i32);
    pub fn gmod_bridge_get_field(
        state: LuaStateRaw,
        stack_pos: i32,
        name: *const std::os::raw::c_char,
    );
    pub fn gmod_bridge_set_field(
        state: LuaStateRaw,
        stack_pos: i32,
        name: *const std::os::raw::c_char,
    );
    pub fn gmod_bridge_create_table(state: LuaStateRaw);
    pub fn gmod_bridge_set_table(state: LuaStateRaw, i: i32);
    pub fn gmod_bridge_set_meta_table(state: LuaStateRaw, i: i32);
    pub fn gmod_bridge_get_meta_table(state: LuaStateRaw, i: i32) -> bool;
    pub fn gmod_bridge_call(state: LuaStateRaw, args: i32, results: i32);
    pub fn gmod_bridge_pcall(state: LuaStateRaw, args: i32, results: i32, error_func: i32) -> i32;
    pub fn gmod_bridge_equal(state: LuaStateRaw, a: i32, b: i32) -> i32;
    pub fn gmod_bridge_raw_equal(state: LuaStateRaw, a: i32, b: i32) -> i32;
    pub fn gmod_bridge_insert(state: LuaStateRaw, stack_pos: i32);
    pub fn gmod_bridge_remove(state: LuaStateRaw, stack_pos: i32);
    pub fn gmod_bridge_next(state: LuaStateRaw, stack_pos: i32) -> i32;
    pub fn gmod_bridge_throw_error(state: LuaStateRaw, error: *const std::os::raw::c_char);
    pub fn gmod_bridge_check_type(state: LuaStateRaw, stack_pos: i32, ty: i32);
    //pub fn gmod_bridge_arg_error(state: LuaStateRaw, arg_num: i32, msg: *const std::os::raw::c_char);
    pub fn gmod_bridge_raw_get(state: LuaStateRaw, stack_pos: i32);
    pub fn gmod_bridge_raw_set(state: LuaStateRaw, stack_pos: i32);

    pub fn gmod_bridge_get_string(
        state: LuaStateRaw,
        stack_pos: i32,
        outlen: *mut u32,
    ) -> *const std::os::raw::c_char;
    pub fn gmod_bridge_get_number(state: LuaStateRaw, stack_pos: i32) -> f64;
    pub fn gmod_bridge_get_bool(state: LuaStateRaw, stack_pos: i32) -> bool;
    pub fn gmod_bridge_get_c_function(state: LuaStateRaw, stack_pos: i32) -> CFunc;
    pub fn gmod_bridge_push_nil(state: LuaStateRaw);
    pub fn gmod_bridge_push_string(state: LuaStateRaw, val: *const std::os::raw::c_char, len: u32);
    pub fn gmod_bridge_push_number(state: LuaStateRaw, val: f64);
    pub fn gmod_bridge_push_bool(state: LuaStateRaw, val: bool);
    pub fn gmod_bridge_push_c_function(state: LuaStateRaw, val: CFunc);
    pub fn gmod_bridge_push_c_closure(state: LuaStateRaw, val: CFunc, vars: i32);

    pub fn gmod_bridge_reference_create(state: LuaStateRaw) -> i32;
    pub fn gmod_bridge_reference_free(state: LuaStateRaw, reference: i32);
    pub fn gmod_bridge_reference_push(state: LuaStateRaw, reference: i32);

    pub fn gmod_bridge_push_special(state: LuaStateRaw, special: i32);
    pub fn gmod_bridge_is_type(state: LuaStateRaw, stack_pos: i32, ty: i32) -> bool;
    pub fn gmod_bridge_get_type(state: LuaStateRaw, stack_pos: i32) -> i32;

    pub fn gmod_bridge_new_user_data(state: LuaStateRaw, size: u32) -> *mut std::ffi::c_void;
    pub fn gmod_bridge_get_user_data(state: LuaStateRaw, stack_pos: i32) -> *mut std::ffi::c_void;
}
