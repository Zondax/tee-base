#![no_std]

extern crate no_std_compat as std;

#[cfg(feature = "framework")]
pub mod framework;
pub mod wrapper;

///// Trait that must be implemented by types that can process commands from Ta
//pub trait HandleTaCommand {
//    fn handle_command(
//        &mut self,
//        cmd_id: u32,
//        input: Option<&[u8]>,
//        output: Option<&mut [u8]>,
//    ) -> Result<(), wrapper::TaErrorCode>;
//}
