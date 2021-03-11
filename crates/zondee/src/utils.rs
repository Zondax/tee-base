use arrayvec::ArrayString;
use core::fmt::Write;

#[cfg(all(feature = "with-serde", feature = "with-serde_cbor"))]
pub fn deserialize<'a: 'b, 'b, T>(bytes: &'a [u8], scratch: &mut [u8]) -> T
where
    T: serde::Deserialize<'b>,
{
    serde_cbor::de::from_slice_with_scratch(bytes, scratch).expect("Bad binary representation")
}

#[cfg(all(feature = "with-serde", feature = "with-serde_cbor"))]
pub fn serialize<T>(instance: &T, buffer: &mut [u8])
where
    T: serde::Serialize,
{
    let writer = serde_cbor::ser::SliceWrite::new(buffer);
    let mut ser = serde_cbor::Serializer::new(writer);
    instance
        .serialize(&mut ser)
        .expect("Bad instance representation");
}

pub fn to_hex(data: &[u8]) -> crate::Result<ArrayString<[u8; 512]>> {
    if data.len() * 2 >= 512 {
        return Err(crate::Error::InvalidHexInput);
    }
    let mut buf = ArrayString::<[_; 512]>::new();
    for &byte in data {
        write!(&mut buf, "{:02x}", byte).expect("Bad hex string");
    }
    Ok(buf)
}
