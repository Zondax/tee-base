// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/master/optee-utee/macros/src/lib.rs

use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input, Expr, Ident, Lit, LitByteStr, Token,
};

struct OptionalUteeParams {
    ext_prop_value_1: Lit,
    ext_prop_value_2: Lit,
    ta_data_size: Lit,
    ta_description: Lit,
    ta_flags: Lit,
    ta_framework_stack_size: Lit,
    ta_stack_size: Lit,
    ta_version: Lit,
    trace_ext_prefix: Lit,
    trace_level: Lit,
}

impl Default for OptionalUteeParams {
    fn default() -> Self {
        Self {
            ext_prop_value_1: Lit::ByteStr(LitByteStr::new(b"EXT_PROP_1\0", Span::call_site())),
            ext_prop_value_2: Lit::Verbatim(Literal::u32_suffixed(0x0010)),
            ta_data_size: Lit::Verbatim(Literal::u32_suffixed(1 * 1024 * 1024)),
            ta_description: Lit::ByteStr(LitByteStr::new(b"Description\0", Span::call_site())),
            ta_flags: Lit::Verbatim(Literal::u32_suffixed(0)),
            ta_framework_stack_size: Lit::Verbatim(Literal::u32_suffixed(2048)),
            ta_stack_size: Lit::Verbatim(Literal::u32_suffixed(2 * 1024)),
            ta_version: Lit::ByteStr(LitByteStr::new(b"Version\0", Span::call_site())),
            trace_ext_prefix: Lit::ByteStr(LitByteStr::new(b"TA_EXT_PREFIX\0", Span::call_site())),
            trace_level: Lit::Verbatim(Literal::i32_suffixed(4)),
        }
    }
}

struct UteeParams {
    oop: OptionalUteeParams,
    uuid: Expr,
}

impl Parse for UteeParams {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let uuid: Expr = input.parse()?;
        let mut oop = OptionalUteeParams::default();
        loop {
            if input.parse::<Token![,]>().is_err() {
                break;
            }
            let ident = input.parse::<Ident>()?;
            let lit = match ident.to_string().as_str() {
                "DATA_SIZE" => &mut oop.ta_data_size,
                "DESCRIPTION" => &mut oop.ta_description,
                "FLAGS" => &mut oop.ta_flags,
                "FRAMEWORK_STACK_SIZE" => &mut oop.ta_framework_stack_size,
                "PROP_VALUE_1" => &mut oop.ext_prop_value_1,
                "PROP_VALUE_2" => &mut oop.ext_prop_value_2,
                "STACK_SIZE" => &mut oop.ta_stack_size,
                "TRACE_EXT_PREFIX" => &mut oop.trace_ext_prefix,
                "TRACE_LEVEL" => &mut oop.trace_level,
                "VERSION" => &mut oop.ta_version,
                _ => break,
            };
            input.parse::<Token![:]>()?;
            match input.parse::<Expr>()? {
                Expr::Lit(expr_lit) => {
                    *lit = expr_lit.lit;
                }
                _ => return Err(input.error("Must pass a literal to an argument")),
            }
        }
        Ok(Self { oop, uuid })
    }
}

