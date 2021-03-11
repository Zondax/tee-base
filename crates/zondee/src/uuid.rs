#[derive(Copy, Clone)]
pub struct Uuid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq_and_node: [u8; 8],
}

impl Uuid {
    pub const fn as_fields(&self) -> (u32, u16, u16, [u8; 8]) {
        (
            self.time_low,
            self.time_mid,
            self.time_hi_and_version,
            self.clock_seq_and_node,
        )
    }
}
