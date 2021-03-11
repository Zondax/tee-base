// https://raw.githubusercontent.com/mesalock-linux/rust-optee-trustzone-sdk/master/optee-teec/src/uuid.rs

use crate::wrapper::raw::TEEC_UUID;
use core::fmt;
use zondee::to_hex;

pub struct Uuid {
    raw: TEEC_UUID,
}

impl Uuid {
    pub const fn from_fields(fields: (u32, u16, u16, [u8; 8])) -> Self {
        let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = fields;
        Self {
            raw: TEEC_UUID {
                timeLow: time_low,
                timeMid: time_mid,
                timeHiAndVersion: time_hi_and_version,
                clockSeqAndNode: clock_seq_and_node,
            },
        }
    }

    /// Converts a uuid to a const raw `TEEC_UUID` pointer.
    pub fn as_ptr(&self) -> *const TEEC_UUID {
        &self.raw
    }
}

impl From<zondee::Uuid> for Uuid {
    fn from(from: zondee::Uuid) -> Self {
        Self {
            raw: TEEC_UUID {
                timeLow: from.time_low,
                timeMid: from.time_mid,
                timeHiAndVersion: from.time_hi_and_version,
                clockSeqAndNode: from.clock_seq_and_node,
            },
        }
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:x}-{:x}-{:x}-{}",
            self.raw.timeLow,
            self.raw.timeMid,
            self.raw.timeHiAndVersion,
            to_hex(&self.raw.clockSeqAndNode).expect("Bad hex string")
        )
    }
}
