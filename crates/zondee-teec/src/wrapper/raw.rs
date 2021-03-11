include!("teec.rs");

pub fn TEEC_PARAM_TYPES(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let tmp = p1 << 4 | p2 << 8 | p3 << 12;
    return p0 | tmp;
}
