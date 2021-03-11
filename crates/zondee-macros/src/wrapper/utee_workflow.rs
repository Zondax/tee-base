// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn utee_create(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_CreateEntryPoint() -> zondee_utee::wrapper::raw::TEE_Result {
            let rslt: zondee_utee::wrapper::Result<()> = #fun_name();
            match rslt {
                Ok(_) => zondee_utee::wrapper::raw::TEE_SUCCESS,
                Err(e) => e as u32
            }
        }

        #fun
    )
    .into()
}

pub fn utee_open_session(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_OpenSessionEntryPoint(
            _param_types: u32,
            _params: &mut [zondee_utee::wrapper::raw::TEE_Param; 4],
            _sess_ctx: *mut *mut core::ffi::c_void,
        ) -> zondee_utee::wrapper::raw::TEE_Result {
            let rslt: zondee_utee::wrapper::Result<()> = #fun_name();
            match rslt {
                Ok(_) => zondee_utee::wrapper::raw::TEE_SUCCESS,
                Err(e) => e as u32
            }
        }

        #fun
    )
    .into()
}

pub fn utee_invoke_command(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_InvokeCommandEntryPoint(
            _sess_ctx: *mut core::ffi::c_void,
            id: u32,
            param_types: u32,
            raw_params: &mut [zondee_utee::wrapper::raw::TEE_Param; 4],
        ) -> zondee_utee::wrapper::raw::TEE_Result {
            let mut params = zondee_utee::wrapper::params(raw_params, param_types);
            let fun: fn(u32, &mut [zondee_utee::wrapper::Param; 4]) -> zondee_utee::wrapper::Result<()>;
            fun = #fun_name;
            match fun(id, &mut params) {
                Ok(_) => zondee_utee::wrapper::raw::TEE_SUCCESS,
                Err(e) => e as u32
            }
        }

        #fun
    )
    .into()
}

pub fn utee_close_session(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_CloseSessionEntryPoint(_sess_ctx: *mut core::ffi::c_void) {
            #fun_name();
        }

        #fun
    )
    .into()
}

pub fn utee_destroy(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;
    quote!(
        #[no_mangle]
        pub extern "C" fn TA_DestroyEntryPoint() {
            #fun_name();
        }

        #fun
    )
    .into()
}
