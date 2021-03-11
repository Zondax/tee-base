#[derive(Debug)]
pub enum Error {
    ConnectionCode(u32),
    ConnectionCodeWithOrigin(u32, u32),
}
