use proc_macro2::TokenStream;
use syn::{ItemFn, Result};

fn validate_exit(item: ItemFn) -> Result<TokenStream> {
    if item.sig.inputs.len() != 1 {
        return Err(syn::Error::new_spanned(
            &item,
            "Exit function must have 1 argument of type LuaState",
        ))?;
    }
    let name = &item.sig.ident;
    Ok(quote::quote! {
        #[no_mangle]
        pub extern "C" fn gmod13_close(raw: gmrs::lua::LuaStateRaw) -> u32 {
            #item
            let state = unsafe { gmrs::lua::LuaState::new(raw) };
            let _ = #name(state);
            0
        }
    })
}

pub fn parse(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);
    match validate_exit(item) {
        Ok(t) => t.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
