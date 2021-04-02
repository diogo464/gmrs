use crate::{
    internal,
    lua::{self, LuaState, ToStack},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OwnedRef(Arc<Internal>);
impl OwnedRef {
    /// Creates a new [OwnedRef] of the element at `stack_pos`
    pub fn new(state: LuaState, stack_pos: i32) -> Self {
        lua::push_copy(state, stack_pos);
        Self::from_top_of_stack(state)
    }

    /// Creates a new [OwnedRef] of the element at the top of the stack and pops it off
    pub fn from_top_of_stack(state: LuaState) -> Self {
        let reference = lua::reference_create(state);
        Self(Arc::new(Internal(reference)))
    }
}
impl ToStack for OwnedRef {
    fn push(self, state: LuaState) -> i32 {
        lua::reference_push(state, self.0 .0);
        1
    }
}

#[derive(Debug, Clone)]
struct Internal(i32);
impl Drop for Internal {
    fn drop(&mut self) {
        if let Some(state) = crate::internal::get_lua_state() {
            lua::reference_free(state, self.0);
        } else {
            internal::remote_reference_free(self.0);
        }
    }
}
