mod entry;
mod exit;
mod function;
mod raw_function;

#[proc_macro_attribute]
pub fn entry(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    entry::parse(args, input)
}

#[proc_macro_attribute]
pub fn exit(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    exit::parse(args, input)
}

#[proc_macro_attribute]
pub fn function(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    function::parse(args, input)
}

#[proc_macro_attribute]
pub fn raw_function(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    raw_function::parse(args, input)
}
