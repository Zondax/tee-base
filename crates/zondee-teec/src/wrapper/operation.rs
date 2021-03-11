use crate::wrapper::{raw::TEEC_Operation, Param, ParamTypes};
use core::marker::PhantomData;

/// This type defines the payload of either an open session operation or an
/// invoke command operation. It is also used for cancellation of operations,
/// which may be desirable even if no payload is passed.
pub struct Operation<A, B, C, D> {
    raw: TEEC_Operation,
    phantom: PhantomData<(A, B, C, D)>,
}

impl<A, B, C, D> Operation<A, B, C, D>
where
    A: Param,
    B: Param,
    C: Param,
    D: Param,
{
    pub fn new(p0: A, p1: B, p2: C, p3: D) -> Operation<A, B, C, D> {
        let mut raw = TEEC_Operation::default();
        raw.started = 0;
        raw.paramTypes = ParamTypes::new(
            p0.param_type(),
            p1.param_type(),
            p2.param_type(),
            p3.param_type(),
        )
        .into();
        raw.params = [p0.into_raw(), p1.into_raw(), p2.into_raw(), p3.into_raw()];

        Operation {
            raw,
            phantom: PhantomData,
        }
    }

    pub fn params(&self) -> (A, B, C, D) {
        let (f0, f1, f2, f3) = ParamTypes::from(self.raw.paramTypes).into_flags();
        (
            A::from_raw(self.raw.params[0], f0),
            B::from_raw(self.raw.params[1], f1),
            C::from_raw(self.raw.params[2], f2),
            D::from_raw(self.raw.params[3], f3),
        )
    }
}

impl<A, B, C, D> Operation<A, B, C, D> {
    pub fn as_mut_ptr(&mut self) -> *mut TEEC_Operation {
        &mut self.raw
    }
}
