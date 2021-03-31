use gmrs::prelude::*;

#[gmrs::function]
fn hello_world(state: LuaStateRaw) -> lua::Result<()> {
    gmrs::print!(state, "Hello from rust");
    Ok(())
}

#[gmrs::function]
fn add(
    a: f64,
    _state: LuaStateRaw, /* The state can be anywhere in the arguments or not be here at all */
    b: f64,
) -> lua::Result<f64> {
    Ok(a + b)
}

#[gmrs::function]
fn sub(state: LuaStateRaw) -> lua::Result<f64> {
    let a: f64 = lua::get(state, 1)?;
    let b: f64 = lua::get(state, 2)?;
    Ok(a - b)
}

#[gmrs::entry]
fn main(state: LuaStateRaw) {
    gmrs::set_global(state, "rust_hello_world", NativeFunc::new(hello_world));
    let tbl = lua::create_table(state);
    tbl.set(state, "add", NativeFunc::new(add));
    tbl.set(state, "sub", NativeFunc::new(sub));
    gmrs::set_global(state, "rust_arithmetic", tbl);
}

#[gmrs::exit]
fn exit(_state: LuaStateRaw) {}
