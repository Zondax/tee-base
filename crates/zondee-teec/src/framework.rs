use crate::wrapper::{Connection, Operation, ParamNone, ParamTmpRef, Uuid};
use heapless::consts::U256;

pub fn send_msg<T, U>(uuid: Uuid, input: T) -> crate::Result<U>
where
    T: serde::Serialize,
    U: for<'a> serde::Deserialize<'a>,
{
    let mut input_buffer = heapless::Vec::<u8, U256>::new();
    let mut output_buffer = heapless::Vec::<u8, U256>::new();
    let ctx = Default::default();
    let mut conn = Connection::new(ctx, "HOST", Default::default()).unwrap();
    conn.open_session(uuid.into(), &mut Default::default())?;
    zondee::serialize(&input, &mut input_buffer);
    let p0 = ParamTmpRef::new_input(&input_buffer);
    let p1 = ParamTmpRef::new_output(&mut output_buffer);
    let mut operation = Operation::new(p0, p1, ParamNone, ParamNone);
    conn.invoke_command(0, &mut operation)?;
    let mut scratch = [0; 256];
    Ok(zondee::deserialize(&output_buffer, &mut scratch))
}
