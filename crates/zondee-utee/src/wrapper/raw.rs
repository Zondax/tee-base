include!(concat!("utee.rs"));

#[allow(non_snake_case)]
pub fn TEE_PARAM_TYPES(t0: u32, t1: u32, t2: u32, t3: u32) -> u32 {
    t0 | t1 << 4 | t2 << 8 | t3 << 12
}

pub type MemRef = TEE_Param__bindgen_ty_1;
pub type Value = TEE_Param__bindgen_ty_2;
