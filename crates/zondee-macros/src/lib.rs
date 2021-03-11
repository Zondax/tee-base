// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;

#[cfg(feature = "framework")]
mod framework;
mod wrapper;

#[cfg(feature = "framework")]
#[proc_macro_attribute]
pub fn framework_utee_setup(_: TokenStream, input: TokenStream) -> TokenStream {
    framework::utee_setup(input)
}

#[proc_macro_attribute]
pub fn wrapper_utee_close_session(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::utee_workflow::utee_close_session(input)
}

#[proc_macro_attribute]
pub fn wrapper_utee_create(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::utee_workflow::utee_create(input)
}

#[proc_macro_attribute]
pub fn wrapper_utee_destroy(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::utee_workflow::utee_destroy(input)
}

#[proc_macro_attribute]
pub fn wrapper_utee_invoke_command(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::utee_workflow::utee_invoke_command(input)
}

#[proc_macro_attribute]
pub fn wrapper_utee_open_session(_: TokenStream, input: TokenStream) -> TokenStream {
    wrapper::utee_workflow::utee_open_session(input)
}

#[proc_macro]
pub fn wrapper_utee_params(input: TokenStream) -> TokenStream {
    wrapper::utee_params::utee_params(input)
}
