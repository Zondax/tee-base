#[allow(
    clippy::all,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uuid {
    timeLow: u32,
    timeMid: u16,
    timeHiAndVersion: u16,
    clockSeqAndNode: [u8; 8],
}

impl Uuid {
    pub const fn from_fields(fields: (u32, u16, u16, [u8; 8])) -> Self {
        let (time_low, time_mid, time_hi_and_version, clock_seq_and_node) = fields;
        Self {
            timeLow: time_low,
            timeMid: time_mid,
            timeHiAndVersion: time_hi_and_version,
            clockSeqAndNode: clock_seq_and_node,
        }
    }
}

impl From<zondee::Uuid> for Uuid {
    fn from(from: zondee::Uuid) -> Self {
        Self {
            timeLow: from.time_low,
            timeMid: from.time_mid,
            timeHiAndVersion: from.time_hi_and_version,
            clockSeqAndNode: from.clock_seq_and_node,
        }
    }
}
