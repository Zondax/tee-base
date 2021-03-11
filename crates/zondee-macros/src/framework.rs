// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub fn utee_setup(input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as syn::ItemFn);
    let fun_name = &fun.sig.ident;

    quote!(
        #[zondee_utee::wrapper::create]
        fn create() -> crate::wrapper::Result<()> {
            Ok(())
        }

        #[zondee_utee::wrapper::open_session]
        fn open_session() -> crate::wrapper::Result<()> {
            Ok(())
        }

        #[zondee_utee::wrapper::invoke_command]
        fn invoke_command(id: u32, params: &mut [zondee_utee::wrapper::Param; 4]) -> wrapper::Result<()> {
            let mut scratch = [0; 128];
            let mut input_mem = unsafe { params[0].as_memref().expect("Buffer doesn't exist") };
            let input = zondee::deserialize(input_mem.buffer(), &mut scratch);
            let output = #fun_name(input)?;
            let mut output_mem = unsafe { params[1].as_memref().expect("Buffer doesn't exist") };
            zondee::serialize(&output, output_mem.buffer());
            Ok(())
        }

        #fun
        #[zondee_utee::wrapper::close_session]
        fn close_session() {}

        #[zondee_utee::wrapper::destroy]
        fn destroy() {}
    )
    .into()
}
