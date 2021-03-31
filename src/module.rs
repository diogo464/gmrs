use crate::lua::{self, CFunc, LuaSpecial, LuaStateRaw, Table, ToStack};

pub struct Module {
    state: LuaStateRaw,
    functions: Vec<String>,
}
impl Module {
    pub unsafe fn from_raw(state: LuaStateRaw) -> Self {
        Self {
            state,
            functions: Default::default(),
        }
    }

    pub fn register_function(&mut self, name: &str, func: CFunc) {
        unsafe {
            lua::push_special(self.state, LuaSpecial::Glob);
            lua::push_c_function(self.state, std::mem::transmute(func));
            lua::set_field(self.state, -2, name);
            lua::pop(self.state, 1);
        }
    }

    pub fn create_table(&mut self) -> Table {
        Table::new(self.state)
    }

    pub fn set_global<T: ToStack>(&mut self, key: &str, value: T) {
        lua::push_special(self.state, LuaSpecial::Glob);
        lua::push(self.state, value);
        lua::set_field(self.state, -2, key);
    }
}
impl Drop for Module {
    fn drop(&mut self) {
        for func in self.functions.iter() {
            lua::push_special(self.state, LuaSpecial::Glob);
            lua::push_nil(self.state);
            lua::set_field(self.state, -2, func);
            lua::pop(self.state, 1);
        }
    }
}
