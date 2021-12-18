//! Implementation of [::rand::CryptoRng]
//!
//! This module contains rust adapters to the underlying RNG primitives,
//! both rand's and getrandom, each accessible with the respective `rand_core` and `getrandom`
//!
//! # Note
//! `getrandom` is currently unsupported completely as there's an issue in how it's compiled...
//!

#[cfg(feature = "getrandom")]
pub fn optee_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    unsafe { crate::wrapper::raw::TEE_GenerateRandom(buf.as_mut_ptr() as _, buf.len() as _) }

    Ok(())
}

#[cfg(feature = "rand_core")]
mod rand_core_wrapper {
    //we need CryptoRng
    // and RngCore
    use rand_core::{CryptoRng, Error, RngCore};

    /// Implements the necessary traits from [::rand]
    ///
    /// To use specify it as the Rng to use
    /// # Examples
    /// ```rust
    /// # use zondee_utee::wrapper::rand::*;
    ///
    /// use rand::Rng;
    ///
    /// let num: u32 =  TEERng.gen();
    /// ```
    #[derive(Default, Copy, Clone)]
    pub struct TEERng;

    static mut RNG: TEERng = TEERng {};

    impl TEERng {
        pub fn new_static() -> &'static mut Self {
            //this is safe because we are single threaded
            unsafe { &mut RNG }
        }
    }

    impl RngCore for TEERng {
        fn next_u32(&mut self) -> u32 {
            rand_core::impls::next_u32_via_fill(self)
        }

        fn next_u64(&mut self) -> u64 {
            rand_core::impls::next_u64_via_fill(self)
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.try_fill_bytes(dest).unwrap()
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
            unsafe {
                crate::wrapper::raw::TEE_GenerateRandom(dest.as_mut_ptr() as _, dest.len() as _)
            }

            //NOTE: the above call MAY panic, but it won't return an error code
            // so TODO: handle panics from OPTEE API (if possible)
            Ok(())
        }
    }

    impl CryptoRng for TEERng {}
}

#[cfg(feature = "rand_core")]
pub use rand_core_wrapper::*;
