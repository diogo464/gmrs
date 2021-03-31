use syn::ItemFn;

pub fn parse(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);

    let vis = &item.vis;
    let name = &item.sig.ident;

    (quote::quote! {
        #vis unsafe extern "C" fn #name(state : gmrs::lua::LuaStateRaw) -> i32 {
            #item
            match #name(state) {
                Ok(count) => count,
                Err(e) => {
                    unsafe { gmrs::lua::throw_error(state, format!("{}", e)) };
                }
            }
        }
    })
    .into()
}