pub fn utee_params(input: TokenStream) -> TokenStream {
    let UteeParams {
        oop:
            OptionalUteeParams {
                ext_prop_value_1,
                ext_prop_value_2,
                ta_data_size,
                ta_description,
                ta_flags,
                ta_framework_stack_size,
                ta_stack_size,
                ta_version,
                trace_ext_prefix,
                trace_level,
            },
        uuid,
    } = parse_macro_input!(input as UteeParams);

    quote!(
        const EXT_PROP_VALUE_1: &[u8] = #ext_prop_value_1;
        const EXT_PROP_VALUE_2: u32 = #ext_prop_value_2;
        const TA_DATA_SIZE: u32 = #ta_data_size;
        const TA_DESCRIPTION: &[u8] = #ta_description;
        const TA_FLAGS: u32 = #ta_flags;
        const TA_FRAMEWORK_STACK_SIZE: u32 = #ta_framework_stack_size;
        const TA_STACK_SIZE: u32 = #ta_stack_size;
        const TA_VERSION: &[u8] = #ta_version;
        const TRACE_EXT_PREFIX: &[u8] = #trace_ext_prefix;
        const TRACE_LEVEL: i32 = #trace_level;

        static FLAG_BOOL: bool = (TA_FLAGS & zondee_utee::wrapper::TA_FLAG_SINGLE_INSTANCE) != 0;
        static FLAG_MULTI: bool = (TA_FLAGS & zondee_utee::wrapper::TA_FLAG_MULTI_SESSION) != 0;
        static FLAG_INSTANCE: bool = (TA_FLAGS & zondee_utee::wrapper::TA_FLAG_INSTANCE_KEEP_ALIVE) != 0;

        extern "C" {
            fn __utee_entry(
                func: libc::c_ulong,
                session_id: libc::c_ulong,
                up: *mut zondee_utee::wrapper::utee_params,
                cmd_id: libc::c_ulong,
            );
        }

        #[no_mangle]
        pub unsafe extern "C" fn tahead_get_trace_level() -> libc::c_int {
            return trace_level;
        }

        #[no_mangle]
        #[link_section = ".ta_head"]
        pub static ta_head: zondee_utee::wrapper::ta_head = zondee_utee::wrapper::ta_head {
            uuid: #uuid,
            stack_size: TA_STACK_SIZE + TA_FRAMEWORK_STACK_SIZE,
            flags: TA_FLAGS,
            entry: __utee_entry
                as unsafe extern "C" fn(
                    libc::c_ulong,
                    libc::c_ulong,
                    *mut zondee_utee::wrapper::utee_params,
                    libc::c_ulong,
                ),
        };

        #[no_mangle]
        #[link_section = ".bss"]
        pub static ta_heap: [u8; TA_DATA_SIZE as usize] = [0; TA_DATA_SIZE as usize];

        #[no_mangle]
        pub static ta_heap_size: libc::size_t = core::mem::size_of::<u8>() * TA_DATA_SIZE as usize;

        #[no_mangle]
        pub static ta_num_props: libc::size_t = 9;

        #[no_mangle]
        pub static ta_props: [zondee_utee::wrapper::user_ta_property; 9] = [
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_SINGLE_INSTANCE.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_BOOL,
                value: &FLAG_BOOL as *const bool as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_MULTI_SESSION.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_BOOL,
                value: &FLAG_MULTI as *const bool as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_KEEP_ALIVE.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_BOOL,
                value: &FLAG_INSTANCE as *const bool as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_DATA_SIZE.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_U32,
                value: &TA_DATA_SIZE as *const u32 as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_STACK_SIZE.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_U32,
                value: &TA_STACK_SIZE as *const u32 as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_VERSION.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_STRING,
                value: TA_VERSION as *const [u8] as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: zondee_utee::wrapper::raw::TA_PROP_STR_DESCRIPTION.as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_STRING,
                value: TA_DESCRIPTION as *const [u8] as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: "gp.ta.description\0".as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_STRING,
                value: EXT_PROP_VALUE_1 as *const [u8] as *mut _,
            },
            zondee_utee::wrapper::user_ta_property {
                name: "gp.ta.version\0".as_ptr() as *const _,
                prop_type: zondee_utee::wrapper::raw::user_ta_prop_type_USER_TA_PROP_TYPE_U32,
                value: &EXT_PROP_VALUE_2 as *const u32 as *mut _,
            },
        ];

        #[no_mangle]
        pub static trace_ext_prefix: &[u8] = TRACE_EXT_PREFIX;

        #[no_mangle]
        pub static trace_level: libc::c_int = TRACE_LEVEL;
    )
    .into()
}
