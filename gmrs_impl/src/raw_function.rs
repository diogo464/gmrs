use syn::ItemFn;

pub fn parse(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);

    let vis = &item.vis;
    let name = &item.sig.ident;

    (quote::quote! {
        #vis extern "C" fn #name(raw : gmrs::lua::LuaStateRaw) -> i32 {
            unsafe { gmrs::internal::set_lua_state_raw(raw) };
            let state = unsafe { gmrs::lua::LuaState::new(raw) };
            #item
            let result = #name(state);
            gmrs::internal::unset_lua_state_raw();
            match result {
                Ok(count) => count,
                Err(e) => {
                    let msg = format!("{}", e);
                    drop(e);
                    unsafe { gmrs::lua::throw_error(state, msg) };
                }
            }
        }
    })
    .into()
}
