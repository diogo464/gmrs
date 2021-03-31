use super::{FromStack, LuaStateRaw, LuaType, Result, ToStack};

pub enum TableKey<'a> {
    String(&'a str),
    Integer(u32),
}
impl<'a> From<&'a str> for TableKey<'a> {
    fn from(s: &'a str) -> Self {
        Self::String(s)
    }
}
impl<'a> From<&'a String> for TableKey<'a> {
    fn from(s: &'a String) -> Self {
        Self::String(s.as_str())
    }
}
impl<'a> From<u32> for TableKey<'a> {
    fn from(i: u32) -> Self {
        Self::Integer(i)
    }
}
impl<'a> ToStack for TableKey<'a> {
    fn push(self, state: LuaStateRaw) -> i32 {
        match self {
            Self::String(str) => super::push(state, str),
            Self::Integer(int) => super::push(state, int),
        }
    }
}

pub trait FromTable: Sized {
    fn from_table(state: LuaStateRaw, tbl: TableView) -> super::Result<Self>;
}
impl<T: FromTable> FromStack for T {
    fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)> {
        super::expect_type(state, stack_pos, LuaType::Table)?;
        let table_ref = TableView::new(state, stack_pos);
        Ok((T::from_table(state, table_ref)?, 1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableView(i32);
impl TableView {
    pub fn new(state: LuaStateRaw, stack_pos: i32) -> Self {
        Self(super::rel_to_abs(state, stack_pos))
    }

    pub fn stack_pos(&self) -> i32 {
        self.0
    }

    /// Tries to get the value with `key`, fails if the conversion to `T` fails.
    pub fn get<'a, T>(&self, state: LuaStateRaw, key: impl Into<TableKey<'a>>) -> Result<T>
    where
        T: FromStack,
    {
        super::push(state, key.into());
        super::get_table(state, self.stack_pos());
        super::get(state, -1)
    }

    pub fn set<'a, T>(&self, state: LuaStateRaw, key: impl Into<TableKey<'a>>, value: T)
    where
        T: ToStack,
    {
        super::push(state, key.into());
        super::push(state, value);
        super::set_table(state, self.stack_pos());
    }

    pub fn unset<'a>(&self, state: LuaStateRaw, key: impl Into<TableKey<'a>>) {
        self.set(state, key, super::NIL)
    }

    /// Pushes the value with `key` on the stack.
    pub fn push_value<'a>(&self, state: LuaStateRaw, key: impl Into<TableKey<'a>>) {
        super::push(state, key.into());
        super::get_table(state, self.stack_pos());
    }
}
impl ToStack for TableView {
    fn push(self, state: LuaStateRaw) -> i32 {
        super::push_copy(state, self.stack_pos());
        1
    }
}
impl FromStack for TableView {
    fn from_stack(state: LuaStateRaw, stack_pos: i32) -> Result<(Self, i32)> {
        super::expect_type(state, stack_pos, LuaType::Table)?;
        Ok((Self::new(state, stack_pos), 1))
    }
}
