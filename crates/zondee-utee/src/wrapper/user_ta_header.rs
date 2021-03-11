// https://github.com/mesalock-linux/rust-optee-trustzone-sdk/blob/e48eef965c0a14b50c481fa4d829c155ce6c711c/optee-utee/optee-utee-sys/src/user_ta_header.rs

use crate::wrapper::raw::TEE_NUM_PARAMS;

pub const TA_FLAG_CACHE_MAINTENANCE: u32 = 1 << 7;
pub const TA_FLAG_EXEC_DDR: u32 = 0;
pub const TA_FLAG_INSTANCE_KEEP_ALIVE: u32 = 1 << 4;
pub const TA_FLAG_MULTI_SESSION: u32 = 1 << 3;
pub const TA_FLAG_REMAP_SUPPORT: u32 = 1 << 6;
pub const TA_FLAG_SECURE_DATA_PATH: u32 = 1 << 5;
pub const TA_FLAG_SINGLE_INSTANCE: u32 = 1 << 2;
pub const TA_FLAG_USER_MODE: u32 = 0;

//#[repr(C)]
//pub struct ta_head {
//    pub uuid: wrapper::Uuid,
//    pub stack_size: u32,
//    pub flags: u32,
//    pub entry: unsafe extern "C" fn(libc::c_ulong, libc::c_ulong, *mut utee_params, libc::c_ulong),
//}
//
//unsafe impl Sync for ta_head {}

//#[repr(C)]
//pub struct user_ta_property {
//    pub name: *const libc::c_char,
//    pub prop_type: u32,
//    pub value: *mut core::ffi::c_void,
//}
//
//unsafe impl Sync for user_ta_property {}

#[repr(C)]
pub struct utee_params {
    types: u64,
    vals: [u64; TEE_NUM_PARAMS as usize * 2],
}
