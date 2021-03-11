///Marker trait for types that can be used as object ids
pub trait ObjectID: Copy + Seal {
    fn byte_slice(&self) -> &[u8];
    fn byte_len(&self) -> usize;
}

pub trait Seal {}

macro_rules! impl_trait_for_array {

    ($([$t:ty; $($size:expr),+]),+) => {
        $($(impl Seal for [$t; $size] {})*)*

        $($(impl ObjectID for [$t; $size] {
            fn byte_slice(&self) -> &[u8] {
                let slice = &self[..];
                bytemuck::cast_slice(&slice)
            }

            fn byte_len(&self) -> usize {
                self.byte_slice().len()
            }
        })*)*
    };
}

impl_trait_for_array!(
            [u8; 1, 2, 3, 4, 5, 6, 7, 8,
              9, 10, 11, 12, 13, 14, 15, 16,
              17, 18, 19, 20, 21, 22, 23, 24,
              25, 26, 27, 28, 29, 30, 31, 32,
              33, 34, 35, 36, 37, 38, 39, 40,
              41, 42, 43, 44, 45, 46, 47, 48,
              49, 50, 51, 52, 53, 54, 55, 56,
              57, 58, 59, 60, 61, 62, 63, 64],
            [u16; 1, 2, 3, 4, 5, 6, 7, 8,
              9, 10, 11, 12, 13, 14, 15, 16,
              17, 18, 19, 20, 21, 22, 23, 24,
              25, 26, 27, 28, 29, 30, 31, 32],
            [u32; 1, 2, 3, 4, 5, 6, 7, 8,
              9, 10, 11, 12, 13, 14, 15, 16],
            [u64; 1, 2, 3, 4, 5, 6, 7, 8],
            [u128; 1, 2, 3, 4]);
