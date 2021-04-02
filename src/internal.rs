use crossbeam::channel::{self, Receiver, Sender};
use std::{cell::Cell, time::Instant};

use crate::lua::{self, LuaState, LuaStateRaw};

std::thread_local!(static CURRENT_LUA_STATE_RAW: Cell<LuaStateRaw> = Cell::new(std::ptr::null_mut()));
lazy_static! {
    static ref CREATION_TIME: Instant = Instant::now();
    static ref INTERNAL_CHANNELS: (Sender<InternalMessage>, Receiver<InternalMessage>) =
        channel::unbounded();
}

enum InternalMessage {
    ReferenceFree(i32),
    RemoteExecute(Box<dyn FnOnce(LuaState) + Send>),
}

pub fn get_lua_state() -> Option<LuaState> {
    let raw_state = CURRENT_LUA_STATE_RAW.with(|c| c.get());
    if raw_state.is_null() {
        None
    } else {
        Some(unsafe { LuaState::new(raw_state) })
    }
}

pub unsafe fn set_lua_state_raw(raw: LuaStateRaw) {
    CURRENT_LUA_STATE_RAW.with(|c| c.set(raw));
}

/// Sets the current thread's lua state to null, probably dont need call this
pub fn unset_lua_state_raw() {
    unsafe { set_lua_state_raw(std::ptr::null_mut()) }
}

/// Should only be called once from the module open function
pub unsafe fn install_hook(state: LuaState) {
    let hook_name = internal_hook_name_from_time(CREATION_TIME.clone());
    crate::hook_add(
        state,
        "Think",
        &hook_name,
        crate::lua::closure(|state| {
            let receiver = INTERNAL_CHANNELS.1.clone();
            internal_think_loop(state, receiver);
            Ok(())
        }),
    );
}

/// Should only be called once from the module close function
pub unsafe fn uninstall_hook(state: LuaState) {
    let hook_name = internal_hook_name_from_time(CREATION_TIME.clone());
    crate::hook_remove(state, "Think", &hook_name);
}

fn send_internal_message(msg: InternalMessage) {
    let _ = INTERNAL_CHANNELS.0.send(msg);
}

pub fn remote_reference_free(reference: i32) {
    send_internal_message(InternalMessage::ReferenceFree(reference));
}

pub fn remote_execute<F, R>(func: F) -> R
where
    R: Send + 'static,
    F: FnOnce(LuaState) -> R + Send + 'static,
{
    let (tx, rx) = channel::bounded(0);
    let msg = InternalMessage::RemoteExecute(Box::new(move |state| {
        let result = func(state);
        // this should never fail
        tx.send(result).unwrap();
    }));
    send_internal_message(msg);
    // this should never fail either
    rx.recv().unwrap()
}

fn internal_hook_name_from_time(time: Instant) -> String {
    format!("gmrs_internal_hook_{:?}", time)
}

fn internal_think_loop(state: LuaState, receiver: Receiver<InternalMessage>) {
    while let Ok(msg) = receiver.try_recv() {
        match msg {
            InternalMessage::ReferenceFree(reference) => lua::reference_free(state, reference),
            InternalMessage::RemoteExecute(func) => func(state),
        }
    }
}
