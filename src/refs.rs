use crate::{
    internal,
    lua::{self, FromStack, LuaState, ToStack},
};
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

// https://www.lua.org/source/5.1/lauxlib.h.html#LUA_NOREF
const LUA_NOREF: i32 = -2;

#[derive(Debug)]
pub struct OwnedRef(i32);
impl OwnedRef {
    /// Creates a new [OwnedRef] of the element at `stack_pos`
    pub fn new(state: LuaState, stack_pos: i32) -> Self {
        lua::push_copy(state, stack_pos);
        Self::from_top_of_stack(state)
    }

    /// Creates a new [OwnedRef] of the element at the top of the stack and pops it off
    pub fn from_top_of_stack(state: LuaState) -> Self {
        let reference = lua::reference_create(state);
        Self(reference)
    }
}

impl Drop for OwnedRef {
    fn drop(&mut self) {
        drop_reference(self.0)
    }
}

impl FromStack for OwnedRef {
    fn from_stack(state: LuaState, stack_pos: i32) -> lua::Result<(Self, i32)> {
        Ok((OwnedRef::new(state, stack_pos), 1))
    }
}

impl ToStack for OwnedRef {
    fn push(self, state: LuaState) -> i32 {
        (&self).push(state)
    }
}

impl ToStack for &OwnedRef {
    fn push(self, state: LuaState) -> i32 {
        lua::reference_push(state, self.0);
        1
    }
}

#[derive(Debug, Clone)]
pub struct ArcRef(Arc<OwnedRef>);
impl ArcRef {
    pub fn new(state: LuaState, stack_pos: i32) -> Self {
        Self(Arc::new(OwnedRef::new(state, stack_pos)))
    }

    pub fn from_top_of_stack(state: LuaState) -> Self {
        Self(Arc::new(OwnedRef::from_top_of_stack(state)))
    }
}

impl From<OwnedRef> for ArcRef {
    fn from(owned_ref: OwnedRef) -> Self {
        Self(Arc::new(owned_ref))
    }
}

impl ToStack for ArcRef {
    fn push(self, state: LuaState) -> i32 {
        (&self).push(state)
    }
}

impl ToStack for &ArcRef {
    fn push(self, state: LuaState) -> i32 {
        self.0.as_ref().push(state)
    }
}

#[derive(Debug, Clone)]
pub struct AtomicRef(Arc<AtomicInternal>);
impl AtomicRef {
    /// Creates a new [AtomicRef] of the element at `stack_pos`
    pub fn new(state: LuaState, stack_pos: i32) -> Self {
        lua::push_copy(state, stack_pos);
        Self::from_top_of_stack(state)
    }

    /// Creates a new [AtomicRef] of the element at the top of the stack and pops it off
    pub fn from_top_of_stack(state: LuaState) -> Self {
        let reference = lua::reference_create(state);
        Self(Arc::new(AtomicInternal(AtomicI32::new(reference))))
    }

    pub fn nill() -> Self {
        Self(Arc::new(AtomicInternal(AtomicI32::new(LUA_NOREF))))
    }

    pub fn replace(&self, mut reference: OwnedRef) {
        let new_ref = reference.0;
        reference.0 = self.0.load();
        self.0.store(new_ref);
    }
}

impl Default for AtomicRef {
    fn default() -> Self {
        Self::nill()
    }
}

impl From<OwnedRef> for AtomicRef {
    fn from(owned_ref: OwnedRef) -> Self {
        let reference = owned_ref.0;
        std::mem::forget(owned_ref);
        Self(Arc::new(AtomicInternal(AtomicI32::new(reference))))
    }
}

impl ToStack for AtomicRef {
    fn push(self, state: LuaState) -> i32 {
        (&self).push(state)
    }
}

impl ToStack for &AtomicRef {
    fn push(self, state: LuaState) -> i32 {
        lua::reference_push(state, self.0.load());
        1
    }
}

#[derive(Debug)]
struct AtomicInternal(AtomicI32);
impl AtomicInternal {
    fn store(&self, reference: i32) {
        self.0.store(reference, Ordering::Relaxed)
    }

    fn load(&self) -> i32 {
        self.0.load(Ordering::Relaxed)
    }
}

impl Drop for AtomicInternal {
    fn drop(&mut self) {
        let reference = self.0.load(Ordering::Relaxed);
        drop_reference(reference)
    }
}

fn drop_reference(reference: i32) {
    if let Some(state) = crate::internal::get_lua_state() {
        lua::reference_free(state, reference);
    } else {
        internal::remote_reference_free(reference);
    }
}
