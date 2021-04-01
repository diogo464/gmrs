use super::{CFunc, FromStack, FromStackError, LuaState, LuaStateRaw, NativeFunc, Result, ToStack};
use std::{
    any::TypeId,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug)]
pub struct UserData<T: UserType> {
    ty: TypeId,
    value: Arc<Mutex<T>>,
}
impl<T: UserType> UserData<T> {
    pub fn new(value: T) -> Self {
        Self {
            ty: TypeId::of::<T>(),
            value: Arc::new(Mutex::new(value)),
        }
    }

    /// Locks the user data and lets you operate on it.
    /// `DONT` call lua functions from inside `with`, if lua
    /// calls us again with this userdata and we try to lock it
    /// we will enter a deadlock.
    pub fn with<F, R>(&self, func: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut this = self.lock();
        func(&mut this)
    }

    /// Carefull locking the same userdata twice
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.value.lock().unwrap()
    }

    /// Checks if 2 `UserData` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}
impl<T: UserType> Clone for UserData<T> {
    fn clone(&self) -> Self {
        Self {
            ty: self.ty,
            value: self.value.clone(),
        }
    }
}

pub trait UserType: Sized + 'static {
    fn build_metatable(builder: &mut MetatableBuilder<Self>);
    /// Called when the user data is garbage collected, the `__gc` method.
    fn lua_drop(&mut self, #[allow(unused)] state: LuaState) {}
}

#[derive(Debug)]
pub struct MetatableBuilder<T: UserType> {
    _phantom: PhantomData<T>,
    methods: Vec<(String, CFunc)>,
}
impl<T: UserType> MetatableBuilder<T> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
            methods: Default::default(),
        }
    }

    pub fn method(&mut self, name: &str, func: CFunc) {
        self.methods.push((name.to_string(), func))
    }
}

impl<T: UserType> ToStack for UserData<T> {
    fn push(self, state: LuaState) -> i32 {
        // It should be safe since we cant share LuaState across threads and its only ever
        // called from the same thread
        static mut METATABLE_REFERENCE: Option<i32> = None;

        let mt_ref = unsafe {
            if let Some(r) = METATABLE_REFERENCE {
                r
            } else {
                let table = super::create_table(state);
                let mut builder = MetatableBuilder::new();
                <T as UserType>::build_metatable(&mut builder);
                for (name, method) in builder.methods {
                    table.set(state, &name, NativeFunc::new(method));
                }
                table.set(state, "__gc", NativeFunc::new(user_data_gc::<T>));
                table.set(state, "__index", table);
                let r = super::reference_create(state);
                METATABLE_REFERENCE = Some(r);
                r
            }
        };

        unsafe {
            let ud = super::new_user_data(state, std::mem::size_of::<Self>() as u32)
                as *mut MaybeUninit<Self>;
            (*ud) = MaybeUninit::new(UserData {
                ty: self.ty,
                value: self.value,
            });
        }
        super::reference_push(state, mt_ref);
        super::set_metatable(state, -2);
        1
    }
}

impl<T: UserType> FromStack for UserData<T> {
    fn from_stack(state: LuaState, stack_pos: i32) -> Result<(Self, i32)> {
        let ud = unsafe {
            let ud = super::get_user_data(state, stack_pos) as *mut Self;
            if ud.is_null() || (*ud).ty != TypeId::of::<T>() {
                Err(FromStackError::InvalidUserdataType)?;
            }
            Clone::clone(&*ud)
        };
        Ok((ud, 1))
    }
}

unsafe extern "C" fn user_data_gc<T: UserType>(state: LuaStateRaw) -> i32 {
    let state = LuaState::new(state);
    let ud = super::get_user_data(state, 1) as *mut UserData<T>;
    crate::print(state, "Calling user data gc");
    if !ud.is_null() {
        let mut this = (*ud).lock();
        T::lua_drop(&mut this, state);
        drop(this);
        std::ptr::drop_in_place(ud);
    }
    0
}
