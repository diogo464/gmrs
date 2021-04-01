use syn::{FnArg, ItemFn, Result};

fn parse_args_list(sig: &syn::Signature) -> Result<Vec<syn::PatType>> {
    let mut args = Vec::new();
    for arg in sig.inputs.iter() {
        match arg {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    &arg,
                    "Receiver arguments are now allowed here",
                ))
            }
            FnArg::Typed(pat) => args.push(pat.clone()),
        }
    }
    Ok(args)
}

pub fn parse(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as ItemFn);

    let vis = &item.vis;
    let name = &item.sig.ident;

    let args = match parse_args_list(&item.sig) {
        Ok(args) => args,
        Err(e) => return e.into_compile_error().into(),
    };
    let arg_ty = args.iter().map(|arg| &arg.ty);
    // let arg_num: Vec<_> = (0..arg_ty.len()).collect();
    let arg_name: Vec<_> = args.iter().map(|arg| &arg.pat).collect();

    (quote::quote! {
        #vis unsafe extern "C" fn #name(raw : gmrs::lua::LuaStateRaw) -> i32 {
            #item
            unsafe {
                let state = gmrs::lua::LuaState::new(raw);
                unsafe fn __inner_native_func_wrapper(state : gmrs::lua::LuaState) -> gmrs::lua::Result<i32> {
                    let mut stack_offset = 1;
                    #(
                        let (#arg_name, push_count) : (#arg_ty, i32) = <#arg_ty as gmrs::lua::FromStack>::from_stack(state, stack_offset)?;
                        stack_offset += push_count;
                    )*
                    let result = #name(#(#arg_name),*)?;
                    Ok(gmrs::lua::push(state, result))
                }
                let result = __inner_native_func_wrapper(state);
                match result {
                    Ok(count) => count,
                    Err(e) => {
                        gmrs::lua::throw_error(state, format!("{}", e));
                    }
                }
            }
        }
    })
    .into()
}
