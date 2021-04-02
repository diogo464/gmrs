use std::time::Duration;

use gmrs::prelude::*;

#[gmrs::function]
fn some_long_computation(state: LuaState) -> lua::Result<()> {
    // Create a reference to the first parameter, a success callback
    let success_callback = OwnedRef::new(state, 1);
    // Create a reference to the second parameter, a failure callback
    let failure_callback = OwnedRef::new(state, 2);

    std::thread::spawn(move || {
        let success_callback = success_callback;
        let _failure_callback = failure_callback;

        std::thread::sleep(Duration::from_secs(5));
        let long_task_result = 50;
        gmrs::remote_execute(move |state| {
            lua::push(state, success_callback);
            lua::push(state, long_task_result);
            // Call the callback with the result and ignore if it fails
            let _ = lua::pcall(state, 1, 0);
        });
    });
    Ok(())
}

#[gmrs::entry]
fn main(state: LuaState) {
    gmrs::set_global(
        state,
        "some_long_computation",
        NativeFunc::new(some_long_computation),
    );
}

#[gmrs::exit]
fn exit(_state: LuaState) {}
